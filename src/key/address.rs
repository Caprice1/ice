
use crate::sendmany::SendManyRecipient;
use crate::sendmany::CAmount;

pub struct AddressManagement {

}

impl AddressManagement {

    pub fn new() -> Self {
        return AddressManagement{};
    }

    pub fn decode_transparent_destination(&self, address: &String) -> bool {
        false
    }

    pub fn decode_z_destination(&self, aaddress: &String) -> bool {
        false
    }

    pub fn decode_outputs(&self, aoutputs_str: &String)
                      -> (Vec<SendManyRecipient>, Vec<SendManyRecipient>, CAmount) {
        (Vec::new(), Vec::new(), 0)
    }
}