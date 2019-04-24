use bellman::groth16::{
    prepare_verifying_key, PreparedVerifyingKey, VerifyingKey as BellmanVerifyingKey,
};
use group::EncodedPoint;
use pairing::bls12_381::{Bls12, G1Uncompressed, G2Uncompressed};
use rustc_hex::FromHex;
use serde::de::{self, Deserialize, Deserializer, Visitor};
use serde_derive::Deserialize;
use std::fmt;

pub type SaplingPreparedVerifyingKey = PreparedVerifyingKey<Bls12>;

lazy_static! {
    pub static ref SPEND_VK: SaplingPreparedVerifyingKey =
        { load_sapling_spend_verifying_key().unwrap() };
    pub static ref OUTPUT_VK: SaplingPreparedVerifyingKey =
        { load_sapling_output_verifying_key().unwrap() };
}

fn clean_0x(s: &str) -> &str {
    if s.starts_with("0x") {
        &s[2..]
    } else {
        s
    }
}

#[derive(Debug, Clone)]
struct Point<EP: EncodedPoint>(EP::Affine);

impl<'de, EP: EncodedPoint> Deserialize<'de> for Point<EP> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EncodedPointVisitor<EP: EncodedPoint>(::std::marker::PhantomData<EP>);

        impl<'de, EP: EncodedPoint> Visitor<'de> for EncodedPointVisitor<EP> {
            type Value = Point<EP>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a hex string")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let mut point = EP::empty();
                let point_raw = clean_0x(value)
                    .from_hex::<Vec<_>>()
                    .map_err(|e| de::Error::custom(format!("Expected hex string: {}", e)))?;
                if point.as_ref().len() != point_raw.len() {
                    return Err(de::Error::custom(format!(
                        "Expected hex string of length {}",
                        point.as_ref().len()
                    )));
                }

                point.as_mut().copy_from_slice(&point_raw);
                point
                    .into_affine()
                    .map_err(|e| de::Error::custom(format!("Invalid curve point: {}", e)))
                    .map(Point)
            }
        }

        deserializer.deserialize_str(EncodedPointVisitor::<EP>(Default::default()))
    }
}

type G1 = Point<G1Uncompressed>;
type G2 = Point<G2Uncompressed>;
#[derive(Clone, Deserialize)]
struct VerifyingKey {
    #[serde(rename = "alphaG1")]
    pub alpha_g1: G1,
    #[serde(rename = "betaG1")]
    pub beta_g1: G1,
    #[serde(rename = "betaG2")]
    pub beta_g2: G2,
    #[serde(rename = "gammaG2")]
    pub gamma_g2: G2,
    #[serde(rename = "deltaG1")]
    pub delta_g1: G1,
    #[serde(rename = "deltaG2")]
    pub delta_g2: G2,
    #[serde(rename = "ic")]
    pub ic: Vec<G1>,
}

impl From<VerifyingKey> for BellmanVerifyingKey<Bls12> {
    fn from(vk: VerifyingKey) -> BellmanVerifyingKey<Bls12> {
        BellmanVerifyingKey {
            alpha_g1: vk.alpha_g1.0,
            beta_g1: vk.beta_g1.0,
            beta_g2: vk.beta_g2.0,
            gamma_g2: vk.gamma_g2.0,
            delta_g1: vk.delta_g1.0,
            delta_g2: vk.delta_g2.0,
            ic: vk.ic.into_iter().map(|p| p.0).collect(),
        }
    }
}

pub fn load_sapling_spend_verifying_key() -> Result<SaplingPreparedVerifyingKey, String> {
    let spend_vk_json = include_bytes!("../res/sapling-spend-verifying-key.json");
    let spend_vk = serde_json::from_slice::<VerifyingKey>(&spend_vk_json[..]).unwrap();
    Ok(prepare_verifying_key(&spend_vk.into()))
}

pub fn load_sapling_output_verifying_key() -> Result<SaplingPreparedVerifyingKey, String> {
    let output_vk_json = include_bytes!("../res/sapling-output-verifying-key.json");
    let output_vk = serde_json::from_slice::<VerifyingKey>(&output_vk_json[..]).unwrap();
    Ok(prepare_verifying_key(&output_vk.into()))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn load_vk_key() {
        assert_eq!(load_sapling_spend_verifying_key().is_ok(), true);
        assert_eq!(load_sapling_output_verifying_key().is_ok(), true);
    }
}
