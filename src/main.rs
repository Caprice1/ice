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
use crate::sendmany::Sender;
use crate::wallet::Wallet;

fn main() {
    sendmany::show();
    wallet::show();


    let wallet = Wallet::new();

    let sender = Sender{
        main_wallet: wallet,
    };

    let (tx, rx) = mpsc::channel();


    thread::spawn(move || {
        //Setup work queue
        for item in rx {
            println!("Received: {}", item);

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

