//Functions and Operation that related to chain operation

use crate::block_chain::{
    Block, BlockIndex, BlockUndo, Chain, DiskBlockPos, TxInUndo, ValidationState,
};
use crate::coins::{CoinViewCache, Coins, CoinsView};
use crate::key::key_management::FrHash;
use crate::key::proof::ProofVerifier;
use crate::sendmany::SaplingOutPoint;
use crate::transaction::Transaction;
use crate::wallet::Wallet;
use ethereum_types::U256;
//use rustzcash::{librustzcash_sapling_check_spend, librustzcash_sapling_verification_ctx_init};
use crate::txmempool::{TxMemPool, TxMemPoolEntry};
use std::collections::hash_set::HashSet;

//bool ReadBlockFromDisk(CBlock& block, const CBlockIndex* pindex)

pub fn read_block_from_disk(pindex: &BlockIndex) -> Option<Block> {
    None
}

/**
 * Make the best chain active, in multiple steps. The result is either failure
 * or an activated best chain. pblock is either NULL or a pointer to a block
 * that is already loaded (to avoid loading it again from disk).
 */
//bool ActivateBestChain(CValidationState &state, CBlock *pblock) {
pub fn active_best_chain() {}

/**
 * Try to make some progress towards making pindexMostWork the active block.
 * pblock is either NULL or a pointer to a CBlock corresponding to pindexMostWork.
 */
//static bool ActivateBestChainStep(CValidationState &state, CBlockIndex *pindexMostWork, CBlock *pblock) {
pub fn active_best_chain_step() {
    //call connect_tip
}

//bool static ConnectTip(CValidationState &state, CBlockIndex *pindexNew, CBlock *pblock)
pub fn connect_tip(
    pcoins_tip: &mut CoinViewCache,
    wallet: &mut Wallet,
    state: &ValidationState,
    pindex_new: &BlockIndex,
    pblock: &Block,
) {
    //call connect_block
    //call wallet.chain_tip

    //SproutMerkleTree oldSproutTree;
    //SaplingMerkleTree oldSaplingTree;
    //assert(pcoinsTip->GetSproutAnchorAt(pcoinsTip->GetBestAnchor(SPROUT), oldSproutTree));
    //assert(pcoinsTip->GetSaplingAnchorAt(pcoinsTip->GetBestAnchor(SAPLING), oldSaplingTree));
    //let old_sapling_tree
    //    = pcoins_tip.get_sapling_anchor_at(pcoins_tip.get_best_anchor());
    let old_sapling_tree = pcoins_tip
        .get_best_anchor()
        .and_then(|anchor| pcoins_tip.get_sapling_anchor_at(anchor));
    connect_block(pblock, state, pindex_new, pcoins_tip, false);

    wallet.chain_tip(pindex_new, pblock, &mut old_sapling_tree.unwrap(), true);
}

/**
 * Disconnect chainActive's tip. You probably want to call mempool.removeForReorg and
 * mempool.removeWithoutBranchId after this, with cs_main held.
 */
//bool static DisconnectTip(CValidationState &state, bool fBare = false) {
pub fn disconnect_tip(
    chain_active: &Chain,
    pcoins_tip: &mut CoinViewCache,
    state: &ValidationState,
    f_bare: bool,
) {
    let pindex_delete = chain_active.tip();
    let block = pindex_delete.and_then(|pindex| read_block_from_disk(pindex));
    let sapling_anchor_before_disconnect = pcoins_tip.get_best_anchor();
    if !disconnect_block(&block.unwrap(), state, &pindex_delete.unwrap(), pcoins_tip) {
        //Report error here
        return;
    }
}

