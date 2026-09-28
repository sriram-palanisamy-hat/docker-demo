# Mini Container (docker-demo)

A lightweight container runtime implementation in Rust that demonstrates the basics of containerization, including network bridge creation and running isolated processes.

## Features

- **Network Management**: Create bridge networks with custom subnets and gateways.
- **Container Execution**: Run containers attached to the created network with a specific IP address.

## Prerequisites

- Rust (edition 2021)
- Cargo
- Linux environment (utilizes `nix` for Linux-specific features like namespaces and mounts)

## Dependencies

- `clap` - Command-line argument parsing
- `nix` - Linux system APIs (namespaces, mounts)
- `rtnetlink` - Netlink routing sockets for network management
- `tokio` - Asynchronous runtime
- `anyhow` - Error handling

## Usage

### Build the Project

```bash
cargo build --release
```

### Create a Network

Create a bridge network by providing a name, subnet, and gateway.

```bash
cargo run -- network create --name <network-name> --subnet <subnet-cidr> --gateway <gateway-ip>
```

**Example:**
```bash
cargo run -- network create --name mybridge --subnet 192.168.1.0/24 --gateway 192.168.1.1
```

### Run a Container

Start a container and connect it to a network with a specific IP address.

```bash
cargo run -- run --net <network-name> --ip <container-ip>
```

**Example:**
```bash
cargo run -- run --net mybridge --ip 192.168.1.10
```

## License

This project is open-source and available under standard terms.
