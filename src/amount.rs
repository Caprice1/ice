use crate::sendmany::CAmount;

pub struct FeeRate {
    n_satoshis_per_k: CAmount,
}

impl FeeRate {
    pub fn new() -> Self {
        FeeRate {
            n_satoshis_per_k: 0,
        }
    }
}
