# distributed-hive

A small Rust example of a "Queen" server assigning tasks to "Drone" clients over TCP using JSON (serde).

Project structure
- queen/ — server that sends tasks
- drone/ — client that receives tasks and replies
- shared/ — shared types (Message enum) used by both

Prerequisites
- Rust toolchain (stable recommended)
- Git

Build & run (from project root)

1) Build both crates

```bash
cargo build --manifest-path queen/Cargo.toml
cargo build --manifest-path drone/Cargo.toml
```

2) Run the Queen server (in one terminal)

```bash
cargo run --manifest-path queen/Cargo.toml
```

3) Run the Drone client (in another terminal)

```bash
cargo run --manifest-path drone/Cargo.toml
```

Notes
- Uses serde / serde_json for message (de)serialization across the `shared` crate.
- Ensure `edition = "2021"` and required dependencies are set in each crate's Cargo.toml.

License
Add your license of choice here.
