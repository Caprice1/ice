
use zip32::ExpandedSpendingKey;

use sapling_crypto::{
    primitives::{Note, PaymentAddress},
};

use pairing::{
    bls12_381::{Bls12, Fr, FrRepr},
};

pub struct SaplingIncomingViewingKey {

}

pub type SaplingExpandedSpendingKey = ExpandedSpendingKey<Bls12>;

pub type SaplingNote = Note<Bls12>;