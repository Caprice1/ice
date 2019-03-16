

use std::collections::HashMap;
use crate::sendmany::SaplingOutPoint;
use crate::sendmany::SaplingNoteData;
use crate::key::key_management::{
   SaplingSpendDescription, SaplingOutputDescription,
};
use bigint::U256;

pub type NoteDataMap =  HashMap<SaplingOutPoint, SaplingNoteData>;

pub struct WalletTransaction{
    //std::map<SaplingOutPoint, SaplingNoteData> mapSaplingData;
    pub mapSaplingData: NoteDataMap,

}

pub struct Transaction {
    pub hash: U256,
    pub v_shielded_spend: Vec<SaplingSpendDescription>,
    pub v_shielded_output: Vec<SaplingOutputDescription>,

}