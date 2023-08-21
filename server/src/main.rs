use anyhow::Result;
use log::{error, info};
use shared::Message;
use std::{
    io::prelude::*,
    net::{TcpListener, TcpStream},
    thread,
};

fn main() -> Result<()> {
    env_logger::init();

    // Note the `?`
    let listener = TcpListener::bind("localhost:8080")?;

    info!("Listening @{}", listener.local_addr()?);

    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                thread::spawn(move || match handle_connection(s) {
                    Err(e) => error!("Error handling connection: {:?}", e),
                    _ => (),
                });
            }
            Err(e) => error!("Connection failed: {:?}", e),
        }
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> Result<()> {
    let peer_addr = stream.peer_addr()?;
    info!("Accepted connection from {}", peer_addr);

    let mut buf = [0 as u8; 1024];
    let bytes_read = stream.read(&mut buf)?;
    if bytes_read == 0 {
        info!("({}) Read 0 bytes, connection closed", peer_addr);
        return Ok(());
    } else {
        info!("({}) Read {} bytes.", peer_addr, bytes_read);
    }

    let recvd: Message = bincode::deserialize(&buf)?;
    info!("({}) Got: {:?}", peer_addr, recvd);

    let to_send = recvd.increment();
    info!("({}) Sending: {:?}", peer_addr, to_send);

    let to_send_bytes = bincode::serialize(&to_send)?;
    stream.write_all(&to_send_bytes)?;
    stream.flush()?;
    info!("({}) {} bytes sent!", peer_addr, to_send_bytes.len());

    Ok(())
}
