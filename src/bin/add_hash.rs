use img_hash::{HasherConfig, HashAlg};
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

    let img = image::load_from_memory(&bytes).unwrap_or_else(|e| {
        eprintln!("Failed to decode image '{}': {}", path, e);
        std::process::exit(1);
    });

    let hasher = HasherConfig::new()
        .hash_alg(HashAlg::Gradient)
        .to_hasher();

    let hash = hasher.hash_image(&img);
    println!("pHash: {}", hex::encode(hash.as_bytes()));
}
