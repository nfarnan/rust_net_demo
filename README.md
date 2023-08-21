# Rust Networking Demo

Relatively basic echo client/server. Sends a custom struct back and forth.

Both client, server, and shared data structures are setup as separate package in a single Cargo workspace
(named `client`, `server`, and `shared`, respectively).

You can run the server (with logging information displayed) from the root of this workspace with:

```bash
RUST_LOG=info cargo run -p server
```

The server is hardcoded to run on `localhost:8080`.

Once the server is started, you can run the client with:

```bash
cargo run -p client
```

## Libraries Used

The core is all standard library, specifically:
  * [`std::net`](https://doc.rust-lang.org/std/net/): Specifically `TcpListener` and `TcpStream`
  * [`std::thread`](https://doc.rust-lang.org/std/thread/): For handling client connections on the server

I also pull from several crates:
  * [`env_logger`](https://crates.io/crates/env_logger) and [`log`](https://crates.io/crates/log): Logging server activity
  * [`anyhow`](https://crates.io/crates/anyhow): Easier error handling/propagation
  * [`bincode`](https://crates.io/crates/bincode): Binary serialization/deserialization
  * [`serde`](https://crates.io/crates/serde): Traits to prepare custom structs to be serialized/deserialized by `bincode`

Because of recent shenangins with `serde`, the provided `Cargo.toml` files lock its version at 1.0.171.