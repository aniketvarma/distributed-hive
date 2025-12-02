# 🐝 The Hive: Distributed Task Execution Engine

The Hive is a fault-tolerant, distributed task scheduler built from scratch in Rust. It implements a custom Application Layer protocol over TCP to coordinate work between a Master Node (Queen) and multiple Worker Nodes (Drones).

## 🚀 Key Engineering Features

- **Distributed Computing:** Nodes perform CPU-intensive cryptographic work (SHA-256 hashing) distributed across the network.  
- **Custom TCP Protocol:** Designed a low-level binary communication protocol, avoiding HTTP overhead for maximum performance.  
- **Master-Slave Architecture:** Implemented a centralized "Queen" server that manages connection lifecycles for ephemeral "Drone" workers.  
- **Binary Serialization:** Utilized `serde` and `serde_json` to marshal complex Rust data structures into bytes for network transport.  
- **Concurrency:** Handled multiple worker connections sequentially using multi-threading without blocking the main server loop.

## 🛠️ Architecture

The system consists of three distinct components within a Rust Workspace:

1. **Queen (Server):** Listens on port 8080. Dispatches computational tasks (strings to hash) and awaits results.  
2. **Drone (Worker):** Connects to the Queen, accepts the payload, calculates the SHA-256 hash, and returns the result.  
3. **Shared Library:** A type-safe definitions crate ensuring protocol compatibility between nodes.

## 💻 How to Run

**Prerequisites:** Rust and Cargo installed.

1. Start the Queen (Terminal 1)
```bash
cargo run -p queen
```

2. Start a Drone (Terminal 2)
```bash
cargo run -p drone
```

## 📸 Output Example

Queen Output:
```
👑 Queen is listening on port 8080 (Swarm Mode)...
🐝 A Drone connected!
🚀 Task sent to background thread.
🎉 Received reply from drone: TaskComplete("7c0b5f54...")
```

Drone Output:
```
🐝 Drone is trying to connect...
✅ Connected to Queen!
🚨 Received Work: Hash the word 'super_secret_password_123'
...Crunching numbers...
✅ Work Done! Hash: 7c0b5f54...
```