

use std::collections::HashMap;
use crate::sendmany::SaplingOutPoint;
use crate::sendmany::SaplingNoteData;
use crate::key::key_management::{
   SaplingSpendDescription, SaplingOutputDescription,FrHash,
};
use bigint::U256;
//use pairing::bls12_381::Fr;

pub type NoteDataMap =  HashMap<SaplingOutPoint, SaplingNoteData>;

pub struct WalletTransaction{
    //std::map<SaplingOutPoint, SaplingNoteData> mapSaplingData;
    pub mapSaplingData: NoteDataMap,

}

pub struct Transaction {
    pub hash: FrHash, //U256,
    pub v_shielded_spend: Vec<SaplingSpendDescription>,
    pub v_shielded_output: Vec<SaplingOutputDescription>,

}