
use bigint::U256;

use crate::sendmany::{
    SpendDescriptionInfo, OutputDescriptionInfo
};

use crate::key::key_management::{
    SaplingIncomingViewingKey,
    SaplingExpandedSpendingKey,
    SaplingNote,
};

use crate::incremental_tree::tree::SaplingWitness;

pub struct TransactionBuilder {
    pub spends: Vec<SpendDescriptionInfo>,
    pub outputs: Vec<OutputDescriptionInfo>,
}


impl TransactionBuilder {
/*
void TransactionBuilder::AddSaplingSpend(
    libzcash::SaplingExpandedSpendingKey expsk,
    libzcash::SaplingNote note,
    uint256 anchor,
    SaplingWitness witness)
{
    // Sanity check: cannot add Sapling spend to pre-Sapling transaction
    if (mtx.nVersion < SAPLING_TX_VERSION) {
        throw std::runtime_error("TransactionBuilder cannot add Sapling spend to pre-Sapling transaction");
    }

    // Consistency check: all anchors must equal the first one
    if (spends.size() > 0 && spends[0].anchor != anchor) {
        throw JSONRPCError(RPC_WALLET_ERROR, "Anchor does not match previously-added Sapling spends.");
    }

    spends.emplace_back(expsk, note, anchor, witness);
    mtx.valueBalance += note.value();
}
*/
    pub fn new() -> Self {
        TransactionBuilder {
            spends: Vec::new(),
            outputs: Vec::new(),
        }
    }

    pub fn add_sapling_spend(&self,
                             expsk: SaplingExpandedSpendingKey,
                             note:  SaplingNote,
                             anchor: U256,
                             witness: Box<SaplingWitness>,
    ) {

    }
}