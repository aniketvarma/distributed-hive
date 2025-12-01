# 🐝 The Hive: Distributed Task Execution Engine

**The Hive** is a fault-tolerant, distributed task scheduler built from scratch in **Rust**. It implements a custom Application Layer protocol over TCP to coordinate work between a Master Node (Queen) and multiple Worker Nodes (Drones).

## 🚀 Key Engineering Features

* **Custom TCP Protocol:** Designed a low-level binary communication protocol, avoiding HTTP overhead for maximum performance.
* **Master-Slave Architecture:** Implemented a centralized "Queen" server that manages connection lifecycles for ephemeral "Drone" workers.
* **Binary Serialization:** Utilized `serde` and `serde_json` to marshal complex Rust data structures into bytes for network transport.
* **Concurrency:** Handled multiple worker connections sequentially without blocking the main server loop.
* **Fault Tolerance:** The system handles worker disconnection gracefully and validates task completion via a full-duplex feedback loop.

## 🛠️ Architecture

The system consists of three distinct components within a Rust Workspace:

1. **Queen (Server):** Listens on Port 8080. Dispatches tasks and awaits completion reports.
2. **Drone (Worker):** Connects to the Queen, accepts the payload, processes the data, and returns a status report.
3. **Shared Library:** A type-safe definitions crate ensuring protocol compatibility between nodes.

## 💻 How to Run

**Prerequisites:** Rust and Cargo installed.

### 1. Start the Queen (Terminal 1)

```bash
cargo run -p queen
```

### 2. Start a Drone (Terminal 2)

```bash
cargo run -p drone
```

## 📸 Output Example

**Queen Output:**

```text
👑 Queen is listening on port 8080...
🐝 A Drone just connected!
🚀 Task sent to Drone. Waiting for report...
🎉 Queen received report: TaskComplete("image_1.png")
```

**Drone Output:**

```text
🐝 Drone is trying to connect...
✅ Connected to Queen!
🚨 I received a JOB: image_1.png
...Working...
✅ Sent completion report to Queen!
```