use crate::incremental_tree::tree::SaplingMerkleTree;
use crate::key::key_management::FrHash;
use crate::transaction::Transaction;
use crate::transaction::{TxIn, TxOut};
use std::collections::hash_map::HashMap;

use ethereum_types::U256;

pub struct Coins {
    pub f_coin_base: bool,

    pub n_height: i32,

    pub vout: Vec<TxOut>,
}

impl Coins {
    pub fn new() -> Self {
        Coins {
            f_coin_base: false,
            n_height: 0,
            vout: Vec::new(),
        }
    }

    pub fn is_available(&self, n_pos: usize) -> bool {
        n_pos < self.vout.len() && !self.vout[n_pos].is_null()
    }

    pub fn clear_unspendable(&mut self) {
        for txout in self.vout.iter_mut() {
            if (txout.script_pub_key.is_unspendable()) {
                txout.set_null();
            }
        }
        self.clean_up();
    }

    //TODO, should release memory?
    pub fn clean_up(&mut self) {
        while self.vout.len() > 0 && self.vout.last().unwrap().is_null() {
            self.vout.pop();
        }
    }

    pub fn clear(&mut self) {
        self.f_coin_base = false;
        self.n_height = 0;
    }

    // check whether the entire Coins is spent
    pub fn is_pruned(&self) -> bool {
        for out in self.vout.iter() {
            if !out.is_null() {
                return false;
            }
        }
        true
    }
}

pub struct CoinsModifier<'a> {
    pub entry: &'a mut CoinsCacheEntry,
}

impl<'a> CoinsModifier<'a> {
    pub fn new(entry: &'a mut CoinsCacheEntry) -> Self {
        CoinsModifier { entry }
    }

    pub fn clear_unspendable(&mut self) {
        self.entry.coins.clear_unspendable();
    }

    pub fn clear(&mut self) {
        self.entry.coins.clear();
    }

    pub fn is_pruned(&self) -> bool {
        self.entry.coins.is_pruned()
    }
}

pub struct AnchorsSaplingCacheEntry {
    entered: bool,
    dirty: bool,
    tree: SaplingMerkleTree,
    //flags: char,
}

impl AnchorsSaplingCacheEntry {
    fn new(tree: SaplingMerkleTree) -> Self {
        AnchorsSaplingCacheEntry {
            entered: false,
            dirty: true,
            //flags: char::from(0),
            tree: tree,
        }
    }
}

struct NullifiersCacheEntry {
    entered: bool,
    dirty: bool,
}

pub struct CoinsCacheEntry {
    pub coins: Coins,
    pub dirty: bool,
    pub fresh: bool,
}

impl CoinsCacheEntry {
    pub fn new() -> Self {
        CoinsCacheEntry {
            coins: Coins::new(),
            dirty: false,
            fresh: false,
        }
    }
}

impl NullifiersCacheEntry {
    fn new() -> Self {
        NullifiersCacheEntry {
            entered: false,
            dirty: false,
        }
    }
}

type AnchorsSaplingMap = HashMap<FrHash, AnchorsSaplingCacheEntry>;
type NullifiersMap = HashMap<U256, NullifiersCacheEntry>;
type CoinsMap = HashMap<FrHash, CoinsCacheEntry>;

pub trait CoinsView {
    fn get_best_anchor(&self) -> Option<FrHash>;

    fn get_best_block(&self) -> U256;

    //Retrieve the tree (Sapling) at a particular anchored root in the chain
    fn get_sapling_anchor_at(&mut self, rt: FrHash) -> Option<SaplingMerkleTree>;

    //Determine whether a nullifier is spent or not
    fn get_nullifier(&mut self, nullifier: U256) -> bool;

    //fn push_anchor(&mut self, tree: SaplingMerkleTree);

    fn set_best_block(&mut self, block_hash: U256);

    fn have_coins(&self, txid: FrHash) -> bool;
}

//
pub struct CoinViewDB {}

impl CoinViewDB {
    pub fn new() -> Self {
        CoinViewDB {}
    }
}

//TODO, not necessary now
impl CoinsView for CoinViewDB {
    fn get_best_anchor(&self) -> Option<FrHash> {
        None
    }

    fn get_best_block(&self) -> U256 {
        unimplemented!()
    }

    fn get_sapling_anchor_at(&mut self, rt: FrHash) -> Option<SaplingMerkleTree> {
        None
    }

    fn get_nullifier(&mut self, nullifier: U256) -> bool {
        false
    }

    fn set_best_block(&mut self, block_hash: U256) {}

    fn have_coins(&self, txid: FrHash) -> bool {
        false
    }
}

pub struct CoinViewCache {
    //mutable uint256 hashSaplingAnchor;
    hash_block: U256,
    hash_sapling_anchor: Option<FrHash>,
    cache_coins: CoinsMap,
    cached_sapling_anchors: AnchorsSaplingMap,
    cached_sapling_nullifiers: NullifiersMap,
    base: CoinViewDB,
}

impl CoinViewCache {
    pub fn new() -> Self {
        CoinViewCache {
            hash_block: U256::from(0),
            hash_sapling_anchor: None,
            cache_coins: CoinsMap::new(),
            cached_sapling_anchors: AnchorsSaplingMap::new(),
            cached_sapling_nullifiers: NullifiersMap::new(),
            base: CoinViewDB::new(),
        }
    }

