use crate::key::key_management::*;
use crate::key::key_management::{
    SaplingExpandedSpendingKey, SaplingExtendedSpendingKey, SaplingPaymentAddress,
    PAYMENT_ADDRESS_LENGTH,
};
use crate::sendmany::CAmount;
use crate::sendmany::SendManyRecipient;
use bech32::{u5, Bech32};
use ethereum_types::U256;
use ethereum_types::H160;
use pairing::bls12_381::Bls12;
use sapling_crypto::jubjub::{edwards, Unknown};
use serde_json;
use serde_json::{Result, Value};
use std::collections::hash_map::HashMap;
use std::str::FromStr;
use zcash_primitives::JUBJUB;

use std::collections::HashSet;

pub fn from_to_u256(value: &[u8; 32]) -> U256 {
   U256::from_little_endian(value)
}



// Struct used to covert between u5 vector and u8 vector.
struct BitVec {
    // TODO: Change to bit implementation.
    pub vec_: Vec<u8>,
}
impl BitVec {
    pub fn new() -> BitVec {
        BitVec { vec_: Vec::new() }
    }
    pub fn from_u5_vec(u5_v: &[u5]) -> BitVec {
        let mut vec = BitVec { vec_: Vec::new() };
        for x in u5_v {
            let mut x_u8 = x.to_u8();
            let mut b5 = [0u8; 5];
            for i in 0..5 {
                b5[4 - i] = x_u8 % 2;
                x_u8 = x_u8 / 2;
            }
            for x in b5.iter() {
                vec.vec_.push(*x);
            }
        }
        return vec;
    }

    pub fn attach_u8_vec(&mut self, u8_v: &[u8]) {
        for x in u8_v {
            let mut b8 = [0u8; 8];
            let mut tmp = x.clone();
            for i in 0..8 {
                b8[7 - i] = tmp % 2;
                tmp = tmp / 2;
            }
            for b in b8.iter() {
                self.vec_.push(*b);
            }
        }
    }

    pub fn to_u5_vec(&mut self) -> Vec<u8> {
        // Add additional 0 to bit vector such that the total lenght is multiplier of 5.
        let mut u5_vec = Vec::new();;
        if self.vec_.len() % 5 != 0 {
            for i in 0..(5 - (self.vec_.len() % 5)) {
                self.vec_.push(0u8);
            }
        }

        for i in 0..self.vec_.len() / 5 {
            let mut num_u8 = 0;
            for j in 0..5 {
                num_u8 += 2_u8.pow(j as u32) * self.vec_[i * 5 + 4 - j];
            }
            u5_vec.push(num_u8);
        }
        return u5_vec;
    }

    pub fn to_u8(&self, start: usize, length: usize) -> Vec<u8> {
        assert!(self.vec_.len() > start + length * 8);
        let mut u8_vec = Vec::with_capacity(length);
        for i in 0..length {
            let mut num_u8 = 0;
            for j in 0..8 {
                num_u8 += 2_u8.pow(j as u32) * self.vec_[i * 8 + 7 - j];
            }
            u8_vec.push(num_u8);
        }
        return u8_vec;
    }
}

pub fn decode_payment_address(address: &str) -> Option<SaplingPaymentAddress> {
    let b32_parsed = address.parse::<Bech32>().unwrap();
    let u5_vec = b32_parsed.data();
    if u5_vec.len() != (PAYMENT_ADDRESS_LENGTH * 8 - 1 + 5) / 5 {
        return None;
    }
    let bit_vec = BitVec::from_u5_vec(u5_vec);
    let u8_vec = bit_vec.to_u8(0, PAYMENT_ADDRESS_LENGTH);

    let mut diversifier = [0u8; 11];
    diversifier.copy_from_slice(&u8_vec[0..11]);
    let pk_d = &u8_vec[11..PAYMENT_ADDRESS_LENGTH];
    let pk_d = match edwards::Point::<Bls12, Unknown>::read(&mut pk_d.as_ref(), &JUBJUB) {
        Ok(p) => p,
        Err(_) => return None,
    };
    let pk_d = match pk_d.as_prime_order(&JUBJUB) {
        Some(pk_d) => pk_d,
        None => return None,
    };
    let payment_address = SaplingPaymentAddress {
        diversifier: sapling_crypto::primitives::Diversifier(diversifier),
        pk_d: pk_d,
    };
    Some(payment_address)
}

pub fn encode_payment_address(address: &SaplingPaymentAddress) -> String {
    let mut pk_d_vec = [0u8; 32];
    address.pk_d.write(&mut pk_d_vec[0..32]);
    let mut bit_vec = BitVec::new();
    bit_vec.attach_u8_vec(&address.diversifier.0);
    bit_vec.attach_u8_vec(&pk_d_vec);
    let b = Bech32::new_check_data("zs".into(), bit_vec.to_u5_vec());
    assert!(b.is_ok());
    let encoded = b.unwrap().to_string();
    return encoded;
}

pub type TxDestination = H160;

pub fn decode_destination(address: &str) -> Option<TxDestination> {
    match TxDestination::from_str(address.trim_start_matches("0x")) {
        Ok(add) => Some(add),
        Err(_) => None,
    }
}

pub struct KeyStore {
    mapIncomingViewKeys: HashMap<SaplingPaymentAddress, SaplingIncomingViewingKey>,
    mapFullViewingKeys: HashMap<SaplingIncomingViewingKey, SaplingFullViewingKey>,
    mapSpendingKeys: HashMap<SaplingFullViewingKey, SaplingExtendedSpendingKey>,
}

impl KeyStore {
    pub fn new() -> Self {
        KeyStore {
            mapIncomingViewKeys: HashMap::new(),
            mapFullViewingKeys: HashMap::new(),
            mapSpendingKeys: HashMap::new(),
        }
    }

