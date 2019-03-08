mod sendmany;
mod transaction;
mod transaction_builder;
mod wallet;
mod incremental_tree;
mod my;
mod key;


use std::thread;
use std::sync::mpsc;
use std::io;
//use std::vec::Vec;

use crate::sendmany::SendMany;


use crate::wallet::Wallet;
use crate::key::address::AddressManagement;


fn sendmany(params: Vec<String>) {




}

fn main() {
    sendmany::show();
    wallet::show();


    let wallet = Wallet::new();
    let address_management = AddressManagement::new();

    let sender = SendMany {
        main_wallet: wallet,
        address_management: address_management,
    };

    let (tx, rx) = mpsc::channel();


    thread::spawn(move || {
        //Setup work queue
        for item in rx {
            println!("Received: {}", item);
            let s = item as String;
            let params = s.split_whitespace().map(|s| s.to_string()).collect::<Vec<_>>();

            sendmany(params);
        }
        println!("Work queue thread end");
    });


    while true {
        //Take user action(sendTransaction etc)
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                println!("{} bytes read", n);
                println!("{}", input);

                tx.send(input).unwrap();
            }
            Err(error) => println!("error: {}", error),
        }
    }


    println!("Start success");


}

#[cfg(test)]
mod test {

}

