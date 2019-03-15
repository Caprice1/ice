
use std::collections::HashMap;
use bigint::U256;

use crate::transaction::Transaction;
use crate::sendmany::SaplingOutPoint;
use crate::incremental_tree::tree::{
    SaplingWitness, SaplingMerkleTree,
};
use crate::block_chain::{
    Block, BlockIndex,
};
use crate::transaction::NoteDataMap;
use crate::my::constants::WITNESS_CACHE_SIZE;


pub struct Wallet {
    pub map_wallet: HashMap<U256, Transaction>,
}

impl Wallet{
    pub fn new() -> Self {
        Wallet{
            map_wallet: HashMap::new(),
        }
    }

    pub fn get_sapling_note_witnesses(&self, notes: Vec<&SaplingOutPoint>)
        -> (Vec<Option<&SaplingWitness>>, Option<U256>) {
        let mut rt: Option<U256> = None;

        let mut witnesses=
        notes.iter().map(|note|
            {
                self.map_wallet.get(&note.hash).and_then(
                    |tx| tx.mapSaplingData.get(&note).and_then(
                        |data| data.witnesses.front().and_then(
                            |witness| {

                                let r = witness.root().unwrap();

                                match rt {
                                    None => { rt = Some(r);}
                                    Some(root) => {assert_eq!(root, r);}
                                }
                                Some(witness)
                            }
                        )
                    )
                )
            }
        ).collect::<Vec<_>>();

        (witnesses, rt)
    }

    //void CWallet::IncrementNoteWitnesses(const CBlockIndex* pindex,
    //                                     const CBlock* pblockIn,
    //                                     SproutMerkleTree& sproutTree,
    //                                     SaplingMerkleTree& saplingTree)
    //{
    pub fn increment_note_witnesses(&self, pindex: &BlockIndex, pblockIn: &Block, saplingTree: &SaplingMerkleTree) {

    }

    fn copy_previous_witnesses(noteDataMap: &mut NoteDataMap , indexHeight: i32, nWitnessCacheSize: usize) {
        for (op, nd) in noteDataMap.iter_mut() {
            if nd.witnessHeight < indexHeight {
                assert!(nWitnessCacheSize >= nd.witnesses.len(), true);
                assert!(nd.witnessHeight == -1 || nd.witnessHeight == indexHeight-1);
                //let mut mut_nd = nd;
                if nd.witnesses.len() > 0 {
                    //nd.push_front(nd.witnesses.front().unwrap());
                    nd.push_front(nd.front().unwrap())
                }
                if nd.witnesses.len() > WITNESS_CACHE_SIZE {
                    nd.pop_back();
                }
            }
        }
    }



}

pub fn show() {
    println!("Wallet show");






}