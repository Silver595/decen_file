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
mod chunker;
mod network;

use std::env;
use std::fs;

fn main() {
    fs::create_dir_all("chunks").unwrap();

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage:\n  server <port>\n  client <ip> <port> <hash>");
        return;
    }

    match args[1].as_str() {
        "server" => {
            let port: u16 = args[2].parse().unwrap();
            network::start_server(port);
        }
        "client" => {
            let ip = &args[2];
            let port: u16 = args[3].parse().unwrap();
            let hash = &args[4];
            network::request_chunk(ip, port, hash);
        }
        _ => {
            println!("Invalid mode.");
        }
    }
}
