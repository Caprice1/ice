use crate::transaction::Transaction;

//Things that need to be intergated


pub struct Block {
    pub vtx: Vec<Transaction>,
}

#[derive(Clone)]
pub struct BlockIndex<'a> {
    pub nHeight: i32,
    pub pprev: &'a BlockIndex<'a>,
}

impl<'a> BlockIndex<'a> {
    pub fn get_block_time(&self) -> i64 {
        0
    }
}
