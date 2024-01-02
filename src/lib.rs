use std::collections::{HashMap, HashSet};
use std::io::{ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::{Arc, RwLock};
use std::{result, u8};
pub mod internal;

pub use crate::internal::messages::*;
pub use crate::internal::queue::*;

pub type Result<T> = result::Result<T, ()>;

#[derive(Debug)]
pub struct Server {
    pub stream: TcpStream,
    pub caches: Arc<RwLock<HashMap<String, HashSet<Vec<u8>>>>>,
}

impl Server {
    pub fn new(
        stream: TcpStream,
        caches: Arc<RwLock<HashMap<String, HashSet<Vec<u8>>>>>,
    ) -> Server {
        Server { stream, caches }
    }
    pub fn run(&mut self, buffer: &mut [u8]) {
        loop {
            let bytes_read = match self.stream.read(buffer) {
                Ok(0) => {
                    println!("INFO: Connection closed by the client");
                    break;
                }
                Ok(n) => n,
                Err(err) => match err.kind() {
                    ErrorKind::WouldBlock => {
                        eprintln!("EROR: would have blocked");
                        break;
                    }
                    _ => {
                        eprintln!("ERROR: Got an unexpected error: {:?}", err);
                        break;
                    }
                },
            };
            let message = decode(&buffer[..bytes_read]);
            match message {
                Ok(message) => {
                    self.handle_client_command(message);
                }
                Err(e) => {
                    eprintln!("EROR: WRONG MESSAGE FORMAT {e}");
                    self.stream
                        .shutdown(std::net::Shutdown::Both)
                        .expect("shutdown call failed");
                }
            }
        }
    }
    pub fn handle_client_command(&mut self, message: Message) {
        match Commands::from_u8(message.command.unwrap()) {
            Commands::QUIT => {
                self.stream
                    .shutdown(std::net::Shutdown::Both)
                    .expect("shutdown call failed");
            }
            Commands::SUBSCRIBE => {
                if let Some(name) = message.queue {
                    if let Some(queue) = self.caches.read().unwrap().get(name) {
                        for messages in queue {
                            let mut data: Vec<u8> = vec![];
                            data.push(messages.len() as u8);
                            data.append(&mut messages.to_owned());
                            self.stream.write_all(&data).unwrap();
                            println!("INFO: RECEIVED MESSAGE TO TOPIC:{name}");
                        }
                    }
                }
            }
            Commands::PUBLISH => {
                if let Some(name) = message.queue {
                    self.caches
                        .write()
                        .unwrap()
                        .entry(name.to_string())
                        .or_default()
                        .insert(message.message.unwrap().to_vec());
                    println!("INFO: PUBLISHED MESSAGE TO TOPIC:{name}");
                }
            }
            Commands::ACK => {
                println!("{:?}", message.queue);
            }
            Commands::NACK => {
                println!("{:?}", message.queue);
            }
            _ => {
                eprintln!("NO SUCH COMMAND");
            }
        }
    }
}
