# Tenjin SDN

[![version]](https://crates.io/crates/tenjin_sdn)
[![download]](https://crates.io/crates/tenjin_sdn)
[![license]](LICENSE)
![size]
[![issue]](https://github.com/Arikato111/Tenjin/issues)
![last-commit]

[last-commit]: https://img.shields.io/github/last-commit/Arikato111/Tenjin
[size]: https://img.shields.io/crates/size/tenjin_sdn
[issue]: https://img.shields.io/github/issues/Arikato111/Tenjin
[license]: https://img.shields.io/github/license/Arikato111/Tenjin
[download]: https://img.shields.io/crates/d/tenjin_sdn
[version]: https://img.shields.io/crates/v/tenjin_sdn

## Table of Contents
- [Features](#features)
- [Quick Start](#quick-start)
  - [As a Command-Line Tool](#as-a-command-line-tool)
  - [As a Library](#as-a-library)
- [Usage Guide](#usage-guide)
  - [Command-Line Interface](#command-line-interface)
  - [Network Emulation with Mininet](#network-emulation-with-mininet)
- [Advanced Installation](#advanced-installation)
  - [Minimal Installation](#minimal-installation)
  - [Binary Installation](#binary-installation)
  - [Docker](#docker)

## Features

- High performance and memory safety through Rust
- Support for OpenFlow 1.0 and 1.3
- Asynchronous operation with Tokio
- Built-in example controllers
- Command-line interface for quick testing
- Mininet integration for network emulation

## Quick Start

### As a Command-Line Tool

1. Install Rust and Cargo:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Install Tenjin:

```bash
cargo install tenjin_sdn
```

3. Run the example controller:

```bash
tenjin run
```

### As a Library

1. Add Tenjin to your project:

```bash
cargo add tenjin_sdn
```

2. Add Tokio for async support:

```bash
cargo add tokio
```

3. Use in your code:

```rust
use tenjin_sdn::{example, openflow::ofp13::ControllerFrame13};

#[tokio::main]
async fn main() {
    let controller = example::Controller13::new();
    controller.listener("127.0.0.1:6633");
}
```
4. if you would like to create your own Controller, you need to install [etherparse](https://crates.io/crates/etherparse).

```bash
cargo add etherparse
```

## Usage Guide

### Command-Line Interface

#### Basic Usage

```bash
# Run default controller (OpenFlow 1.3)
tenjin run

# Run OpenFlow 1.0 controller
tenjin run ctrl10

# Run on specific ports
tenjin run --port 6653
tenjin run --port 6653,6633
```

For more options:

```bash
tenjin run --help
```

### Network Emulation with Mininet

#### OpenFlow 1.3

```bash
sudo mn --controller=remote,ip=127.0.0.1 --mac --switch=ovsk,protocols=OpenFlow13 --topo=tree,2
```

#### OpenFlow 1.0

```bash
sudo mn --controller=remote,ip=127.0.0.1 --mac --switch=ovsk,protocols=OpenFlow10 --topo=tree,2
```

## Advanced Installation

### Minimal Installation

For faster compilation, you can install only the features you need:

```bash
cargo install tenjin_sdn --no-default-features
```

To include example controllers, add the `example` feature:

```bash
cargo install tenjin_sdn --no-default-features -F example
```

### Binary Installation

Using cargo-binstall for pre-compiled binaries:

```bash
cargo binstall tenjin_sdn
```

### Docker

run the following command to pull and run.

```
docker run -it --rm --name tenjin ghcr.io/arikato111/tenjin:latest run

```

or using alias command on Linux.

```
alias tenjin='docker run -it --rm --name tenjin ghcr.io/arikato111/tenjin:latest'
```
