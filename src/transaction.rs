use crate::key::key_management::{FrHash, SaplingOutputDescription, SaplingSpendDescription};
use crate::script::Script;
use crate::sendmany::CAmount;
use crate::sendmany::SaplingNoteData;
use crate::sendmany::SaplingOutPoint;
use crate::wallet::Wallet;
use ethereum_types::U256;
use std::collections::HashMap;

pub type NoteDataMap = HashMap<SaplingOutPoint, SaplingNoteData>;

//Program cache
pub struct WalletTransaction {
    //std::map<SaplingOutPoint, SaplingNoteData> mapSaplingData;
    pub mapSaplingData: NoteDataMap,
    pub tx: Transaction,
    //pub p_wallet: &'a Wallet,
}

impl WalletTransaction {
    pub fn new(tx: Transaction) -> Self {
        WalletTransaction {
            mapSaplingData: NoteDataMap::new(),
            tx,
            //p_wallet
        }
    }

    //TODO
    pub fn bind_wallet(&self, p_wallet_in: &Wallet) {}
}

/** An inpoint - a combination of a transaction and an index n into its vin */
pub struct SaplingInPoint<'a> {
    pub ptx: &'a Transaction,
    pub n: usize,
}

impl<'a> SaplingInPoint<'a> {
    pub fn new(tx: &'a Transaction, index: usize) -> Self {
        SaplingInPoint { ptx: tx, n: index }
    }
}

#[derive(Clone)]
pub struct TxIn {
    pub prevout: SaplingOutPoint,
    pub script_sig: Script,
}

#[derive(Copy, Clone)]
pub struct TxOut {
    pub n_value: i64,
    pub script_pub_key: Script,
}

impl TxOut {
    pub fn is_null(&self) -> bool {
        self.n_value == -1
    }
    pub fn set_null(&mut self) {
        self.n_value = -1;
        self.script_pub_key.clear();
    }
}
//In DB and network
#[derive(Clone)]
pub struct Transaction {
    pub hash: FrHash, //U256,

    pub vin: Vec<TxIn>,
    pub vout: Vec<TxOut>,
    pub v_shielded_spend: Vec<SaplingSpendDescription>,
    pub v_shielded_output: Vec<SaplingOutputDescription>,
    pub balancing_value: i64,
    pub binding_sig: [u8; 64],
}

impl Transaction {
    //TODO
    pub fn is_coin_base(&self) -> bool {
        false
    }
}

/*
pub struct TxUndo {
    vpreout: Vec<TxInUndo>,
}

pub struct TxInUndo {}*/
