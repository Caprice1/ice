pub const ZC_MEMO_SIZE: usize = 512;

pub const COINBASE_MATURITY: usize = 100;
pub const MAX_REORG_LENGTH: usize = COINBASE_MATURITY - 1;
pub const WITNESS_CACHE_SIZE: usize = MAX_REORG_LENGTH + 1;
