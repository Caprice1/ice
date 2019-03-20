use crate::transaction::Transaction;

//Things that need to be intergated

pub struct Block {
    pub vtx: Vec<Transaction>,
}

pub struct BlockIndex {
    pub nHeight: i32,
}
