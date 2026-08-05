use std::{env, fs, process};

use csdlc_v2::TerminalReceipt;

fn main() {
    let path = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("terminal receipt path is required");
        process::exit(2);
    });
    let bytes = fs::read(&path).unwrap_or_else(|error| {
        eprintln!("cannot read terminal receipt: {error}");
        process::exit(2);
    });
    let receipt: TerminalReceipt = serde_json::from_slice(&bytes).unwrap_or_else(|error| {
        eprintln!("invalid typed terminal receipt: {error}");
        process::exit(2);
    });
    let expected = receipt.digest.clone();
    let mut unsigned = receipt;
    unsigned.digest.clear();
    let canonical = serde_json::to_vec(&unsigned).expect("typed receipt serializes");
    let actual = blake3::hash(&canonical).to_hex().to_string();
    if expected != actual {
        eprintln!("terminal receipt BLAKE3 digest mismatch");
        process::exit(2);
    }
    println!("{{\"schema\":\"adl.wp13.terminal_receipt_digest.v1\",\"status\":\"pass\"}}");
}
