
use zip32::ExpandedSpendingKey;

use sapling_crypto::{
    primitives::{Note, PaymentAddress},
};

use pairing::{
    bls12_381::{Bls12, Fr, FrRepr},
};

use zip32::{
    FullViewingKey, ExtendedSpendingKey, OutgoingViewingKey,
};

use zcash_primitives::transaction::components::{
    SpendDescription, OutputDescription
};


pub struct SaplingIncomingViewingKey {

}

pub type SaplingExtendedSpendingKey = ExtendedSpendingKey;

pub type SaplingExpandedSpendingKey = ExpandedSpendingKey<Bls12>;

pub type SaplingOutgoingViewingKey = OutgoingViewingKey;

pub type SaplingPaymentAddress = PaymentAddress<Bls12>;

pub type SaplingFullViewKey = FullViewingKey<Bls12>;

pub type SaplingNote = Note<Bls12>;

pub type SaplingSpendDescription = SpendDescription;

pub type SaplingOutputDescription = OutputDescription;