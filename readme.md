# 🦀️ Rust gRPC Hello World

A simple gRPC server and client implementation in Rust.

## 📋 Table of Contents

- [🌟 Features](#-features)
- [🛠️ Prerequisites](#️-prerequisites)
- [🚀 Getting Started](#-getting-started)
- [🏗️ Project Structure](#️-project-structure)
- [🔧 Usage](#-usage)


## 🌟 Features

- Simple gRPC server implementation
- Matching gRPC client
- Asynchronous communication using Tokio
- Protocol Buffers for efficient data serialization

## 🛠️ Prerequisites

Before you begin, ensure you have met the following requirements:

- Rust (latest stable version)
- Cargo (comes with Rust)
- Protocol Buffers compiler (protoc)

## 🚀 Getting Started

To get this project up and running on your local machine, follow these steps:

1. Clone the repository:
   ```
   git clone https://github.com/DAGGE3R/rust-grpc.git
   cd rust-grpc-hello-world
   ```

2. Build the project:
   ```
   cargo run
   ```

## 🏗️ Project Structure

```
rust-grpc-hello-world/
│
├── src/
│   ├── server.rs    # gRPC server implementation
│   └── client.rs    # gRPC client implementation
│
├── proto/
│   └── helloworld.proto    # Protocol Buffers definition
│
├── build.rs    # Build script for compiling .proto files
└── Cargo.toml  # Rust package manifest
```

## 🔧 Usage

1. Start the server:
   ```
   cargo run --bin grpc-server
   ```

2. In a new terminal, run the client:
   ```
   cargo run --bin grpc-client
   ```

The client will send a request to the server and print the response.



Made with ❤️ and Rust