use crate::amount::FeeRate;
use crate::key::key_management::FrHash;
use crate::sendmany::SaplingOutPoint;
use crate::transaction::SaplingInPoint;
use crate::transaction::Transaction;
use ethereum_types::U256;
use std::collections::{HashMap, VecDeque};

pub struct TxMemPoolEntry<'a> {
    tx: &'a Transaction,
    n_fee: u32,
    n_mode_size: u32,
    n_usage_size: u32,
    fee_rate: FeeRate,
    n_time: i64,
    d_priority: f64,
    n_height: u32,
    had_no_dependencies: bool,
    spends_coinbase: bool,
    n_branch_id: u32,
}

impl<'a> TxMemPoolEntry<'a> {
    pub fn new(tx: &'a Transaction) -> Self {
        TxMemPoolEntry {
            tx: tx,
            n_fee: 0,
            n_mode_size: 0,
            n_usage_size: 0,
            fee_rate: FeeRate::new(),
            n_time: 0,
            d_priority: 0.0,
            n_height: 0,
            had_no_dependencies: false,
            spends_coinbase: false,
            n_branch_id: 0,
        }
    }
}

pub struct TxMemPool<'a> {
    pub mapTx: HashMap<FrHash, TxMemPoolEntry<'a>>,
    pub mapNextTx: HashMap<SaplingOutPoint, SaplingInPoint<'a>>,
    pub map_sapling_nullifier: HashMap<U256, &'a Transaction>,
}

impl<'a> TxMemPool<'a> {
    pub fn exists(&self, hash: FrHash) -> bool {
        self.mapTx.contains_key(&hash)
    }

    pub fn nullifier_exists(&self, nullifier: U256) -> bool {
        self.map_sapling_nullifier.contains_key(&nullifier)
    }

    pub fn add_unchecked(&mut self, hash: FrHash, entry: TxMemPoolEntry<'a>) {
        let tx = entry.tx;
        for i in 0..tx.vin.len() {
            //self.mapNextTx[&tx.vin[i].prevout] = SaplingInPoint::new(&tx, i);
            self.mapNextTx
                .insert(tx.vin[i].prevout, SaplingInPoint::new(&tx, i));
        }
        for spend_desciption in tx.v_shielded_spend.iter() {
            //self.map_sapling_nullifier[&U256::from(spend_desciption.nullifier)] = tx;
            self.map_sapling_nullifier
                .insert(U256::from(spend_desciption.nullifier), tx);
        }

        self.mapTx.insert(hash, entry);
    }

    pub fn remove_for_block(
        &mut self,
        vtx: &Vec<Transaction>,
        n_block_height: u32,
        conflicts: &mut VecDeque<&'a Transaction>,
        f_current_estimeate: bool,
    ) {
        let mut entries = Vec::new();
        for tx in vtx.iter() {
            let hash = &tx.hash;
            if self.mapTx.contains_key(hash) {
                entries.push(self.mapTx.get(hash).unwrap());
            }
        }
        for tx in vtx.iter() {
            let mut dummy = VecDeque::new();
            self.remove(tx, &mut dummy, false);
            self.remove_conflicts(tx, conflicts);
        }
    }

    pub fn remove(
        &mut self,
        orig_tx: &Transaction,
        removed: &mut VecDeque<&'a Transaction>,
        f_recursive: bool,
    ) {
        let mut tx_to_remove = VecDeque::new();
        tx_to_remove.push_back(orig_tx.hash);
        for i in 0..orig_tx.vout.len() {
            let outpoint = SaplingOutPoint {
                hash: orig_tx.hash,
                n: i,
            };
            let tx = self.mapNextTx.get(&outpoint);
            if !tx.is_none() {
                tx_to_remove.push_back(tx.unwrap().ptx.hash);
            }
        }
        while !tx_to_remove.is_empty() {
            let hash = tx_to_remove.pop_front().unwrap();
            if !self.mapTx.contains_key(&hash) {
                continue;
            }
            let tx = self.mapTx.get(&hash).unwrap().tx;
            if f_recursive {
                for i in 0..orig_tx.vout.len() {
                    let outpoint = SaplingOutPoint {
                        hash: orig_tx.hash,
                        n: i,
                    };
                    let tx = self.mapNextTx.get(&outpoint);
                    if !tx.is_none() {
                        tx_to_remove.push_back(tx.unwrap().ptx.hash);
                    }
                }
            }
            for txin in tx.vin.iter() {
                self.mapNextTx.remove(&txin.prevout);
            }

            removed.push_back(tx);
        }
    }

    pub fn remove_conflicts(&mut self, tx: &Transaction, removed: &mut VecDeque<&'a Transaction>) {
        //let result = VecDeque::new();
        for txin in tx.vin.iter() {
            let op = self.mapNextTx.get(&txin.prevout);
            if !op.is_none() {
                let inpoint = op.unwrap();
                let tx_conflict = inpoint.ptx;
                if tx_conflict.hash != tx.hash {
                    self.remove(tx_conflict, removed, true);
                }
            }
        }

        for spend_description in tx.v_shielded_spend.iter() {
            let op = self
                .map_sapling_nullifier
                .get(&U256::from(spend_description.nullifier));
            if !op.is_none() {
                let tx_conflict = op.unwrap();
                if tx_conflict.hash != tx.hash {
                    self.remove(tx_conflict, removed, true);
                }
            }
        }
    }
}
