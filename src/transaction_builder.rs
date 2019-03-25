use bigint::U256;

use crate::sendmany::{CAmount, OutputDescriptionInfo, SpendDescriptionInfo};

use crate::key::key_management::{
    FrHash, SaplingExpandedSpendingKey, SaplingIncomingViewingKey, SaplingNote,
    SaplingOutgoingViewingKey, SaplingPaymentAddress,
};

use crate::key::key_store::TxDestination;

use crate::incremental_tree::tree::SaplingWitness;

use crate::wallet::Wallet;

pub struct TransactionBuilder<'a> {
    pub spends: Vec<SpendDescriptionInfo>,
    pub outputs: Vec<OutputDescriptionInfo>,
    pub wallet: &'a Wallet<'a>,
    pub next_block_height: i32,
}

impl<'a> TransactionBuilder<'a> {
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
    pub fn new(height: i32, wallet: &'a Wallet) -> Self {
        TransactionBuilder {
            spends: Vec::new(),
            outputs: Vec::new(),
            next_block_height: height,
            wallet: wallet,
        }
    }

    pub fn add_sapling_spend(
        &self,
        expsk: &SaplingExpandedSpendingKey,
        note: &SaplingNote,
        anchor: FrHash,
        witness: &&SaplingWitness,
    ) {

    }

    //void TransactionBuilder::AddSaplingOutput(
    //    uint256 ovk,
    //    libzcash::SaplingPaymentAddress to,
    //    CAmount value,
    //    std::array<unsigned char, ZC_MEMO_SIZE> memo)
    //{

    pub fn add_sapling_output(
        &self,
        ovk: &SaplingOutgoingViewingKey,
        to: SaplingPaymentAddress,
        value: &CAmount,
        meme: &String,
    ) {

    }

    //AddTransparentOutput
    pub fn add_transparent_output(&self, address: TxDestination, amount: &CAmount) {}

    pub fn build(&self) {}
}