//TODO
pub fn disconnect_block(
    block: &Block,
    state: &ValidationState,
    pindex: &BlockIndex,
    view: &mut CoinViewCache,
) -> bool {
    assert!(pindex.get_block_hash() == view.get_best_block());
    let mut f_clean = true;

    let pos = pindex.get_undo_pos();
    let hash = pindex.pprev.as_ref().unwrap().get_block_hash();
    let block_undo = undo_read_from_disk(pos, hash);

    assert!(block_undo.vtxundo.len() + 1 == block.vtx.len());

    for i in (0..block.vtx.len()).rev() {
        let tx = &block.vtx[i];
        let hash = tx.hash;

        {
            let outs = view.modify_coins(hash);
            //outs.and_then(|outs| {outs.clear_unspendable(); None});
            if !outs.is_none() {
                let mut outs = outs.unwrap();
                outs.clear_unspendable();
                outs.clear();
            }

            //let outs_block = Coins::new();
            //outs.and_then(|outs| {outs.clear(); None});
        }

        view.set_nullifiers(&tx, false);

        //restore inputs
        if i > 0 {
            // not coinbases
            let tx_undo = &block_undo.vtxundo[i - 1];
            if tx_undo.vprevout.len() != tx.vin.len() {
                error!("DisconnectBlock(): transaction and undo data inconsistent");
            }
            for j in (0..tx.vin.len()).rev() {
                let out = &tx.vin[j].prevout;
                let undo = &tx_undo.vprevout[j];
                if !apply_tx_in_undo(undo, view, out) {
                    f_clean = false;
                }
            }
        }
    }

    view.pop_anchor(pindex.pprev.as_ref().unwrap().hash_final_sapling_root);

    view.set_best_block(pindex.pprev.as_ref().unwrap().get_block_hash());

    f_clean
}

pub fn apply_tx_in_undo(undo: &TxInUndo, view: &mut CoinViewCache, out: &SaplingOutPoint) -> bool {
    let mut f_clean = true;

    let mut coins = view.modify_coins(out.hash).unwrap();
    if undo.n_height != 0 {
        // undo data cointains height: this is the last output of the prevout tx being spent
        if !coins.is_pruned() {
            error!("undo data overwriting existing transaction");
            f_clean = false;
            return f_clean;
        }
        coins.clear();
        coins.entry.coins.f_coin_base = undo.f_coin_base;
        coins.entry.coins.n_height = undo.n_height;
    } else {
        if coins.is_pruned() {
            error!("undo data adding output to missing transaction");
            f_clean = false;
            return f_clean;
        }
    }
    if coins.entry.coins.is_available(out.n) {
        error!("undo data overwriting existing output");
        f_clean = false;
        return f_clean;
    }
    /*if coins.entry.coins.vout.len() < out.n+1 {

    }*/
    coins.entry.coins.vout[out.n] = undo.txout;
    f_clean
}

pub fn undo_read_from_disk(pos: DiskBlockPos, block_hash: U256) -> BlockUndo {
    unimplemented!()
}

//bool ConnectBlock(const CBlock& block, CValidationState& state,
// CBlockIndex* pindex, CCoinsViewCache& view, bool fJustCheck)

pub fn update_coins(tx: &Transaction, inputs: &mut CoinViewCache, height: i32) {
    /*if !tx.is_coin_base() {
        for txin in tx.vin {
        }
    }*/

    inputs.set_nullifiers(tx, true);

    //add outputs
}

pub fn connect_block(
    block: &Block,
    state: &ValidationState,
    pindex: &BlockIndex,
    view: &mut CoinViewCache,
    f_just_check: bool,
) {
    /*BOOST_FOREACH(const CTransaction& tx, block.vtx) {
        const CCoins* coins = view.AccessCoins(tx.GetHash());
        if (coins && !coins->IsPruned())
            return state.DoS(100, error("ConnectBlock(): tried to overwrite transaction"),
                             REJECT_INVALID, "bad-txns-BIP30");
    }*/
    let sapling_tree = view
        .get_best_anchor()
        .and_then(|anchor| view.get_sapling_anchor_at(anchor))
        .unwrap();

    for tx in block.vtx.iter() {
        update_coins(tx, view, pindex.nHeight);
        for output in tx.v_shielded_output.iter() {
            sapling_tree.append(FrHash(output.cmu));
        }
    }

    view.push_anchor(sapling_tree);

    view.set_best_block(pindex.get_block_hash());
}

pub fn check_block(
    block: &Block,
    state: &ValidationState,
    verifier: &ProofVerifier,
    f_check_POW: bool,
    f_check_merkle_root: bool,
) -> bool {
    if !check_block_header(block, state, f_check_POW) {
        return false;
    }

    true
}

pub fn check_block_header(block: &Block, state: &ValidationState, f_check_POW: bool) -> bool {
    true
}

//bool ProcessNewBlock(CValidationState &state, CNode* pfrom,
// CBlock* pblock, bool fForceProcessing, CDiskBlockPos *dbp)

pub fn process_new_block() {
    accept_block();
    //call active_best_chain
}

//bool AcceptBlock(CBlock& block, CValidationState& state,
// CBlockIndex** ppindex, bool fRequested, CDiskBlockPos* dbp)

