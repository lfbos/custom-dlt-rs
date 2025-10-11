use std::{env, process::exit};

use btclib::{
    crypto::PrivateKey,
    types::{Transaction, TransactionOutput},
    util::Saveable,
};
use uuid::Uuid;

fn main() {
    let path = if let Some(arg) = env::args().nth(1) {
        arg
    } else {
        eprintln!("Usage: block_print <tx_file>");
        exit(1);
    };
    let private_key = PrivateKey::new_key();
    let tx = Transaction::new(
        vec![],
        vec![TransactionOutput {
            unique_id: Uuid::new_v4(),
            value: btclib::INITIAL_REWARD * 10u64.pow(8),
            pubkey: private_key.public_key(),
        }],
    );
    tx.save_to_file(path).expect("Failed to save transaction");
}
