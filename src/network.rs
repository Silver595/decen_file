use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::fs;

pub fn start_server(port: u16) {
    let listener = TcpListener::bind(("0.0.0.0", port)).expect("Failed to bind");

    println!("[+] Listening on port {}", port);

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut hash_buf = [0u8; 64]; // SHA256 hex length
                stream.read(&mut hash_buf).unwrap();
                let hash_str = String::from_utf8_lossy(&hash_buf).trim_matches(char::from(0)).to_string();

                println!("[>] Client requested chunk: {}", hash_str);

                let chunk_path = format!("chunks/{}", hash_str);
                if let Ok(chunk) = fs::read(&chunk_path) {
                    stream.write_all(&chunk).unwrap();
                    println!("[+] Sent chunk {}", hash_str);
                } else {
                    stream.write_all(b"NOT_FOUND").unwrap();
                    println!("[-] Chunk not found: {}", hash_str);
                }
            }
            Err(e) => {
                println!("Connection failed: {}", e);
            }
        }
    }
}

pub fn request_chunk(peer_ip: &str, port: u16, hash: &str) {
    let addr = format!("{}:{}", peer_ip, port);
    let mut stream = TcpStream::connect(addr).expect("Failed to connect to peer");

    // Send hash
    stream.write_all(hash.as_bytes()).unwrap();

    let mut buffer = vec![0; 1024 * 1024]; // 1MB max
    let n = stream.read(&mut buffer).unwrap();

    if &buffer[..9] == b"NOT_FOUND" {
        println!("[-] Peer does not have this chunk.");
        return;
    }

    let chunk_path = format!("chunks/{}", hash);
    fs::write(chunk_path, &buffer[..n]).unwrap();
    println!("[+] Chunk {} received and saved.", hash);
}
