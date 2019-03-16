
use std::collections::HashMap;
use bigint::U256;

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

use crate::key::key_management::SaplingOutputDescription;

pub struct Wallet {
    pub map_wallet: HashMap<U256, WalletTransaction>,
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
    pub fn increment_note_witnesses(&mut self, pindex: &BlockIndex, pblockIn: &Block, saplingTree: &SaplingMerkleTree) {
        for (_, tx) in self.map_wallet.iter_mut() {
            copy_previous_witnesses(&mut tx.mapSaplingData, pindex.nHeight, self.nWitnessCacheSize);
        }
        if self.nWitnessCacheSize < WITNESS_CACHE_SIZE {
            self.nWitnessCacheSize += 1;
        }

        for tx in pblockIn.vtx.iter() {
            let hash = tx.hash;
            let tx_is_ours = self.map_wallet.contains_key(&hash);
            for (i, item) in tx.v_shielded_output.iter().enumerate() {

            }
        }



        //for (const CTransaction& tx : pblock->vtx) {
        //        auto hash = tx.GetHash();
        //        bool txIsOurs = mapWallet.count(hash);
        //        // Sapling
        //        for (uint32_t i = 0; i < tx.vShieldedOutput.size(); i++) {
        //            const uint256& note_commitment = tx.vShieldedOutput[i].cm;
        //            saplingTree.append(note_commitment);
        //
        //            // Increment existing witnesses
        //            for (std::pair<const uint256, CWalletTx>& wtxItem : mapWallet) {
        //                ::AppendNoteCommitment(wtxItem.second.mapSaplingNoteData, pindex->nHeight, nWitnessCacheSize, note_commitment);
        //            }
        //
        //            // If this is our note, witness it
        //            if (txIsOurs) {
        //                SaplingOutPoint outPoint {hash, i};
        //                ::WitnessNoteIfMine(mapWallet[hash].mapSaplingNoteData, pindex->nHeight, nWitnessCacheSize, outPoint, saplingTree.witness());
        //            }
        //        }
        //    }
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

pub fn show() {
    println!("Wallet show");






}