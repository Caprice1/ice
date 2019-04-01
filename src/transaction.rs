use crate::key::key_management::{FrHash, SaplingOutputDescription, SaplingSpendDescription};
use crate::script::Script;
use crate::sendmany::CAmount;
use crate::sendmany::SaplingNoteData;
use crate::sendmany::SaplingOutPoint;
use ethereum_types::U256;
use std::collections::HashMap;

pub type NoteDataMap = HashMap<SaplingOutPoint, SaplingNoteData>;

//Program cache
pub struct WalletTransaction {
    //std::map<SaplingOutPoint, SaplingNoteData> mapSaplingData;
    pub mapSaplingData: NoteDataMap,
}

pub struct TxIn {
    prevout: SaplingOutPoint,
    script_sig: Script,
}

pub struct TxOut {
    n_value: CAmount,
    script_pub_key: Script,
}
//In DB and network
pub struct Transaction {
    pub hash: FrHash, //U256,

    pub vin: Vec<TxIn>,
    pub vout: Vec<TxOut>,
    pub v_shielded_spend: Vec<SaplingSpendDescription>,
    pub v_shielded_output: Vec<SaplingOutputDescription>,
}

impl Transaction {
    //TODO
    pub fn is_coin_base(&self) -> bool {
        false
    }
}

pub struct TxUndo {
    vpreout: Vec<TxInUndo>,
}

pub struct TxInUndo {}
