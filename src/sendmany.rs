


extern crate zip32;
extern crate pairing;

use zip32::ExpandedSpendingKey;

use pairing::{
    bls12_381::{Bls12, Fr, FrRepr},
};

use sapling_crypto::{
    primitives::{Note, PaymentAddress},
};

use crate::my::constants;

//SaplingNote

pub struct SpendDescriptionInfo {
    pub expsk: ExpandedSpendingKey<Bls12>,
    note: Note<Bls12>,
    pub alpha: Fr,
    pub ahchor: Fr,

}

/*
struct SaplingNoteEntry
{
    SaplingOutPoint op;
    libzcash::SaplingPaymentAddress address;
    libzcash::SaplingNote note;
    std::array<unsigned char, ZC_MEMO_SIZE> memo;
    int confirmations;
};
*/
pub struct SaplingOutPoint
{
    //uint256
    pub hash: Fr,
    pub n: u32,
}


pub struct SaplingNoteEntry {
    op: SaplingOutPoint,
    //PaymentAddress not sure is Bls12
    address: PaymentAddress<Bls12>,
    note: Note<Bls12>,
    memo: [char; constants::ZC_MEMO_SIZE],
    confirmation: i32,
}

type CAmount = u64;

pub struct SendManyRecipient(String, CAmount, String);

pub fn show() {
    println!("sendmany show");

    let v = [1, 2, 3, 4, 5];


}

pub fn sendmany(_z_inputs_: Vec<SaplingNoteEntry>, _z_outputs_: Vec<SendManyRecipient>) {
    /*
    if (isfromzaddr_) {
            auto sk = boost::get<libzcash::SaplingExtendedSpendingKey>(spendingkey_);
            expsk = sk.expsk;
            ovk = expsk.full_viewing_key().ovk;
    }
    */

    let mut ops = Vec::new();
    let mut notes = Vec::new();
    /*
    CAmount sum = 0;
    for (auto t : z_sapling_inputs_) {
    ops.push_back(t.op);
    notes.push_back(t.note);
    sum += t.note.value();
    if (sum >= targetAmount) {
    break;
    }
    }*/
    let mut targetAmount = 100;
    let (ops, sum) = _z_inputs_.iter().try_fold(([], 0),
                               |(a, s), t|
                                   if (sum < targetAmount) { Some((a + t, s+t.note.value) ) }
                                    else {None});



    println!("In sendmany");
}