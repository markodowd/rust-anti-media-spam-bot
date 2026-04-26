use sha2::{Digest, Sha256};
use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let path = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: add_hash <image_file>");
        std::process::exit(1);
    });

    let bytes = fs::read(&path).unwrap_or_else(|e| {
        eprintln!("Failed to read '{}': {}", path, e);
        std::process::exit(1);
    });

    let hash = hex::encode(Sha256::digest(&bytes));

    let hash_file = "BLACKLISTED_HASHES.txt";
    let existing = fs::read_to_string(hash_file).unwrap_or_default();
    let known: HashSet<&str> = existing.lines().map(str::trim).collect();

    if known.contains(hash.as_str()) {
        println!("Hash already blacklisted: {}", hash);
        return;
    }

    let entry = if existing.ends_with('\n') || existing.is_empty() {
        format!("{}\n", hash)
    } else {
        format!("\n{}\n", hash)
    };

    fs::write(
        hash_file,
        format!("{}{}", existing, entry),
    )
    .unwrap_or_else(|e| {
        eprintln!("Failed to write '{}': {}", hash_file, e);
        std::process::exit(1);
    });

    println!("Added hash: {}", hash);
}
