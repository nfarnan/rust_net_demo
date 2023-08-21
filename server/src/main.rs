use anyhow::Result;
use clap::Parser;
use log::{error, info};
use shared::Message;
use std::{
    io::prelude::*,
    net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream},
    thread,
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, default_value_t = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)))]
    addr: IpAddr,

    #[arg(short, long, default_value_t = 8080, value_parser = clap::value_parser!(u16).range(1..))]
    port: u16,
}

fn main() -> Result<()> {
    env_logger::init();

    let cli = Cli::parse();

    // Note the `?`
    let listener = TcpListener::bind(SocketAddr::new(cli.addr, cli.port))?;

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
