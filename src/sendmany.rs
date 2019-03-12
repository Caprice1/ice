


extern crate zip32;
extern crate pairing;



use pairing::{
    bls12_381::{Bls12, Fr, FrRepr},
};

use sapling_crypto::{
    primitives::{Note, PaymentAddress},
};

use bigint::U256;

use std::collections::LinkedList;
use std::cmp::Eq;
use std::hash::{Hash, Hasher};

use crate::my::constants::ZC_MEMO_SIZE;
use crate::wallet::Wallet;
use crate::incremental_tree::tree::{
    SaplingWitness,
};

use crate::key::key_management::{
    SaplingIncomingViewingKey,
    SaplingExpandedSpendingKey,
    SaplingExtendedSpendingKey,
    SaplingNote,
};

use crate::transaction_builder::TransactionBuilder;
use crate::other::sanity_check::SanityChecker;
use crate::key::key_store::KeyStore;

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
pub struct SaplingOutPoint
{
    pub hash: U256,
    pub n: u32,
}


pub struct SaplingNoteData
{
    /*
    std::list<SaplingWitness> witnesses;
    int witnessHeight;
    libzcash::SaplingIncomingViewingKey ivk;
    boost::optional<uint256> nullifier;
    */

    pub witnesses: LinkedList<Box<SaplingWitness>>,

    pub witnessHeight: u64,

    pub ivk: SaplingIncomingViewingKey,

    pub nullifier: Option<U256>,
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

pub struct SendMany {
    pub main_wallet: Wallet,
    //pub address_management: AddressManagement,
    pub key_store: KeyStore,
    pub sanity_checker: SanityChecker,
}


impl SendMany {

    pub fn pre_send_many(&self, params:Vec<String>) {
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
        let (payment_address, spending_key_option)
            = self.key_store.decode_z_destination(fromaddress);


        /*let outputs_str = params[2].split(",").collect::<Vec<_>>();
        if outputs_str.len() == 0 {
            panic!("No outputs");
        }*/

        let outputs_str = &params[1];
        let (zaddrRecipients, taddrRecipients, total_amount)
            = self.key_store.decode_outputs(outputs_str);

        self.sanity_checker.check_transaction_size(&zaddrRecipients);

        let nMinDepth = self.sanity_checker.get_check_mindepth(&params);

        let nFee = self.sanity_checker.get_check_fee(&params);

        //let context_info = vec![];

        //int nextBlockHeight = chainActive.Height() + 1;
        //TODO
        let next_block_height = 0;

        let builder =
            TransactionBuilder::new(next_block_height, &self.main_wallet);

        //let spending_key_op =
        //    self.key_store.get_sapling_extended_spending_key(spending_key_option);
        let expsk: Option<SaplingExpandedSpendingKey> = spending_key_option.and_then(|spending_key: SaplingExtendedSpendingKey | Some(spending_key.expsk));

        let sendmany_operation
            = SendManyOperation::new(
                &builder, fromaddress.clone(), taddrRecipients, zaddrRecipients,
                nMinDepth, nFee, expsk.unwrap());

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
    transaction_builder_: &'a TransactionBuilder<'a>,

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

    fn new(builder: &'a TransactionBuilder,
           /*contextualTx: MutableTransaction,*/
           fromaddress: String,
           t_outputs: Vec<SendManyRecipient>,
           z_outputs: Vec<SendManyRecipient>,
           min_depth: i32,
           fee: CAmount,
           spendingkey_: SaplingExpandedSpendingKey,
           /*contextInfo: */) -> Self {
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

        //let sk = self.spendingkey_;
        //let expsk = sk;


        let wallet = &self.transaction_builder_.wallet;
        let mut target_amount = 100;
        let result = self.z_inputs_.iter().try_fold((vec![], vec![], 0),
                                                |(mut a, mut b, s), t|
                                                    if s < target_amount {
                                                        a.push(&t.op);
                                                        b.push(&t.note);
                                                        Some((a, b, s+t.note.value))
                                                    }
                                                        else {None});
        assert_eq!(result.is_none(), false);

        let (ops, notes, _) = result.unwrap();

        let (witnesses, anchor)
            = wallet.get_sapling_note_witnesses(&ops);

        for (i, witness_op) in witnesses.iter().enumerate() {
            match witness_op {
                None => { panic!("No witness found");  }
                Some(witness) => {
                    self.transaction_builder_.add_sapling_spend(
                        self.spendingkey_, notes[i], anchor.unwrap(), witness
                    );
                }
            }
        }


        println!("In sendmany");
    }
}






