# EdgeDB

EdgeDB is an experimental, high-performance, distributed database prototype inspired by Redis. Written in Rust, EdgeDB supports in-memory key–value operations with TTL and advanced eviction, as well as stubs for clustering, pub/sub messaging, transactions, scripting, security/ACLs, and monitoring.

## Features

- **Core Key–Value Store:** In-memory storage with TTL and background cleanup, plus optional RocksDB persistence.
- **Advanced Data Structures:** (Stubs) Support for lists, sets, hashes, sorted sets, etc.
- **Clustering & Replication:** (Stub) Distributed deployment and replication.
- **Pub/Sub:** Basic publish/subscribe messaging using asynchronous channels.
- **Transactions & Scripting:** (Stub) Atomic operations and Lua scripting integration.
- **Security & ACLs:** (Stub) Authentication and fine-grained access control.
- **Monitoring:** Integrated Prometheus metrics.
- **Optimized TTL Management:** Background cleanup task and placeholders for advanced eviction strategies.

## Getting Started

### Build

Ensure you have Rust installed via [rustup](https://rustup.rs). Then run:
```bash
cargo build
