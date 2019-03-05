
use std::collections::HashMap;
use bigint::U256;

use crate::transaction::Transaction;
use crate::sendmany::SaplingOutPoint;
use crate::sapling_witness::SaplingWitness;


pub struct Wallet {
    /*
    std::map<uint256, CWalletTx> mapWallet;
    */
    pub map_wallet: HashMap<U256, Transaction>,
}

impl Wallet{
    pub fn new() -> Self {
        Wallet{
            map_wallet: HashMap::new(),
        }
    }

    fn get_sapling_note_witnesses(&self, notes: Vec<SaplingOutPoint>,
                                witnesses: &mut Vec<SaplingWitness>,
                                anchor: &mut U256) {

    }
}

pub fn show() {
    println!("Wallet show");
}