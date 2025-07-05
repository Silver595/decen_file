use sha2::{Sha256, Digest};
use std::fs::File;
use std::io::{Read, Write};

pub const CHUNK_SIZE: usize = 1024 * 1024; // 1 MB

pub fn chunk_file(file_path: &str) -> Vec<String> {
    let mut file = File::open(file_path).expect("File not found");
    let mut buffer = [0u8; CHUNK_SIZE];
    let mut chunk_hashes = Vec::new();

    let mut i = 0;
    loop {
        let n = file.read(&mut buffer).unwrap();
        if n == 0 { break; }

        let chunk = &buffer[..n];
        let hash = Sha256::digest(chunk);
        let hash_hex = hex::encode(&hash);
        chunk_hashes.push(hash_hex.clone());

        let mut chunk_file = File::create(format!("chunks/{}", &hash_hex)).unwrap();
        chunk_file.write_all(chunk).unwrap();
        println!("Chunk {} saved with hash {}", i, &hash_hex);
        i += 1;
    }

    chunk_hashes
}
