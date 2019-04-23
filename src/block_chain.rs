use crate::key::key_management::FrHash;
use crate::transaction::{Transaction, TxOut};

//Things that need to be intergated
use ethereum_types::U256;

pub struct Block {
    pub vtx: Vec<Transaction>,
}

pub struct DiskBlockPos {}

pub struct TxInUndo {
    pub txout: TxOut,      // the txout data before being spent
    pub f_coin_base: bool, // if the outpoint was the last unspent: whether it belonged to a coinbase
    pub n_height: i32,
}

impl TxInUndo {
    pub fn new(txout_in: TxOut) -> Self {
        TxInUndo {
            txout: txout_in,
            f_coin_base: false,
            n_height: 0,
        }
    }

    pub fn set_n_height(&mut self, n_height: i32) {
        self.n_height = n_height;
    }

    pub fn set_f_coin_base(&mut self, f_coin_base: bool) {
        self.f_coin_base = f_coin_base;
    }
}

pub struct TxUndo {
    pub vprevout: Vec<TxInUndo>,
}

impl TxUndo {
    pub fn new() -> Self {
        TxUndo {
            vprevout: Vec::new(),
        }
    }
}

pub struct BlockUndo {
    pub vtxundo: Vec<TxUndo>,
}

#[derive(Clone)]
pub struct BlockIndex {
    phash_block: U256,

    pub nHeight: i32,

    pub pprev: Option<Box<BlockIndex>>,
    pub hash_final_sapling_root: FrHash,
}

impl BlockIndex {
    pub fn get_block_time(&self) -> i64 {
        0
    }
    pub fn get_pprev(self) -> Option<BlockIndex> {
        self.pprev.map(|b| *b)
    }
    pub fn get_block_hash(&self) -> U256 {
        self.phash_block
    }
    pub fn get_undo_pos(&self) -> DiskBlockPos {
        unimplemented!()
    }
}

pub struct Chain {
    pub v_chain: Vec<BlockIndex>,
}

impl Chain {
    pub fn new() -> Self {
        Chain {
            v_chain: Vec::new(),
        }
    }

    pub fn tip(&self) -> Option<&BlockIndex> {
        self.v_chain.last()
    }

    pub fn next(&self, pindex: BlockIndex) -> Option<BlockIndex> {
        None
    }
}

pub struct ValidationState {}
