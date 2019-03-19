


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

    pub fn chain_tip(&mut self, pindex: &BlockIndex, pblock: &Block,
                     saplingTree: &mut SaplingMerkleTree, added: bool) {
        if added {
            self.increment_note_witnesses(pindex, pblock, saplingTree);
        } else {
            self.decrement_note_witnesses(pindex);
        }
        self.update_sapling_nullifier_note_map_for_block(pblock);
    }

    fn update_sapling_nullifier_note_map_for_block(&mut self, pblock: &Block) {
        for tx in pblock.vtx.iter() {
            let hash = &tx.hash;
            let tx_is_ours = self.map_wallet.contains_key(hash);
            if tx_is_ours {
                let wtx = self.map_wallet.get_mut(hash);
                //self.update_sapling_nullifier_note_map_with_tx(&mut wtx.unwrap());
            }
        }
    }
    //            uint64_t position = nd.witnesses.front().position();
    //            SaplingFullViewingKey fvk = mapSaplingFullViewingKeys.at(nd.ivk);
    //            OutputDescription output = wtx.vShieldedOutput[op.n];
    //            auto optPlaintext = SaplingNotePlaintext::decrypt(output.encCiphertext, nd.ivk, output.ephemeralKey, output.cm);
    //            if (!optPlaintext) {
    //                // An item in mapSaplingNoteData must have already been successfully decrypted,
    //                // otherwise the item would not exist in the first place.
    //                assert(false);
    //            }
    //            auto optNote = optPlaintext.get().note(nd.ivk);
    //            if (!optNote) {
    //                assert(false);
    //            }
    //            auto optNullifier = optNote.get().nullifier(fvk, position);
    //            if (!optNullifier) {
    //                // This should not happen.  If it does, maybe the position has been corrupted or miscalculated?
    //                assert(false);
    //            }
    //            uint256 nullifier = optNullifier.get();
    //            mapSaplingNullifiersToNotes[nullifier] = op;
    //            item.second.nullifier = nullifier;

    fn update_sapling_nullifier_note_map_with_tx(&mut self, wtx: &mut WalletTransaction) {

        for (op, nd) in wtx.mapSaplingData.iter() {
            if nd.witnesses.is_empty() {

            } else {

            }

        }
    }

    ///**
    // * Update mapSaplingNullifiersToNotes, computing the nullifier from a cached witness if necessary.
    // */
    //void CWallet::UpdateSaplingNullifierNoteMapWithTx(CWalletTx& wtx) {
    //    LOCK(cs_wallet);
    //
    //    for (mapSaplingNoteData_t::value_type &item : wtx.mapSaplingNoteData) {
    //        SaplingOutPoint op = item.first;
    //        SaplingNoteData nd = item.second;
    //
    //        if (nd.witnesses.empty()) {
    //            // If there are no witnesses, erase the nullifier and associated mapping.
    //            if (item.second.nullifier) {
    //                mapSaplingNullifiersToNotes.erase(item.second.nullifier.get());
    //            }
    //            item.second.nullifier = boost::none;
    //        }
    //        else {
    //            uint64_t position = nd.witnesses.front().position();
    //            SaplingFullViewingKey fvk = mapSaplingFullViewingKeys.at(nd.ivk);
    //            OutputDescription output = wtx.vShieldedOutput[op.n];
    //            auto optPlaintext = SaplingNotePlaintext::decrypt(output.encCiphertext, nd.ivk, output.ephemeralKey, output.cm);
    //            if (!optPlaintext) {
    //                // An item in mapSaplingNoteData must have already been successfully decrypted,
    //                // otherwise the item would not exist in the first place.
    //                assert(false);
    //            }
    //            auto optNote = optPlaintext.get().note(nd.ivk);
    //            if (!optNote) {
    //                assert(false);
    //            }
    //            auto optNullifier = optNote.get().nullifier(fvk, position);
    //            if (!optNullifier) {
    //                // This should not happen.  If it does, maybe the position has been corrupted or miscalculated?
    //                assert(false);
    //            }
    //            uint256 nullifier = optNullifier.get();
    //            mapSaplingNullifiersToNotes[nullifier] = op;
    //            item.second.nullifier = nullifier;
    //        }
    //    }
    //}



    // void CWallet::UpdateSaplingNullifierNoteMapForBlock(const CBlock *pblock) {
    //    LOCK(cs_wallet);
    //
    //    for (const CTransaction& tx : pblock->vtx) {
    //        auto hash = tx.GetHash();
    //        bool txIsOurs = mapWallet.count(hash);
    //        if (txIsOurs) {
    //            UpdateSaplingNullifierNoteMapWithTx(mapWallet[hash]);
    //        }
    //    }
    //}

    pub fn decrement_note_witnesses(&mut self, pindex: &BlockIndex) {

    }


    //void CWallet::IncrementNoteWitnesses(const CBlockIndex* pindex,
    //                                     const CBlock* pblockIn,
    //                                     SproutMerkleTree& sproutTree,
    //                                     SaplingMerkleTree& saplingTree)
    //{
    pub fn increment_note_witnesses(&mut self, pindex: &BlockIndex, pblockIn: &Block, saplingTree: &mut SaplingMerkleTree) {
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

        for (_, wtx) in self.map_wallet.iter_mut() {
            update_witness_heights(&mut wtx.mapSaplingData, pindex.nHeight, self.nWitnessCacheSize);
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

fn update_witness_heights(noteDataMap: &mut NoteDataMap, indexHeight: i32, nWitnessCacheSize: usize) {
    for (op, nd) in noteDataMap.iter_mut() {
        if nd.witnessHeight < indexHeight {
            nd.witnessHeight = indexHeight;
            assert!(nWitnessCacheSize >= nd.witnesses.len());
        }
    }
}


pub fn show() {
    println!("Wallet show");

}