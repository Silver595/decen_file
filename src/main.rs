mod chunker;

use std::fs;

fn main() {
    fs::create_dir_all("chunks").unwrap();
    let file_path = "./sample.pdf"; 
    let hashes = chunker::chunk_file(file_path);

    println!("\nChunk hashes:");
    for h in hashes {
        println!("{}", h);
    }
}
