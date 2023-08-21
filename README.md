# Rust Networking Demo

Relatively basic echo client/server. Sends a custom struct back and forth.

Both client, server, and shared data structures are setup as separate packages in a single Cargo workspace
(named `client`, `server`, and `shared`, respectively).

You can run the server (with logging information displayed) from the root of this workspace with:

```bash
RUST_LOG=info cargo run -p server
```

Once the server is started, you can run the client with:

```bash
cargo run -p client
```

The server accepts command line arguments to set the address and port to listen on (defaults to `127.0.0.1:8080`).

The client accepts command line arguments to set the address and port to connect to, as well as the integer and string to
send in the message.

Note that command line args must be passed to an app running in cargo after a `--`. E.g., to listen on IPv6:

```bash
RUST_LOG=info cargo run -p server -- -a ::1
```

## Libraries Used

Core functionality is from standard library modules:
  * [`std::net`](https://doc.rust-lang.org/std/net/): Specifically `TcpListener` and `TcpStream`
  * [`std::thread`](https://doc.rust-lang.org/std/thread/): For handling client connections on the server

Also pulls from several crates:
  * [`clap`](https://crates.io/crates/clap): Command line argument parsing
  * [`env_logger`](https://crates.io/crates/env_logger) and [`log`](https://crates.io/crates/log): Logging server activity
  * [`anyhow`](https://crates.io/crates/anyhow): Easier error handling/propagation
  * [`bincode`](https://crates.io/crates/bincode): Binary serialization/deserialization
  * [`serde`](https://crates.io/crates/serde): Traits to prepare custom structs to be serialized/deserialized by `bincode`
