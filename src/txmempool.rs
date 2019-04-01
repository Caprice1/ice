use crate::amount::FeeRate;
use crate::key::key_management::FrHash;
use crate::sendmany::SaplingOutPoint;
use crate::transaction::SaplingInPoint;
use crate::transaction::Transaction;
use std::collections::HashMap;

pub struct TxMemPoolEntry {
    tx: Transaction,
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

pub struct TxMemPool<'a> {
    pub mapTx: HashMap<FrHash, TxMemPoolEntry>,
    pub mapNextTx: HashMap<SaplingOutPoint, SaplingInPoint<'a>>,
}

impl<'a> TxMemPool<'a> {
    pub fn exists(&self, hash: FrHash) -> bool {
        self.mapTx.contains_key(&hash)
    }
}
