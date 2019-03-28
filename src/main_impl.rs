
//Functions and Operation that related to chain operation

use crate::block_chain::{Block, BlockIndex, ValidationState};
use crate::coins::CoinViewCache;

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
pub fn active_best_chain() {

}

/**
 * Try to make some progress towards making pindexMostWork the active block.
 * pblock is either NULL or a pointer to a CBlock corresponding to pindexMostWork.
 */
//static bool ActivateBestChainStep(CValidationState &state, CBlockIndex *pindexMostWork, CBlock *pblock) {
pub fn active_best_chain_step() {
    //call connect_tip
}


//bool static ConnectTip(CValidationState &state, CBlockIndex *pindexNew, CBlock *pblock)
pub fn connect_tip() {
    //call connect_block
    //call wallet.chain_tip
}

/**
 * Disconnect chainActive's tip. You probably want to call mempool.removeForReorg and
 * mempool.removeWithoutBranchId after this, with cs_main held.
 */
//bool static DisconnectTip(CValidationState &state, bool fBare = false) {
pub fn disconnect_tip() {

}



//bool ConnectBlock(const CBlock& block, CValidationState& state,
// CBlockIndex* pindex, CCoinsViewCache& view, bool fJustCheck)

pub fn connect_block(
    block: Block,
    state: ValidationState,
    pindex: BlockIndex,
    view: CoinViewCache,
    f_just_check: bool,
) {

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
