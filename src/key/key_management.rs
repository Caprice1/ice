use std::hash::{Hash, Hasher};

use zip32::ExpandedSpendingKey;

use sapling_crypto::primitives::{Note, PaymentAddress, IncomeViewingKey};

use pairing::bls12_381::{Bls12, Fr, FrRepr};

use ff::PrimeField;

use zip32::{ExtendedSpendingKey, FullViewingKey, OutgoingViewingKey};

use zcash_primitives::transaction::components::{OutputDescription, SpendDescription};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct FrHash(pub Fr);

impl Hash for FrHash {
    fn hash<H: Hasher>(&self, state: &mut H) {
        //Fr::into_repr(&self.0)
        let arr = self.0.into_repr();
        //let tmp = arr.0;
        state.write_u64(arr.0[0]);
        state.write_u64(arr.0[1]);
        state.write_u64(arr.0[2]);
        state.write_u64(arr.0[3]);
        state.finish();
    }
}

pub type SaplingIncomingViewingKey = IncomeViewingKey<Bls12>;

pub type SaplingExtendedSpendingKey = ExtendedSpendingKey;

pub type SaplingExpandedSpendingKey = ExpandedSpendingKey<Bls12>;

pub type SaplingOutgoingViewingKey = OutgoingViewingKey;

pub type SaplingPaymentAddress = PaymentAddress<Bls12>;

pub type SaplingFullViewKey = FullViewingKey<Bls12>;

pub type SaplingNote = Note<Bls12>;

pub type SaplingSpendDescription = SpendDescription;

pub type SaplingOutputDescription = OutputDescription;

// 11(d) + 32(pk_d)
pub const PAYMENT_ADDRESS_LENGTH: usize = 43;
