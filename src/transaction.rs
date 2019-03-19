use crate::key::key_management::{FrHash, SaplingOutputDescription, SaplingSpendDescription};
use crate::sendmany::SaplingNoteData;
use crate::sendmany::SaplingOutPoint;
use bigint::U256;
use std::collections::HashMap;
//use pairing::bls12_381::Fr;

pub type NoteDataMap = HashMap<SaplingOutPoint, SaplingNoteData>;

//Program cache
pub struct WalletTransaction {
    //std::map<SaplingOutPoint, SaplingNoteData> mapSaplingData;
    pub mapSaplingData: NoteDataMap,
}

//In DB and network
pub struct Transaction {
    pub hash: FrHash, //U256,
    pub v_shielded_spend: Vec<SaplingSpendDescription>,
    pub v_shielded_output: Vec<SaplingOutputDescription>,
}
