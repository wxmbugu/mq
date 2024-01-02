use core::time::Duration;
use mq::Message;
use std::io::prelude::*;
use std::net::TcpStream;

fn construct_message<'a>(message: Message<'a>, buffer: &'a mut [u8]) -> &'a mut [u8] {
    message.encode(buffer);
    buffer
}

fn read_messages(stream: &mut TcpStream) -> std::io::Result<()> {
    let message = Message {
        command: Some(1),
        queue: Some("dsfkaslf"),
        message: None,
    };
    let mut buffer = [0; 128];
    let message_buffer = construct_message(message, &mut buffer);

    // Write the message with length byte
    stream.write_all(message_buffer)?;

    // Read the length byte
    match stream.read_exact(&mut buffer[..1]) {
        Ok(_) => {
            let message_length = buffer[0] as usize;
            // Read the entire message using the calculated length
            stream.read_exact(&mut buffer[..message_length])?;

            let received_data = String::from_utf8_lossy(&buffer[..message_length]);
            println!("Received data: {}", received_data);
            Ok(())
        }
        Err(ref err) if err.kind() == std::io::ErrorKind::WouldBlock => {
            // Handle WouldBlock error here
            println!("WouldBlock error. Sleeping or retrying...");
            Ok(())
        }
        Err(err) => Err(err),
    }
}
static ADDR: &str = "127.0.0.1:8000";

fn main() {
    let mut stream = TcpStream::connect(ADDR).unwrap();
    stream
        .set_read_timeout(Some(Duration::from_millis(1000)))
        .unwrap();

    loop {
        read_messages(&mut stream).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
