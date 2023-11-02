mod wallet_parser;

use std::fs;
use crate::wallet_parser::parse_wallet;

fn main() {
    let content = fs::read_to_string("/Users/mark.tkachenko/Projects/wallet-parser/wallet.txt").unwrap();
    parse_wallet(content);
}