    pub fn set_nullifiers(&mut self, tx: &Transaction, spent: bool) {
        for spend_description in tx.v_shielded_spend.iter() {
            let mut entry = NullifiersCacheEntry::new();
            entry.entered = spent;
            entry.dirty = true;
            //TODO
            let nullifier = U256::from(spend_description.nullifier);
            //let nullifier = from_to_u256(&spend_description.nullifier);
            self.cached_sapling_nullifiers.insert(nullifier, entry);
        }
    }
}

impl CoinViewCache {
    pub fn push_anchor(&mut self, tree: SaplingMerkleTree) {
        let newrt = tree.root().unwrap();
        let current_root = self.get_best_anchor().unwrap();
        if newrt != current_root {
            let cache_entry = AnchorsSaplingCacheEntry::new(tree);
            self.cached_sapling_anchors.insert(newrt, cache_entry);
        }
        self.hash_sapling_anchor = Some(newrt);
    }

    pub fn pop_anchor(&mut self, newrt: FrHash) {
        let current_root = self.get_best_anchor();
        if !current_root.is_none() {
            let current_root = current_root.unwrap();
            if current_root != newrt {
                let entry = self.cached_sapling_anchors.get_mut(&current_root);
                if !entry.is_none() {
                    let e = entry.unwrap();
                    e.entered = false;
                    e.dirty = true;
                }
                /*entry.and_then(|e| {
                    e.entered = false;
                    e.dirty = true;
                });*/
            };
            self.hash_sapling_anchor = Some(newrt);
        }
        /*current_root.and_then(|current_root| {
            if current_root != newrt {
                let entry = self.cached_sapling_anchors.get_mut(&current_root);
                entry.and_then(|e| {
                    e.entered = false;
                    e.dirty = true;
                    None
                });
            };
            self.hash_sapling_anchor = Some(newrt);
            None
        });*/
    }

    /**
     * Return a modifiable reference to a CCoins. If no entry with the given
     * txid exists, a new one is created. Simultaneous modifications are not
     * allowed.
     */

    //For now omit this, since we omit transaction check
    pub fn modify_coins(&mut self, txid: FrHash) -> Option<CoinsModifier> {
        //assert();
        if !self.cache_coins.contains_key(&txid) {
            self.cache_coins.insert(txid, CoinsCacheEntry::new());
        }
        let entry = self.cache_coins.get_mut(&txid);

        /*if !entry.is_none() {
            entry.unwrap().dirty = true;
        }*/

        entry.and_then(|mut e| {
            e.dirty = true;
            Some(CoinsModifier::new(e))
        })
    }

    pub fn get_coins() -> Option<Coins> {
        unimplemented!()
    }

    pub fn have_inputs(&self, tx: &Transaction) -> bool {
        if !tx.is_coin_base() {
            for txin in tx.vin.iter() {
                let prevout = &txin.prevout;
                let coins = self.access_coins(prevout.hash);

                if !coins.is_none() && coins.unwrap().is_available(prevout.n) {
                    return false;
                }
            }
        }
        true
    }

    //Code include fetch_coins
    //TODO, implement DB action
    pub fn access_coins(&self, txid: FrHash) -> Option<&Coins> {
        let entry = self.cache_coins.get(&txid);
        entry.map(|e| &e.coins)
    }

    //pub fn fetch_coins(txid: FrHash)

    //Check if all sapling spend requirement(anchors/nullifiers) are satisfied.
    pub fn have_shield_requirements(&mut self, tx: &Transaction) -> bool {
        for spend_description in tx.v_shielded_spend.iter() {
            if self.get_nullifier(U256::from(spend_description.nullifier)) {
                return false;
            }
            let tree = self.get_sapling_anchor_at(FrHash(spend_description.anchor));
            if tree.is_none() {
                return false;
            }
        }
        true
    }
}

impl CoinsView for CoinViewCache {
    fn get_best_anchor(&self) -> Option<FrHash> {
        self.hash_sapling_anchor
    }

    fn get_best_block(&self) -> U256 {
        self.hash_block
    }

    //bool CCoinsViewCache::GetSaplingAnchorAt(const uint256 &rt, SaplingMerkleTree &tree) const {
    fn get_sapling_anchor_at(&mut self, rt: FrHash) -> Option<SaplingMerkleTree> {
        let res = self.cached_sapling_anchors.get(&rt);
        match res {
            None => {
                return None;
            }
            Some(ref entry) => {
                if entry.entered {
                    return Some(entry.tree.clone());
                } else {
                    return None;
                }
            }
        }

        //TODO, implement DB
        /*let tree = self.base.get_sapling_anchor_at(rt.clone());
        let tree_clone = tree.clone();
        match tree_clone {
            None => {
                return None;
            }
            Some(t) => {
                let entry = AnchorsSaplingCacheEntry::new(t);
                let rt_clone = rt.clone();
                self.cached_sapling_anchors.insert(rt_clone, entry);
                tree
            }
        }*/
    }

    fn get_nullifier(&mut self, nullifier: U256) -> bool {
        let entry = self.cached_sapling_nullifiers.get(&nullifier);
        let res = entry.map(|e| e.entered);
        if res.is_none() {
            return false;
        } else {
            return res.unwrap();
        }
    }

    fn set_best_block(&mut self, block_hash: U256) {}

    fn have_coins(&self, txid: FrHash) -> bool {
        self.cache_coins.contains_key(&txid)
    }

    /*fn fetch_coins(&self, txid: U256) {
        self.cache_coins.
    }*/
}
