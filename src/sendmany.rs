extern crate pairing;
extern crate zip32;

use pairing::bls12_381::{Bls12, Fr, FrRepr};

use ff::PrimeField;

use sapling_crypto::primitives::{Note, PaymentAddress};

use bigint::U256;

use std::cmp::Eq;
use std::collections::LinkedList;
use std::hash::{Hash, Hasher};

use crate::incremental_tree::tree::SaplingWitness;
use crate::my::constants::ZC_MEMO_SIZE;
use crate::wallet::Wallet;

use crate::key::key_management::{
    FrHash, SaplingExpandedSpendingKey, SaplingExtendedSpendingKey, SaplingIncomingViewingKey,
    SaplingNote,
};

use crate::key::key_store::{KeyStore, TxDestination};
use crate::other::sanity_check::SanityChecker;
use crate::transaction_builder::TransactionBuilder;

use crate::key::key_store::{decode_destination, decode_payment_address};

//static mut pMainWallet: Wallet = Wallet::new();

//SaplingNote

pub struct SpendDescriptionInfo {
    pub expsk: SaplingExpandedSpendingKey,
    pub note: SaplingNote,
    pub alpha: Fr,
    pub ahchor: Fr,
}

pub struct OutputDescriptionInfo {
    pub ovk: U256,
    pub note: SaplingNote,
    pub memo: [char; ZC_MEMO_SIZE],
}

#[derive(PartialEq, Eq, Hash)]
pub struct SaplingOutPoint {
    pub hash: FrHash, //U256,
    pub n: u32,
}

pub struct SaplingNoteData {
    /*
    std::list<SaplingWitness> witnesses;
    int witnessHeight;
    libzcash::SaplingIncomingViewingKey ivk;
    boost::optional<uint256> nullifier;
    */
    pub witnesses: LinkedList<SaplingWitness>,

    pub witnessHeight: i32,

    pub ivk: SaplingIncomingViewingKey,

    pub nullifier: Option<U256>,
}

impl SaplingNoteData {
    pub fn push_front(&mut self, witness: SaplingWitness) {
        self.witnesses.push_front(witness);
    }

    pub fn pop_back(&mut self) {
        self.witnesses.pop_back();
    }

    pub fn front(&self) -> Option<SaplingWitness> {
        None
    }
}

pub struct SaplingNoteEntry {
    op: SaplingOutPoint,
    //PaymentAddress not sure is Bls12
    address: PaymentAddress<Bls12>,
    note: Note<Bls12>,
    memo: [char; ZC_MEMO_SIZE],
    confirmation: i32,
}

pub type CAmount = u64;

pub type SendManyRecipient = (String, CAmount, String);

pub fn show() {
    println!("sendmany show");

    let v = [1, 2, 3, 4, 5];
}

pub struct SendMany<'a> {
    pub main_wallet: &'a Wallet,
    //pub address_management: AddressManagement,
    pub key_store: KeyStore,
    pub sanity_checker: SanityChecker,
}

impl<'a> SendMany<'a> {
    pub fn pre_send_many(&self, params: Vec<String>) {
        println!("In sendmany");

        let len = params.len();

        //if (fHelp || params.size() < 2 || params.size() > 4)
        if len < 3 || len > 5 {
            panic!("Please check you input");
        }

        if params[0] == "sendmany" {
            for param in &params {
                print!("{} ", param);
            }
            println!("");
        } else {
            println!("Not sendmany");
            return;
        }

        let params = &params[1..];
        let fromaddress = &params[0];
        let is_tx = self.key_store.decode_transparent_destination(fromaddress);
        let (payment_address, spending_key_option) =
            self.key_store.decode_z_destination(fromaddress);

        /*let outputs_str = params[2].split(",").collect::<Vec<_>>();
        if outputs_str.len() == 0 {
            panic!("No outputs");
        }*/

        let outputs_str = &params[1];
        let (zaddrRecipients, taddrRecipients, total_amount) =
            self.key_store.decode_outputs(outputs_str);

        self.sanity_checker.check_transaction_size(&zaddrRecipients);

        let nMinDepth = self.sanity_checker.get_check_mindepth(&params);

        let nFee = self.sanity_checker.get_check_fee(&params);

        //let context_info = vec![];

        //int nextBlockHeight = chainActive.Height() + 1;
        //TODO
        let next_block_height = 0;

        let builder = TransactionBuilder::new(next_block_height, self.main_wallet);

        //let spending_key_op =
        //    self.key_store.get_sapling_extended_spending_key(spending_key_option);
        let expsk: Option<SaplingExpandedSpendingKey> = spending_key_option
            .and_then(|spending_key: SaplingExtendedSpendingKey| Some(spending_key.expsk));

        let sendmany_operation = SendManyOperation::new(
            builder,
            fromaddress.clone(),
            taddrRecipients,
            zaddrRecipients,
            nMinDepth,
            nFee,
            expsk.unwrap(),
        );

        sendmany_operation.main_impl();
    }
}