    pub fn decode_transparent_destination(&self, address: &str) -> bool {
        match decode_destination(address) {
            Some(_) => true,
            None => false,
        }
    }

    pub fn decode_z_destination(
        &self,
        address: &str,
    ) -> (
        Option<SaplingPaymentAddress>,
        Option<SaplingExtendedSpendingKey>,
    ) {
        let payment_address = decode_payment_address(address);
        match decode_payment_address(address) {
            Some(a) => (Some(a), self.get_extended_spending_key(&a)),
            None => (None, None),
        }
    }

    pub fn decode_outputs(
        &self,
        aoutputs_str: &str,
    ) -> (Vec<SendManyRecipient>, Vec<SendManyRecipient>, CAmount) {
        let v: Vec<Value> = serde_json::from_str(aoutputs_str).unwrap();
        let mut total: CAmount = 0;
        let mut t_recipients = Vec::new();
        let mut z_recipients = Vec::new();
        for r in v {
            let recipient = (
                r["address"].as_str().unwrap().to_string(),
                r["amount"].as_u64().unwrap(),
                "".to_string(),
            );
            total += recipient.1;
            if self.decode_transparent_destination(&recipient.0) {
                t_recipients.push(recipient);
            } else {
                z_recipients.push(recipient);
            }
        }
        (z_recipients, t_recipients, total)
    }

    pub fn get_extended_spending_key(
        &self,
        address: &SaplingPaymentAddress,
    ) -> Option<SaplingExtendedSpendingKey> {
        self.get_incoming_viewing_key(address)
            .and_then(|ivk| self.get_full_viewing_key(&ivk))
            .and_then(|fvk| self.get_spending_key(&fvk))
    }

    pub fn get_incoming_viewing_key(
        &self,
        address: &SaplingPaymentAddress,
    ) -> Option<SaplingIncomingViewingKey> {
        match self.mapIncomingViewKeys.get(address) {
            Some(&v) => Some(v),
            None => None,
        }
    }

    pub fn get_full_viewing_key(
        &self,
        ivk: &SaplingIncomingViewingKey,
    ) -> Option<SaplingFullViewingKey> {
        match self.mapFullViewingKeys.get(ivk) {
            Some(&v) => Some(v),
            None => None,
        }
    }

    pub fn get_spending_key(
        &self,
        fvk: &SaplingFullViewingKey,
    ) -> Option<SaplingExtendedSpendingKey> {
        match self.mapSpendingKeys.get(fvk) {
            Some(&v) => Some(v),
            None => None,
        }
    }

    pub fn add_spending_key(
        &mut self,
        esk: SaplingExtendedSpendingKey,
        address: SaplingPaymentAddress,
    ) -> bool {
        let fvk = SaplingFullViewingKey::from_expanded_spending_key(&esk.expsk, &JUBJUB);
        self.mapSpendingKeys.insert(fvk, esk);
        return self.add_full_viewing_key(fvk, address);
    }

    pub fn add_full_viewing_key(
        &mut self,
        fvk: SaplingFullViewingKey,
        address: SaplingPaymentAddress,
    ) -> bool {
        let ivk = fvk.vk.ivk();
        self.mapFullViewingKeys.insert(ivk, fvk);
        return self.add_incoming_viewing_key(ivk, address);
    }
    pub fn add_incoming_viewing_key(
        &mut self,
        ivk: SaplingIncomingViewingKey,
        address: SaplingPaymentAddress,
    ) -> bool {
        self.mapIncomingViewKeys.insert(address, ivk);
        true
    }

    pub fn get_sapling_payment_addresses(&self) -> HashSet<SaplingPaymentAddress> {
        let mut set = HashSet::new();
        for (k, _) in &self.mapIncomingViewKeys {
            set.insert(k.clone());
        }
        set
    }

    pub fn have_sapling_spending_key(&self, fvk: &SaplingFullViewingKey) -> bool {
        return self.mapSpendingKeys.contains_key(fvk);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_decode_payment_address() {
        let address =
            "zs14j53eenhdjp85dlfctsttmtgav8sqkkttsl6qxvpmn74jk7edsyzp08r550dzu96hu9gwj2nl86";
        let option = decode_payment_address(address);
        match option {
            Some(p) => assert_eq!(
                address,
                encode_payment_address(&p),
                "encoded address doesn't equal."
            ),
            None => panic!("Can't decode address"),
        }
    }

    #[test]
    fn test_decode_outputs() {
        let output = r#"[{"address": "ztfaW34Gj9FrnGUEf833ywDVL62NWXBM81u6EQnM6VR45eYnXhwztecW1SjxA7JrmAXKJhxhj3vDNEpVCQoSvVoSpmbhtjf" ,"amount": 5}, 
                         {"address": "0x793ea9692Ada1900fBd0B80FFFEc6E431fe8b391" ,"amount": 6}]"#;
        let k = KeyStore::new();
        let (zaddr_recipients, taddr_recipients, total_amount) = k.decode_outputs(output);
        assert_eq!(total_amount, 11);
        assert_eq!(zaddr_recipients.len(), 1);
        assert_eq!(zaddr_recipients[0], ("ztfaW34Gj9FrnGUEf833ywDVL62NWXBM81u6EQnM6VR45eYnXhwztecW1SjxA7JrmAXKJhxhj3vDNEpVCQoSvVoSpmbhtjf".to_string(),
                                        5, "".to_string()));
        assert_eq!(zaddr_recipients.len(), 1);
        assert_eq!(taddr_recipients.len(), 1);
        assert_eq!(
            taddr_recipients[0],
            (
                "0x793ea9692Ada1900fBd0B80FFFEc6E431fe8b391".to_string(),
                6,
                "".to_string()
            )
        );
    }
}
