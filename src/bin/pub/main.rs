#![allow(dead_code)]
use core::time::Duration;
use mq::Message;
use std::io::prelude::*;
use std::net::TcpStream;

fn construct_message<'a>(message: Message<'a>, buffer: &'a mut [u8]) -> &'a mut [u8] {
    message.encode(buffer);
    buffer
}
fn send_messages(stream: &mut TcpStream, message: &str) -> std::io::Result<()> {
    let message = Message {
        command: Some(2),
        queue: Some("dsfkaslf"),
        message: Some(message.as_bytes()),
    };
    let mut buffer = [0; 512];
    let message_buffer = construct_message(message, &mut buffer);

    stream.write_all(message_buffer)?;
    Ok(())
}

static ADDR: &str = "127.0.0.1:8000";

fn main() {
    let mut stream = TcpStream::connect(ADDR).unwrap();
    stream
        .set_read_timeout(Some(Duration::from_millis(1000)))
        .unwrap();

    // let mut i = 0;
    send_messages(&mut stream, "Hello world!!").unwrap();
    std::thread::sleep(std::time::Duration::from_millis(10000));
    send_messages(&mut stream, "Trial Send message").unwrap();
    std::thread::sleep(std::time::Duration::from_millis(10000));
    send_messages(&mut stream, "Another Send message test").unwrap();
    std::thread::sleep(std::time::Duration::from_millis(10000));
    send_messages(&mut stream, "Blue eye Samurai").unwrap();
    std::thread::sleep(std::time::Duration::from_millis(10000));

    // if i == 4 {
    // break;
    // }
    // i += 1;
}
