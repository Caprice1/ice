use crate::key::key_management::FrHash;
use crate::transaction::Transaction;

//Things that need to be intergated

pub struct Block {
    pub vtx: Vec<Transaction>,
}

#[derive(Clone)]
pub struct BlockIndex<'a> {
    pub nHeight: i32,
    pub pprev: Option<&'a BlockIndex<'a>>,
    pub hash_final_sapling_root: FrHash,
}

impl<'a> BlockIndex<'a> {
    pub fn get_block_time(&self) -> i64 {
        0
    }
}
