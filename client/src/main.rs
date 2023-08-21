use anyhow::Result;
use clap::Parser;
use shared::Message;
use std::{
    io::prelude::*,
    net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream},
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, default_value_t = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)))]
    addr: IpAddr,

    #[arg(short, long, default_value_t = 8080, value_parser = clap::value_parser!(u16).range(1..))]
    port: u16,

    #[arg(short, long, default_value_t = 5)]
    int_to_send: i32,

    #[arg(short, long, default_value_t = String::from("foo"))]
    string_to_send: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    println!("Connecting to {}:{}", cli.addr, cli.port);

    // Note the `?`
    let mut stream = TcpStream::connect(SocketAddr::new(cli.addr, cli.port))?;

    let msg = Message::new(cli.int_to_send, cli.string_to_send, true);
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
