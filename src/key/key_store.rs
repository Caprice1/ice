

use crate::key::key_management::*;
use std::collections::hash_map::HashMap;
use crate::sendmany::SendManyRecipient;
use crate::sendmany::CAmount;
use crate::key::key_management::{
    SaplingPaymentAddress, SaplingExpandedSpendingKey, SaplingExtendedSpendingKey
};

pub struct KeyStore {

    //SaplingSpendingKeyMap mapSaplingSpendingKeys;
    //    SaplingFullViewingKeyMap mapSaplingFullViewingKeys;
    //    SaplingIncomingViewingKeyMap mapSaplingIncomingViewingKeys;
    mapSaplingIncomingViewKeys: HashMap<SaplingPaymentAddress, SaplingIncomingViewingKey>,
}

impl KeyStore {

    pub fn new() -> Self {
        KeyStore {
            mapSaplingIncomingViewKeys: HashMap::new(),
        }
    }

    pub fn decode_transparent_destination(&self, address: &String) -> bool {
        false
    }

    fn decode_payment_address(address: &String) -> Option<SaplingPaymentAddress> {
        None
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

    pub fn decode_z_destination(&self, address: &String)
                                -> (Option<SaplingPaymentAddress>, Option<SaplingExtendedSpendingKey>)
    {
        (None, None)
    }

    pub fn decode_outputs(&self, aoutputs_str: &String)
                          -> (Vec<SendManyRecipient>, Vec<SendManyRecipient>, CAmount) {
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
    pub fn get_sapling_extended_spending_key(&self, address: SaplingPaymentAddress)
                                            -> Option<SaplingExtendedSpendingKey> {
        //self.get_sapling_incomming_viewing_key(address).and_then
        None
    }

    pub fn get_sapling_incomming_viewing_key(&self, address: SaplingPaymentAddress)
            -> Option<SaplingIncomingViewingKey>{
        //self.mapSaplingIncomingViewKeys.get(address)
        None
    }

    pub fn get_sapling_full_view_key(&self, ivk: &SaplingIncomingViewingKey)
            -> Option<SaplingFullViewKey>{
        None
    }

    pub fn get_sapling_spending_key(&self, fvk: &SaplingFullViewKey)
            -> Option<SaplingExtendedSpendingKey> {
        None
    }

}