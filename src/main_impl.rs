//Functions and Operation that related to chain operation

use crate::block_chain::{Block, BlockIndex, ValidationState};
use crate::coins::{CoinViewCache, CoinsView};
use crate::key::key_management::FrHash;
use crate::key::proof::ProofVerifier;
use crate::wallet::Wallet;
use crate::transaction::Transaction;

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
pub fn connect_tip(pcoins_tip: &mut CoinViewCache, wallet: &mut Wallet,  state: &ValidationState,
                pindex_new: &BlockIndex, pblock: &Block) {
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
pub fn disconnect_tip() {}

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
        for output  in tx.v_shielded_output.iter() {
            sapling_tree.append(FrHash(output.cmu));
        }
    }

    view.push_anchor(sapling_tree);

    view.set_best_block(pindex.get_block_hash());

}

pub fn check_block(block: &Block, state: &ValidationState, verifier: &ProofVerifier,
    f_check_POW: bool, f_check_merkle_root: bool) -> bool {
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
