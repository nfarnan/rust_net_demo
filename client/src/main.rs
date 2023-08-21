use anyhow::Result;
use shared::Message;
use std::{io::prelude::*, net::TcpStream};

fn main() -> Result<()> {
    // Note the `?`
    let mut stream = TcpStream::connect("localhost:8080")?;

    let msg = Message::new(5, String::from("foo"), true);
    let msg_bytes = bincode::serialize(&msg)?;

    stream.write_all(&msg_bytes)?;
    stream.flush()?;

    println!("Sent {} bytes to server!", msg_bytes.len());

    println!("Getting server response...");
    let mut buf = [0 as u8; 1024];
    let bytes_read = stream.read(&mut buf)?;
    if bytes_read == 0 {
        println!("Connection closed, read nothing...");
        return Ok(());
    } else {
        println!("Read {} bytes.", bytes_read);
    }

    let recvd: Message = bincode::deserialize(&buf)?;
    println!("Got: {:?}", recvd);

    Ok(())
}
