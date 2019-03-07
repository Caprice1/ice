


extern crate zip32;
extern crate pairing;



use pairing::{
    bls12_381::{Bls12, Fr, FrRepr},
};

use sapling_crypto::{
    primitives::{Note, PaymentAddress},
};

use bigint::U256;

use std::collections::LinkedList;
use std::cmp::Eq;
use std::hash::{Hash, Hasher};

use crate::my::constants::ZC_MEMO_SIZE;
use crate::wallet::Wallet;
use crate::incremental_tree::tree::{
    SaplingWitness,
};

use crate::key::key_management::{
    SaplingIncomingViewingKey,
    SaplingExpandedSpendingKey,
    SaplingNote,
};


//static mut pMainWallet: Wallet = Wallet::new();

//SaplingNote

pub struct SpendDescriptionInfo {
    pub expsk: SaplingExpandedSpendingKey,
    pub note: SaplingNote,
    pub alpha: Fr,
    pub ahchor: Fr,
}

pub struct OutputDescriptionInfo {
    pub ovk: U256,
    pub note: SaplingNote,
    pub memo: [char; ZC_MEMO_SIZE],

}

#[derive(PartialEq, Eq, Hash)]
pub struct SaplingOutPoint
{
    pub hash: U256,
    pub n: u32,
}


pub struct SaplingNoteData
{
    /*
    std::list<SaplingWitness> witnesses;
    int witnessHeight;
    libzcash::SaplingIncomingViewingKey ivk;
    boost::optional<uint256> nullifier;
    */

    pub witnesses: LinkedList<Box<SaplingWitness>>,

    pub witnessHeight: u64,

    pub ivk: SaplingIncomingViewingKey,

    pub nullifier: Option<U256>,
}


pub struct SaplingNoteEntry {
    op: SaplingOutPoint,
    //PaymentAddress not sure is Bls12
    address: PaymentAddress<Bls12>,
    note: Note<Bls12>,
    memo: [char; ZC_MEMO_SIZE],
    confirmation: i32,
}

type CAmount = u64;

pub struct SendManyRecipient(String, CAmount, String);

pub fn show() {
    println!("sendmany show");

    let v = [1, 2, 3, 4, 5];

}

pub struct Sender {
    pub main_wallet: Wallet,
}

impl Sender {

    pub fn send_many(&self, _z_inputs_: Vec<SaplingNoteEntry>, _z_outputs_: Vec<SendManyRecipient>) {

        let mut target_amount = 100;
        let result = _z_inputs_.iter().try_fold((vec![], vec![], 0),
                                    |(mut a, mut b, s), t|
                                       if s < target_amount {
                                               a.push(&t.op);
                                               b.push(&t.note);
                                               Some((a, b, s+t.note.value))
                                           }
                                           else {None});
        assert_eq!(result.is_none(), false);

        let (ops, notes, _) = result.unwrap();

        let (witnesses, anchor)
            = self.main_wallet.get_sapling_note_witnesses(&ops);

        for witness_op in witnesses {
            match witness_op {
                None => { panic!("No witness found");  }
                Some(witness) => {

                }
            }
        }





        println!("In sendmany");
    }
}