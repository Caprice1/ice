use crate::key::key_management::*;
use crate::key::key_management::{
    SaplingExpandedSpendingKey, SaplingExtendedSpendingKey, SaplingPaymentAddress,
    PAYMENT_ADDRESS_LENGTH,
};
use crate::sendmany::CAmount;
use crate::sendmany::SendManyRecipient;
use bech32::{u5, Bech32};
use pairing::bls12_381::Bls12;
use sapling_crypto::jubjub::{edwards, Unknown};
use std::collections::hash_map::HashMap;
use zcash_primitives::JUBJUB;


use std::collections::HashSet;

pub struct KeyStore {
    //SaplingSpendingKeyMap mapSaplingSpendingKeys;
//    SaplingFullViewingKeyMap mapSaplingFullViewingKeys;
//    SaplingIncomingViewingKeyMap mapSaplingIncomingViewingKeys;

//mapSaplingIncomingViewKeys: HashMap<SaplingPaymentAddress, SaplingIncomingViewingKey>,
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
    assert_eq!(
        u5_vec.len(),
        (PAYMENT_ADDRESS_LENGTH * 8 - 1 + 5) / 5,
        "Incorrect address length."
    );
    let mut bit_vec = BitVec::from_u5_vec(u5_vec);
    let u8_vec = bit_vec.to_u8(0, PAYMENT_ADDRESS_LENGTH);

    let mut diversifier = [0u8; 11];
    diversifier.copy_from_slice(&u8_vec[0..11]);
    let mut pk_d = &u8_vec[11..PAYMENT_ADDRESS_LENGTH];
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

pub struct TxDestination {}

pub fn decode_destination(address: &String) -> Option<TxDestination> {
    None
}

impl KeyStore {
    pub fn new() -> Self {
        KeyStore {
            //mapSaplingIncomingViewKeys: HashMap::new(),
        }
    }

    pub fn decode_transparent_destination(&self, address: &String) -> bool {
        false
    }

    //if (!isfromtaddr_) {
    //        auto address = DecodePaymentAddress(fromAddress);
    //        if (IsValidPaymentAddress(address)) {
    //            // We don't need to lock on the wallet as spending key related methods are thread-safe
    //            if (!boost::apply_visitor(HaveSpendingKeyForPaymentAddress(pwalletMain), address)) {
    //                throw JSONRPCError(RPC_INVALID_ADDRESS_OR_KEY, "Invalid from address, no spending key found for zaddr");
    //            }
    //
    //            isfromzaddr_ = true;
    //            frompaymentaddress_ = address;
    //            spendingkey_ = boost::apply_visitor(GetSpendingKeyForPaymentAddress(pwalletMain), address).get();
    //        } else {
    //            throw JSONRPCError(RPC_INVALID_ADDRESS_OR_KEY, "Invalid from address");
    //        }
    //    }

    pub fn decode_z_destination(
        &self,
        address: &String,
    ) -> (
        Option<SaplingPaymentAddress>,
        Option<SaplingExtendedSpendingKey>,
    ) {
        (None, None)
    }

    pub fn decode_outputs(
        &self,
        aoutputs_str: &String,
    ) -> (Vec<SendManyRecipient>, Vec<SendManyRecipient>, CAmount) {
        (Vec::new(), Vec::new(), 0)
    }

    //bool CBasicKeyStore::GetSaplingExtendedSpendingKey(const libzcash::SaplingPaymentAddress &addr,
    //                                    libzcash::SaplingExtendedSpendingKey &extskOut) const {
    //    libzcash::SaplingIncomingViewingKey ivk;
    //    libzcash::SaplingFullViewingKey fvk;
    //
    //    return GetSaplingIncomingViewingKey(addr, ivk) &&
    //            GetSaplingFullViewingKey(ivk, fvk) &&
    //            GetSaplingSpendingKey(fvk, extskOut);
    //}
    pub fn get_sapling_extended_spending_key(
        &self,
        address: SaplingPaymentAddress,
    ) -> Option<SaplingExtendedSpendingKey> {
        //self.get_sapling_incomming_viewing_key(address).and_then
        None
    }

    pub fn get_sapling_incomming_viewing_key(
        &self,
        address: SaplingPaymentAddress,
    ) -> Option<SaplingIncomingViewingKey> {
        //self.mapSaplingIncomingViewKeys.get(address)
        None
    }

    pub fn get_sapling_full_view_key(
        &self,
        ivk: &SaplingIncomingViewingKey,
    ) -> Option<SaplingFullViewKey> {
        None
    }

    pub fn get_sapling_spending_key(
        &self,
        fvk: &SaplingFullViewKey,
    ) -> Option<SaplingExtendedSpendingKey> {
        None
    }


    //GetSaplingPaymentAddresses
    //TODO wu xin
    pub fn get_sapling_payment_addresses() -> HashSet<SaplingPaymentAddress> {
        //HashSet::new()
        unimplemented!()
    }

    //bool HaveSaplingSpendingKey(const libzcash::SaplingFullViewingKey &fvk) const
    //support wallet::GetFilteredNotes
    //TODO wu xin
    pub fn have_sapling_spending_key() -> bool {
        false
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
}
