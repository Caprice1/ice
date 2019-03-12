//
//use crate::sendmany::SendManyRecipient;
//use crate::sendmany::CAmount;
//use crate::key::key_management::{
//    SaplingPaymentAddress, SaplingExpandedSpendingKey, SaplingExtendedSpendingKey
//};
//
//pub struct AddressManagement {
//
//}
//
//impl AddressManagement {
//
//    pub fn new() -> Self {
//        return AddressManagement{};
//    }
//
//    pub fn decode_transparent_destination(&self, address: &String) -> bool {
//        false
//    }
//
//    fn decode_payment_address(address: &String) -> Option<SaplingPaymentAddress> {
//        None
//    }
//
//    //if (!isfromtaddr_) {
//    //        auto address = DecodePaymentAddress(fromAddress);
//    //        if (IsValidPaymentAddress(address)) {
//    //            // We don't need to lock on the wallet as spending key related methods are thread-safe
//    //            if (!boost::apply_visitor(HaveSpendingKeyForPaymentAddress(pwalletMain), address)) {
//    //                throw JSONRPCError(RPC_INVALID_ADDRESS_OR_KEY, "Invalid from address, no spending key found for zaddr");
//    //            }
//    //
//    //            isfromzaddr_ = true;
//    //            frompaymentaddress_ = address;
//    //            spendingkey_ = boost::apply_visitor(GetSpendingKeyForPaymentAddress(pwalletMain), address).get();
//    //        } else {
//    //            throw JSONRPCError(RPC_INVALID_ADDRESS_OR_KEY, "Invalid from address");
//    //        }
//    //    }
//
//    pub fn decode_z_destination(&self, address: &String)
//            -> (Option<SaplingPaymentAddress>, Option<SaplingExtendedSpendingKey>)
//    {
//
//    }
//
//    pub fn decode_outputs(&self, aoutputs_str: &String)
//                      -> (Vec<SendManyRecipient>, Vec<SendManyRecipient>, CAmount) {
//        (Vec::new(), Vec::new(), 0)
//    }
//}