pub fn accept_block() {
    accept_block_header()
}

//bool AcceptBlockHeader(const CBlockHeader& block,
// CValidationState& state, CBlockIndex** ppindex)

pub fn accept_block_header() {}

pub fn check_transaction_without_proof_verification(
    tx: &Transaction,
    state: &ValidationState,
) -> bool {
    let mut v_sapling_nullifiers = HashSet::new();
    for spend in tx.v_shielded_spend.iter() {
        if v_sapling_nullifiers.contains(&spend.nullifier) {
            return false;
        }
        v_sapling_nullifiers.insert(spend.nullifier);
    }
    if tx.is_coin_base() {
        if tx.v_shielded_spend.len() > 0 {
            return false;
        }
        if tx.v_shielded_output.len() > 0 {
            return false;
        }
    } else {

    }
    true
}

pub fn check_transaction(tx: &Transaction, state: &ValidationState) -> bool {
    if !check_transaction_without_proof_verification(tx, state) {
        return false;
    }
    true
}

//TODO wu xin
//check out SignatureHash implements
fn signature_hash() -> U256 {
    U256::from(0)
}

//TODO wu xin
//Notice: mainly check spend(check signature and zkProof), invoke method from librust
//SignatureHash should also be implement
pub fn contexual_check_transaction(tx: &Transaction, state: &ValidationState) -> bool {
    //uint256 dataToBeSigned;
    //
    //    if (!tx.vjoinsplit.empty() ||
    //        !tx.vShieldedSpend.empty() ||
    //        !tx.vShieldedOutput.empty())
    //    {
    //        auto consensusBranchId = CurrentEpochBranchId(nHeight, Params().GetConsensus());
    //        // Empty output script.
    //        CScript scriptCode;
    //        try {
    //            dataToBeSigned = SignatureHash(scriptCode, tx, NOT_AN_INPUT, SIGHASH_ALL, 0, consensusBranchId);
    //        } catch (std::logic_error ex) {
    //            return state.DoS(100, error("CheckTransaction(): error computing signature hash"),
    //                                REJECT_INVALID, "error-computing-signature-hash");
    //        }
    //    }
    if !tx.v_shielded_spend.is_empty() || !tx.v_shielded_output.is_empty() {
        /*let ctx = librustzcash_sapling_verification_ctx_init();


        for spend in tx.v_shielded_spend {
            if ctx.check_spend(
                spend.cv,
                spend.anchor,
                &spend.nullifier,
                spend.rk,
                spend.,
                spend.spend_auth_sig,
                spend.zkproof
            )
        }*/
    }
    true
}

fn contextual_check_inputs() -> bool {
    true
}

//TODO
fn check_final_tx() -> bool {
    true
}

pub fn accept_to_mem_pool<'a>(
    pool: &'a mut TxMemPool<'a>,
    state: &ValidationState,
    tx: &'a Transaction,
    pcoins_tip: &mut CoinViewCache,
) -> bool {
    if !check_transaction(tx, state) {
        return false;
    }
    if !contexual_check_transaction(tx, state) {
        return false;
    }
    // Coinbase is only valid in a block, not as a loose transaction
    if tx.is_coin_base() {
        return false;
    }
    if !check_final_tx() {
        return false;
    }

    let hash = tx.hash;
    if pool.exists(hash) {
        return false;
    }

    for txin in tx.vin.iter() {
        if pool.mapNextTx.contains_key(&txin.prevout) {
            //Disable replacement feature for now
            return false;
        }
    }

    for spend_description in tx.v_shielded_spend.iter() {
        if pool.nullifier_exists(U256::from(spend_description.nullifier)) {
            return false;
        }
    }

    {
        //TODO, backed view
        let mut view = pcoins_tip;

        if view.have_coins(hash) {
            return false;
        }

        for txin in tx.vin.iter() {
            if !view.have_coins(txin.prevout.hash) {
                //pf_missing_inputs
                return false;
            }
        }

        if !view.have_inputs(tx) {
            return false;
        }

        if !view.have_shield_requirements(tx) {
            return false;
        }

        let entry = TxMemPoolEntry::new(tx);
        //let entry_ptr: &'a TxMemPoolEntry = &entry;

        if !contextual_check_inputs() {
            return false;
        }

        //Different check, with different flags
        if !contextual_check_inputs() {
            return false;
        }

        pool.add_unchecked(hash, entry);
    }

    true
}
