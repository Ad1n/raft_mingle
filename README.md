# Raft Mingle

![WIP](https://img.shields.io/badge/status-WIP-yellow)
![Build Status](https://img.shields.io/badge/build-passing-brightgreen)

Raft consensus algorithm implementation

## Overview

`raft_mingle` is an educational implementation of the Raft consensus algorithm in Rust. The Raft algorithm is a
distributed consensus algorithm that ensures a replicated log is consistently replicated across a group of servers. This
project aims to provide a clear and understandable implementation of Raft, making it easier to learn and experiment with
distributed consensus.

## Features

- **Leader Election**: Implements the leader election process to ensure a single leader is chosen among the servers.
- **Log Replication**: Ensures that logs are consistently replicated across all servers in the cluster.
- **Safety**: Guarantees that the system remains in a consistent state even in the presence of failures.
- **Fault Tolerance**: Handles server failures gracefully, ensuring the system can continue to operate.

## Getting Started

To get started with `raft_mingle`, follow these steps:

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Docker](https://www.docker.com/get-started)
- [Docker Compose](https://docs.docker.com/compose/install/)

### Installation

1. Clone the repository:
    ```sh
    git clone https://github.com/antonshevtsov/raft_mingle.git
    ```

2. Navigate to the project directory:
    ```sh
    cd raft_mingle
    ```
3. Ensure that Docker and Docker Compose are installed on your system;

4. Use the provided `docker-compose.yml` file to set up the environment. The file configures three instances of the Raft
   server. You can add new instances if needed. Just ensure that:

    ```yaml
    instance1:                          # Unique instanceX name
      image: raft_mingle:latest         # Docker image name
      hostname: raft1                   # Unique hostname for each node
      environment:
        - ID=1                          # Unique ID within all nodes
        - PEER_IDS=2,3                  # IDs of other nodes
        - PORT=8001                     # Port to start the server
        - RPC_CLIENTS_PORTS=8002,8003   # Other nodes ports
      ports:
        - "0.0.0.0:8001:8001"
    ```

A few things to keep in mind:

- Each node should have a unique ID and hostname.
- The `PEER_IDS` list should include the IDs of the other nodes in the cluster.
- Expose the ports correctly so the nodes can communicate with each other.

### Running the educational cluster

1. After the configuration is complete, bring up the cluster:
    ```shell
    docker compose up
    ```

## Configuration

The project uses a `Cargo.toml` file for configuration. Dependencies and other settings are defined in this file. The
project is organized into several libraries and services, each with its own `Cargo.toml` file.

## Project Structure

The project is organized as follows:

- **libs**: Contains several subdirectories, each representing a distinct library.
    - **consensus**: The core implementation of the Raft consensus algorithm.
    - **error**: Custom error types and handling.
    - **log_entry**: Log entry management.
    - **storage**: Persistent storage for logs and state.
- **services**: Contains the server implementation.
    - **server**: The main server application.
- **Dockerfile** and **Dockerfile.sh**: Docker containerization and orchestration files.
- **docker-compose.yml**: Defines and runs multi-container Docker applications.
- **build.sh**: Shell script for building the project.
- **rust-toolchain** and **rustfmt.toml**: Rust toolchain configuration and formatting files.

## Usage

The `raft_mingle` project can be used to learn and experiment with the Raft consensus algorithm. It is suitable for
building distributed systems or understanding the underlying principles of consensus algorithms.

## Contributing

Contributions are welcome! If you have any suggestions, bug reports, or feature requests, please open an issue or submit
a pull request.

## Acknowledgements

- [Raft Consensus Algorithm](https://raft.github.io)
- [Rust Programming Language](https://www.rust-lang.org)

## Contact

For any questions or inquiries, please contact [Ad1n](https://github.com/Ad1n)
