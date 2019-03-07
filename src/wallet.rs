
use std::collections::HashMap;
use bigint::U256;

use crate::transaction::Transaction;
use crate::sendmany::SaplingOutPoint;
use crate::incremental_tree::SaplingWitness;


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

    pub fn get_sapling_note_witnesses(&self, notes: &Vec<&SaplingOutPoint>)
        -> (Vec<&Box<SaplingWitness>>, Option<U256>) {
        let mut witnesses: Vec<&Box<SaplingWitness>> = Vec::new();
        let mut rt: Option<U256> = None;

        for note in notes {
            self.map_wallet.get(&note.hash).and_then(
                |tx| tx.mapSaplingData.get(&note).and_then(
                    |data| data.witnesses.front().and_then(
                        |witness| {
                            witnesses.push(witness);
                            let r = witness.root();

                            match rt {
                                None => { rt = Some(r);}
                                Some(root) => {assert_eq!(root, r);}
                            }
                            rt
                        }
                    )
                )
            );
        }

        (witnesses, rt)
    }

}

pub fn show() {
    println!("Wallet show");
}