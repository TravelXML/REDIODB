Below is a complete README file for Redio DB that incorporates your additional details. You can adjust any sections as needed:

---

# Redio DB

**Redio DB** is a lightweight, high-performance distributed database designed for real-time data operations and pub/sub messaging. It provides both a modern gRPC API and a Redis‑like CLI for non‑interactive and interactive usage.

[![Build Status](https://img.shields.io/travis/TravelXML/rediodb.svg)](https://travis-ci.org/TravelXML/rediodb)  
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

---

## Table of Contents

- [Features](#features)
- [Architecture](#architecture)
- [Installation](#installation)
- [Configuration](#configuration)
- [Usage](#usage)
  - [CLI Commands](#cli-commands)
  - [Interactive Mode (REPL)](#interactive-mode-repl)
  - [Direct gRPC Testing](#direct-grpc-testing)
- [Development](#development)
- [Contributing](#contributing)
- [License](#license)

---

## Features

**Basic Key–Value Operations:**

- **SET:** Insert a key with a value and optional TTL.
- **GET:** Retrieve the value of a key.
- **EXPIRE:** Set or update the TTL for a key.
- **TTL:** Check the remaining TTL for a key.
- **DEL:** Delete a key.

**Atomic Operations:**

- **INCR:** Atomically increment a numeric key.
- **DECR:** Atomically decrement a numeric key.
- **APPEND:** Append data to an existing key.

**Key Pattern Matching:**

- **KEYS:** List keys by pattern matching.

**Data Structures:**

- **Lists:**  
  - **LPUSH:** Push an element onto a list.  
  - **LPOP:** Pop an element from a list.
- **Sets:**  
  - **SADD:** Add a member to a set.  
  - **SMEMBERS:** Retrieve all members of a set.
- **Hashes:**  
  - **HSET:** Set a field in a hash.  
  - **HGET:** Retrieve a field from a hash.

**Transactions:**

- **MULTI/EXEC:** Queue and atomically execute multiple commands.

**Enhanced Pub/Sub:**

- **PUBLISH:** Publish messages to channels.
- **SUBSCRIBE:** Subscribe to one or more channels (supports multiple channels and pattern matching).

**CLI Interface:**

- A Redis‑cli–like command-line tool offering both one‑shot commands and an interactive shell (REPL mode).

**gRPC API:**

- All operations are exposed via a modern gRPC interface using protocol buffers.

---

## Architecture

Redio DB employs a distributed architecture to ensure high availability and real‑time performance. Its core components include:

- **Server:**  
  Handles client requests via gRPC, manages data storage, transactions, and pub/sub channels.
- **CLI Client:**  
  Offers a non‑interactive mode for one-shot commands and an interactive shell for REPL-style usage.
- **Storage Layer:**  
  Provides support for multiple backend storage systems (e.g., in‑memory, RocksDB, Arrow Cache).
- **Consensus & Replication:**  
  Implements built-in consensus algorithms to support clustering and replication.

---

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/) (stable toolchain recommended)
- [Cargo](https://doc.rust-lang.org/cargo/)
- A C compiler (e.g., gcc)
- [grpcurl](https://github.com/fullstorydev/grpcurl) (for direct gRPC testing)

### Build from Source

Clone the repository:

```bash
git clone https://github.com/yourusername/rediodb.git
cd rediodb
```

Build the project with Cargo:

```bash
cargo build --release
```

> **Note:** If you encounter linker issues, verify your configuration as described in the [Configuration](#configuration) section below.

---

## Configuration

Redio DB uses both environment variables and Cargo configuration files for runtime and build settings.

### Environment Variables

- **EDGEDB_ADDRESS:**  
  Specifies the address of the gRPC server. The default is `http://127.0.0.1:50051`.

Set it in your shell:

```bash
export EDGEDB_ADDRESS="http://127.0.0.1:50051"
```

### Cargo Linker Settings

If you need to change the linker (for example, to use gcc instead of lld), create a configuration file in your project root:

1. Create a directory named `.cargo` in the project root if it doesn’t exist:

   ```bash
   mkdir -p .cargo
   ```

2. Create a file named `config.toml` inside `.cargo` with the following content:

   ```toml
   [build]
   target = "x86_64-unknown-linux-gnu"
   rustflags = ["-C", "linker=gcc"]

   [target.x86_64-unknown-linux-gnu]
   linker = "gcc"
   ```

3. Clean and rebuild the project:

   ```bash
   cargo clean
   cargo build
   ```

---

## Usage

Redio DB provides both CLI and gRPC interfaces.

### CLI Commands

The CLI client is provided by the `rediodb-cli` binary. You can run commands in non‑interactive mode:

- **Display Help:**

  ```bash
  cargo run --bin rediodb-cli -- --help
  ```

- **Set a Key with TTL:**

  ```bash
  cargo run --bin rediodb-cli -- set mykey "myvalue" 60
  ```

- **Get a Key:**

  ```bash
  cargo run --bin rediodb-cli -- get mykey
  ```

- **Increment a Key:**

  ```bash
  cargo run --bin rediodb-cli -- incr counter 1
  ```

- **Subscribe to Channels:**

  ```bash
  cargo run --bin rediodb-cli -- subscribe channel1 channel2
  ```

Each command corresponds to a specific gRPC endpoint on the Redio server.

### Interactive Mode (REPL)

Launch the interactive shell with:

```bash
cargo run --bin rediodb-cli -- interactive
```

At the `rediodb>` prompt, you can enter commands like:

```plaintext
set mykey "myvalue" 60
get mykey
incr counter 1
subscribe channel1
exit
```

This mode provides command history and editing features similar to redis‑cli.

### Direct gRPC Testing with grpcurl

If your Redio server is running (default port: 50051) and you have the proto definition in `proto/rediodb.proto`, you can test endpoints directly using grpcurl.

#### Basic Key–Value Operations

- **SET:**

  ```bash
  grpcurl -plaintext -proto proto/rediodb.proto -import-path proto \
    -d '{"key": "mykey", "value": "myvalue", "ttl": 60}' \
    localhost:50051 rediodb.Rediodb/Set
  ```

- **GET:**

  ```bash
  grpcurl -plaintext -proto proto/rediodb.proto -import-path proto \
    -d '{"key": "mykey"}' \
    localhost:50051 rediodb.Rediodb/Get
  ```

- **EXPIRE:**

  ```bash
  grpcurl -plaintext -proto proto/rediodb.proto -import-path proto \
    -d '{"key": "mykey", "ttl": 120}' \
    localhost:50051 rediodb.Rediodb/Expire
  ```

- **TTL:**

  ```bash
  grpcurl -plaintext -proto proto/rediodb.proto -import-path proto \
    -d '{"key": "mykey"}' \
    localhost:50051 rediodb.Rediodb/Ttl
  ```

- **DEL:**

  ```bash
  grpcurl -plaintext -proto proto/rediodb.proto -import-path proto \
    -d '{"key": "mykey"}' \
    localhost:50051 rediodb.Rediodb/Del
  ```

#### Extended Atomic Operations

- **INCR:**

  ```bash
  grpcurl -plaintext -proto proto/rediodb.proto -import-path proto \
    -d '{"key": "counter", "amount": 1}' \
    localhost:50051 rediodb.Rediodb/Incr
  ```

- **DECR:**

  ```bash
  grpcurl -plaintext -proto proto/rediodb.proto -import-path proto \
    -d '{"key": "counter", "amount": 1}' \
    localhost:50051 rediodb.Rediodb/Decr
  ```

- **APPEND:**

  ```bash
  grpcurl -plaintext -proto proto/rediodb.proto -import-path proto \
    -d '{"key": "mykey", "value": " extra"}' \
    localhost:50051 rediodb.Rediodb/Append
  ```

#### Pub/Sub Operations

- **PUBLISH:**

  ```bash
  grpcurl -plaintext -proto proto/rediodb.proto -import-path proto \
    -d '{"channel": "channel1", "message": "Hello, world!"}' \
    localhost:50051 rediodb.Rediodb/Publish
  ```

- **SUBSCRIBE:**

  ```bash
  grpcurl -plaintext -proto proto/rediodb.proto -import-path proto \
    -d '{"channels": ["channel1"]}' \
    localhost:50051 rediodb.Rediodb/Subscribe
  ```

> **Note:** The subscribe endpoint is a streaming call and will remain active until interrupted (Ctrl+C).

---

## Development

To contribute or modify Redio DB:

1. **Fork the repository** and clone your fork locally.
2. **Create a new branch** for your feature or fix:

   ```bash
   git checkout -b feature/your-feature-name
   ```

3. **Make your changes** and run tests:

   ```bash
   cargo test
   ```

4. **Commit your changes** and push your branch.
5. **Submit a pull request.**

Please adhere to the existing coding standards and include appropriate documentation with your changes.

---

## Contributing

Contributions are welcome! Please review the [CONTRIBUTING](CONTRIBUTING.md) guidelines for details on our code of conduct and the process for submitting pull requests.

---

## License

This project is licensed under the MIT License – see the [LICENSE](LICENSE) file for details.

---

By following these instructions, you can run Redio DB in both non‑interactive and interactive modes, test its endpoints via grpcurl, and extend or contribute to the project. Happy coding!

