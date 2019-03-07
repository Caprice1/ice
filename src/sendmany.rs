


extern crate zip32;
extern crate pairing;

use zip32::ExpandedSpendingKey;

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

use crate::my::constants;
use crate::wallet::Wallet;
use crate::incremental_tree::{
    SaplingWitness,
};

use crate::key::key_management::SaplingIncomingViewingKey;


//static mut pMainWallet: Wallet = Wallet::new();

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

#[derive(PartialEq, Eq, Hash)]
pub struct SaplingOutPoint
{
    pub hash: U256,
    pub n: u32,
}

/*
impl Eq for SaplingOutPoint {

}

impl Hash for SaplingOutPoint {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hash.hash(state);
        self.n.hash(state);
    }
}*/

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
    memo: [char; constants::ZC_MEMO_SIZE],
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
        /*
        if (isfromzaddr_) {
                auto sk = boost::get<libzcash::SaplingExtendedSpendingKey>(spendingkey_);
                expsk = sk.expsk;
                ovk = expsk.full_viewing_key().ovk;
        }
        */

        //let mut ops = Vec::new();
        //let mut notes = Vec::new();
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

        /*let mut witnesses: Vec<SaplingWitness> = Vec::new();
        let mut anchor: U256 = U256::from(0);
        self.main_wallet.get_sapling_note_witnesses(&ops, &mut witnesses, &mut anchor);
        */

        let (witnesses, anchor)
            = self.main_wallet.get_sapling_note_witnesses(&ops);





        println!("In sendmany");
    }
}