use std::{env, fs::File, process::exit};

use btclib::{types::Block, util::Saveable};

fn main() {
    let path = if let Some(arg) = env::args().nth(1) {
        arg
    } else {
        eprintln!("Usage: block_print <block_file>");
        exit(1);
    };

    if let Ok(file) = File::open(path) {
        let block = Block::load(file).expect("Failed to load blcok");
        println!("{:#?}", block);
    }
}
