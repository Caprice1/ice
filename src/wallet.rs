


use std::collections::HashMap;
//use bigint::U256;
use pairing::{
    bls12_381::{Bls12, Fr, FrRepr},
    PrimeField,
};

use crate::transaction::WalletTransaction;
use crate::sendmany::SaplingOutPoint;
use crate::incremental_tree::tree::{
    SaplingWitness, SaplingMerkleTree,
};
use crate::block_chain::{
    Block, BlockIndex,
};
use crate::transaction::NoteDataMap;
use crate::my::constants::WITNESS_CACHE_SIZE;

use crate::key::key_management::{
    SaplingOutputDescription, FrHash,
};



pub struct Wallet {
    pub map_wallet: HashMap<FrHash, WalletTransaction>,
    nWitnessCacheSize: usize,
}

impl Wallet{
    pub fn new() -> Self {
        Wallet{
            nWitnessCacheSize: 0,
            map_wallet: HashMap::new(),
        }
    }

    pub fn get_sapling_note_witnesses(&self, notes: Vec<&SaplingOutPoint>)
        -> (Vec<Option<&SaplingWitness>>, Option<FrHash>) {
        let mut rt: Option<FrHash> = None;

        let mut witnesses=
        notes.iter().map(|note|
            {
                self.map_wallet.get(&note.hash).and_then(
                    |tx| tx.mapSaplingData.get(&note).and_then(
                        |data| data.witnesses.front().and_then(
                            |witness| {

                                let r = witness.root().unwrap();

                                match rt.clone() {
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
    pub fn increment_note_witnesses(&mut self, pindex: &BlockIndex, pblockIn: &Block, saplingTree: &SaplingMerkleTree) {
        for (_, wtx) in self.map_wallet.iter_mut() {
            copy_previous_witnesses(&mut wtx.mapSaplingData, pindex.nHeight, self.nWitnessCacheSize);
        }
        if self.nWitnessCacheSize < WITNESS_CACHE_SIZE {
            self.nWitnessCacheSize += 1;
        }

        for tx in pblockIn.vtx.iter() {
            let hash = &tx.hash;
            let tx_is_ours = self.map_wallet.contains_key(hash);
            for (i, item) in tx.v_shielded_output.iter().enumerate() {
                //let repr = item.cmu.into_repr().as_ref();
                //let note_commitement = U256::from(repr);
                let cm = item.cmu;
                let note_commitement = FrHash(cm);
                let note_commitement_1 = note_commitement.clone();
                let note_commitement_2 = note_commitement.clone();
                saplingTree.append(note_commitement);



                for (_, wtx) in self.map_wallet.iter_mut() {
                    let cm = note_commitement_1.clone();
                    append_note_commitement(&mut wtx.mapSaplingData, pindex.nHeight,
                                        self.nWitnessCacheSize, cm);
                }

                if tx_is_ours {
                    let t_hash = tx.hash.clone();
                    let out_point = SaplingOutPoint{ hash: t_hash, n: i as u32};
                    let nd = self.map_wallet.get_mut(&hash).unwrap();

                    witness_note_if_mine(&mut nd.mapSaplingData, pindex.nHeight,
                                         self.nWitnessCacheSize, note_commitement_2,
                                         out_point, saplingTree.witness().unwrap());
                }
            }
        }

    }



}

fn copy_previous_witnesses(noteDataMap: &mut NoteDataMap , indexHeight: i32, nWitnessCacheSize: usize) {
    for (op, nd) in noteDataMap.iter_mut() {
        if nd.witnessHeight < indexHeight {
            assert!(nWitnessCacheSize >= nd.witnesses.len(), true);
            assert!(nd.witnessHeight == -1 || nd.witnessHeight == indexHeight-1);
            if nd.witnesses.len() > 0 {
                nd.push_front(nd.front().unwrap())
            }
            if nd.witnesses.len() > WITNESS_CACHE_SIZE {
                nd.pop_back();
            }
        }
    }
}

fn append_note_commitement(noteDataMap: &mut NoteDataMap, indexHeight: i32,
                           nWitnessCacheSize: usize, note_commitement: FrHash) {
    for (_, nd) in noteDataMap.iter_mut() {
        if nd.witnessHeight < indexHeight && nd.witnesses.len() > 0 {
            assert!(nWitnessCacheSize >= nd.witnesses.len());
            //nd.witnesses.front().
            //    and_then(|witness| witness.append(note_commitement));
            let cm = note_commitement.clone();
            nd.witnesses.front().unwrap().append(cm);
        }
    }
}

fn witness_note_if_mine(noteDataMap: &mut NoteDataMap, indexHeight: i32,
                        nWitnessCacheSize: usize, note_commitement: FrHash,
                        key: SaplingOutPoint, witness: SaplingWitness) {
    if noteDataMap.contains_key(&key) && noteDataMap[&key].witnessHeight < indexHeight {
        let nd = noteDataMap.get_mut(&key).unwrap();
        if nd.witnesses.len() > 0 {
            info!("Inconsistent witness cache state found");
            nd.witnesses.clear();
        }
        nd.push_front(witness);
        nd.witnessHeight = indexHeight - 1;
        assert!(nWitnessCacheSize >= nd.witnesses.len());
    }
}

pub fn show() {
    println!("Wallet show");






}