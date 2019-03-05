mod sendmany;
mod transaction;
mod wallet;
mod sapling_witness;
mod my;


use std::thread;
use std::sync::mpsc;
use std::io;
use sendmany::Sender;
use wallet::Wallet;

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

