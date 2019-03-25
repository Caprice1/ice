use crate::key::key_management::FrHash;
use crate::transaction::Transaction;

//Things that need to be intergated

pub struct Block {
    pub vtx: Vec<Transaction>,
}

#[derive(Clone)]
pub struct BlockIndex {
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
}

pub struct ChainActive {

}

impl ChainActive {

    pub fn new() -> Self {
        ChainActive {}
    }

    pub fn next(&self, pindex: BlockIndex) -> Option<BlockIndex> {
        None
    }
}
