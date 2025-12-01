# Ì∞ù The Hive: Distributed Task Execution Engine

**The Hive** is a compact, fault-tolerant distributed task scheduler written in Rust. It demonstrates a Master/Worker model where a central Master (the "Queen") dispatches tasks to lightweight Worker clients ("Drones") over a small custom TCP application protocol and JSON-encoded payloads.

This repository is intended as both an educational example and a small, working demo you can run locally.

---

## Ì∫Ä Features

- Master (Queen) / Worker (Drone) architecture
- Small custom TCP application protocol with JSON-serialized messages using `serde` and `serde_json`
- Shared types (the `shared` crate) to guarantee type-safe protocol compatibility
- Example code demonstrating task dispatch, processing, and completion acknowledgement
- Small codebase ‚Äî easy to extend, test, and learn from

---

## Ìª† Architecture Overview

The workspace contains three crates:

1. `queen` ‚Äî The server (master). Listens on TCP port 8080 and dispatches tasks to connected Drones.
2. `drone` ‚Äî The worker (client). Connects to the Queen, accepts a task, processes it, and returns a completion message.
3. `shared` ‚Äî Types shared between crates (the `Message` enum) to keep the protocol consistent.

Messages are serialized with `serde`/`serde_json` and exchanged as UTF-8 JSON strings.

---

## Ì≤ª Quickstart ‚Äî Run locally

**Prerequisites**

- Rust toolchain + Cargo (stable channel recommended)
- Git

**Clone the repository**

```bash
git clone https://github.com/aniketvarma/distributed-hive.git
cd distributed-hive
```

**Build (optional)**

```bash
cargo build --manifest-path queen/Cargo.toml
cargo build --manifest-path drone/Cargo.toml
```

**Start the Queen** (Terminal 1)

```bash
cargo run --manifest-path queen/Cargo.toml
```

**Start one or more Drone clients** (Terminal 2+)

```bash
cargo run --manifest-path drone/Cargo.toml
```

You should see the Queen accept connections from the Drone(s) and exchange a Task ‚Üí TaskComplete message pair.

---

## Ì∑© Extending The Hive

- Add new message types to `Message` in `shared` for richer functionality (heartbeats, status, job metadata).
- Move to a binary serialization format (e.g. `bincode`) to reduce network overhead.
- Replace the synchronous server loop with asynchronous concurrency using `tokio` to support many simultaneous Drones.
- Add authentication and TLS for secure, production-ready communication.

---

## Ì∑™ Tests & development

Run tests for each crate explicitly:

```bash
cargo test -p shared
cargo test -p queen
cargo test -p drone
```

For interactive debugging, start Queen and one or more Drone processes and inspect the logs.

---

## Ì≥¶ License

This project is licensed under the MIT License ‚Äî see the [LICENSE](LICENSE) file for details.

---

If you'd like I can also add:

- GitHub Actions CI that builds the workspace on push/PR
- An example demonstrating multiple concurrent Drone workers
- Improved logging, error handling, and a small CLI wrapper around each crate

Tell me which you'd prefer next and I will add it.