pub struct SendManyOperation<'a> {
    /*
    std::vector<SendManyRecipient> t_outputs_;
    std::vector<SendManyRecipient> z_outputs_;
    std::vector<SendManyInputUTXO> t_inputs_;
    std::vector<SendManyInputJSOP> z_sprout_inputs_;
    std::vector<SaplingNoteEntry> z_sapling_inputs_;

    CAmount fee_;
    int mindepth_;
    std::string fromaddress_;
    bool isfromtaddr_;
    bool isfromzaddr_;
    */
    z_inputs_: Vec<SaplingNoteEntry>,
    t_outputs_: Vec<SendManyRecipient>,
    z_outputs_: Vec<SendManyRecipient>,
    transaction_builder_: TransactionBuilder<'a>,

    spendingkey_: SaplingExpandedSpendingKey,

    fee_: CAmount,
    mindepth: i32,
    fromaddress_: String,
}

impl<'a> SendManyOperation<'a> {
    //std::shared_ptr<AsyncRPCOperation>
    // operation( new AsyncRPCOperation_sendmany
    // (builder, contextualTx, fromaddress, taddrRecipients,
    // zaddrRecipients, nMinDepth, nFee, contextInfo) );
    //

    //AsyncRPCOperation_sendmany(
    //        boost::optional<TransactionBuilder> builder,
    //        CMutableTransaction contextualTx,
    //        std::string fromAddress,
    //        std::vector<SendManyRecipient> tOutputs,
    //        std::vector<SendManyRecipient> zOutputs,
    //        int minDepth,
    //        CAmount fee = ASYNC_RPC_OPERATION_DEFAULT_MINERS_FEE,
    //        UniValue contextInfo = NullUniValue);

    fn new(
        builder: TransactionBuilder<'a>,
        /*contextualTx: MutableTransaction,*/
        fromaddress: String,
        t_outputs: Vec<SendManyRecipient>,
        z_outputs: Vec<SendManyRecipient>,
        min_depth: i32,
        fee: CAmount,
        spendingkey_: SaplingExpandedSpendingKey,
        /*contextInfo: */
    ) -> Self {
        SendManyOperation {
            transaction_builder_: builder,
            fromaddress_: fromaddress,
            t_outputs_: t_outputs,
            z_outputs_: z_outputs,
            mindepth: min_depth,
            fee_: fee,

            spendingkey_: spendingkey_,

            z_inputs_: Vec::new(),
        }
    }

    pub fn main_impl(&self) {
        let wallet = &self.transaction_builder_.wallet;
        let mut target_amount = 100;
        let result = self
            .z_inputs_
            .iter()
            .try_fold((vec![], vec![], 0), |(mut a, mut b, s), t| {
                if s < target_amount {
                    a.push(&t.op);
                    b.push(&t.note);
                    Some((a, b, s + t.note.value))
                } else {
                    None
                }
            });
        assert_eq!(result.is_none(), false);

        let (ops, notes, _) = result.unwrap();

        let (witnesses, anchor) = wallet.get_sapling_note_witnesses(ops);

        //for witness_op in witnesses {
        for (i, witness_op) in witnesses.iter().enumerate() {
            match witness_op {
                None => {
                    panic!("No witness found");
                }
                Some(witness) => {
                    let t_ancher = anchor.clone();
                    self.transaction_builder_.add_sapling_spend(
                        &self.spendingkey_,
                        notes[i],
                        t_ancher.unwrap(),
                        witness,
                    );
                }
            }
        }

        //// Add Sapling outputs
        //        for (auto r : z_outputs_) {
        //            auto address = std::get<0>(r);
        //            auto value = std::get<1>(r);
        //            auto hexMemo = std::get<2>(r);
        //
        //            auto addr = DecodePaymentAddress(address);
        //            assert(boost::get<libzcash::SaplingPaymentAddress>(&addr) != nullptr);
        //            auto to = boost::get<libzcash::SaplingPaymentAddress>(addr);
        //
        //            auto memo = get_memo_from_hex_string(hexMemo);
        //
        //            builder_.AddSaplingOutput(ovk, to, value, memo);
        //        }

        let ovk = &self.spendingkey_.ovk;

        for (address, value, memo) in self.z_outputs_.iter() {
            let to = decode_payment_address(address);
            self.transaction_builder_
                .add_sapling_output(ovk, to.unwrap(), value, memo);
        }

        //// Add transparent outputs
        //        for (auto r : t_outputs_) {
        //            auto outputAddress = std::get<0>(r);
        //            auto amount = std::get<1>(r);
        //
        //            auto address = DecodeDestination(outputAddress);
        //            builder_.AddTransparentOutput(address, amount);
        //        }
        for (address, amount, memo) in self.t_outputs_.iter() {
            let addr = decode_destination(address);
            self.transaction_builder_
                .add_transparent_output(addr.unwrap(), amount);
        }

        self.transaction_builder_.build();

        //TODO, Send out transaction

        println!("In sendmany");
    }
}
