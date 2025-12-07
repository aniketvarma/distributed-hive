# 🐝 The Hive: Distributed Task Execution Engine

The Hive is a fault-tolerant, distributed task scheduler built from scratch in Rust. It implements a hybrid architecture combining a low-level TCP protocol for worker coordination and a high-level REST API for data observability.

## 🚀 Key Engineering Features

- **Hybrid Server Architecture:** Runs two concurrent services (TCP + HTTP) within a single Async Runtime (Tokio).
- **Distributed Computing:** Nodes perform CPU-intensive cryptographic work (SHA-256 hashing) distributed across the network.
- **Persistence:** All results are asynchronously logged to a SQLite database using connection pooling.
- **REST API:** Exposes a public JSON dashboard using Axum to monitor task status in real-time.
- **Custom TCP Protocol:** Designed a binary communication protocol with `serde` serialization for maximum worker performance.

## 🛠️ Architecture

The system consists of three distinct components within a Rust workspace:

- **Queen (Master Node):**
  - Port 8080 (TCP): Manages ephemeral "Drone" workers.
  - Port 3000 (HTTP): Serves the JSON API.
  - Database: Manages the SQLite connection pool.

- **Drone (Worker Node):** Connects to the Queen, accepts payloads, calculates SHA-256 hashes, and returns results.

- **Shared Library:** A type-safe definitions crate ensuring protocol compatibility.

## 💻 How to Run

**Prerequisites:** Rust and Cargo installed.

1. Start the Queen (Terminal 1)

   This initializes the database and starts both the TCP and HTTP servers:

   ```bash
   cargo run -p queen
   ```

2. Start a Drone (Terminal 2)

   ```bash
   cargo run -p drone
   ```

3. Check the Dashboard (Browser)

   Open your browser to see the real-time list of completed tasks (JSON):

   http://localhost:3000/tasks

## 📸 Output Example

**Queen Output:**

```
💾 Database initialized.
🌍 HTTP API running at http://localhost:3000/tasks
👑 TCP Service active on port 8080...
🎉 Worker finished task: 7c0b5f54...
```

**Browser Output (JSON):**

```json
[
  {
    "id": 1,
    "task_name": "super_secret_password_123",
    "result_hash": "28052f444cf50f176029269c3f0e3e128edfffa52df2d0c619e3a2b4b1b0a250"
  }
]
```
