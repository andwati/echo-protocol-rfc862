use env_logger;
use log::{error, info};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let peer_addr = stream.peer_addr().unwrap();
    info!("Connected to client at {}", peer_addr);

    let mut buffer = [0; 1024];
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                info!("Client at {} closed the connection", peer_addr);
                break; // End-of-file: client finished sending data
            }
            Ok(n) => {
                if let Err(e) = stream.write_all(&buffer[..n]) {
                    error!("Failed to send data back to {}: {}", peer_addr, e);
                    break;
                }
                info!("Echoed {} bytes to {}", n, peer_addr);
            }
            Err(e) => {
                error!("Error reading from {}: {}", peer_addr, e);
                break;
            }
        }
    }
    info!("Closing connection with client at {}", peer_addr);
}

fn main() -> std::io::Result<()> {
    env_logger::init();

    let listener = TcpListener::bind("127.0.0.1:3000")?;
    info!("TCP Echo server listening on 127.0.0.1:3000");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                error!("Failed to accept a connection: {}", e);
            }
        }
    }
    Ok(())
}
