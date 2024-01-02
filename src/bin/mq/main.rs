use mq::{Result, Server};
use std::collections::{HashMap, HashSet};
use std::io::{ErrorKind, Read};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, /*  Mutex, */ RwLock};

const PORT: u32 = 8000;
const BUFFER: usize = 1024;

fn main() -> Result<()> {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", PORT)).unwrap();
    println!("INFO: listening on port:{}", PORT);
    let caches = Arc::new(RwLock::new(HashMap::new()));
    let mut buffer = [0; BUFFER];

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let caches_clone = Arc::clone(&caches);
                std::thread::spawn(move || {
                    handle_client(stream, caches_clone, &mut buffer);
                });
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
    Ok(())
}

fn handle_client(
    mut stream: TcpStream,
    caches: Arc<RwLock<HashMap<String, HashSet<Vec<u8>>>>>,
    buffer: &mut [u8],
) {
    stream
        .set_nonblocking(true)
        .expect("Failed to set non-blocking mode");

    loop {
        let bytes_read = match stream.read(buffer) {
            Ok(0) => {
                println!("INFO: Connection closed by the client");
                break;
            }
            Ok(n) => n,
            Err(err) => match err.kind() {
                ErrorKind::WouldBlock => {
                    continue;
                }
                _ => {
                    eprintln!("ERROR: Got an unexpected error: {:?}", err);
                    break;
                }
            },
        };
        let mut server = Server::new(stream.try_clone().unwrap(), caches.clone());
        server.run(&mut buffer[..bytes_read]);
    }
}
