use crate::block_chain::{Block, BlockIndex, ValidationState};
use crate::coins::CoinViewCache;

//bool ReadBlockFromDisk(CBlock& block, const CBlockIndex* pindex)

pub fn read_block_from_disk(pindex: &BlockIndex) -> Option<Block> {
    None
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
}

//bool AcceptBlock(CBlock& block, CValidationState& state,
// CBlockIndex** ppindex, bool fRequested, CDiskBlockPos* dbp)

pub fn accept_block() {
    accept_block_header()
}

//bool AcceptBlockHeader(const CBlockHeader& block,
// CValidationState& state, CBlockIndex** ppindex)

pub fn accept_block_header() {}
