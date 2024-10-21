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


Tenjin is The software-defined networking framework written in Rust, offering high performance and memory safety. It can be used as both a framework and a command line tool.

## Menu

- [Installation to your project](#installation-to-your-project)
- [Installation as Command line program](#installation-as-command-line-program)
- [Cli usage](#cli-usage)
- [Run The example controller](#run-the-example-controller)
- [Mininet](#mininet)

## Installation to your project

#### Normal install includes `full` feature by default.

```bash
cargo add tenjin_sdn
```

#### if you pefer to use only needed library with lightweight and faster compile.

some features needed dependencies you might be not using,which can cause slow compilation.
Using only features you needed can decrease number of dependencies you need to compile.

```bash
cargo install tenjin_sdn --no-default-features 
```
if you need to use example controller, add `-F example` flag to command.

### Install Tokio 

Tenjin is asynchronous,so for using it, you need to make your main function be async by using [tokio](https://tokio.rs/). install tokio with command below.

```bash
cargo add tokio
```


## Installation as Command line program

With Tenjin as cli you can run **The example controller** with your terminal without writing any code. (see example controller at ./src/example)

### Install [Rust](https://www.rust-lang.org/)

first, you need `rust` and `cargo` to install Tenjin as comand line program. go to official website for installation.

[official webpage installation](https://www.rust-lang.org/tools/install)

or run this command below. (For macOS, Linux, or another Unix-like OS)

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Install Tenjin

after installing rust and cargo, you can use command below to install Tenjin.

```
cargo install tenjin_sdn 
```

## Cli usage

#### Run Controller by default (Controller13 with OpenFlow 1.3)

```bash
tenjin run
```

#### Run Controller10 with Openflow 1.0

```bash
tenjin run ctrl10
```

#### Run with specific port

```bash
tenjin run --port 6653
```

```bash
tenjin run --port 6653,6633
```


#### Show details of `run` command

```bash
tenjin run --help
```

## Run The example controller

After you install `tenjin_sdn` to your project with feature `example`, you can run example controller with this code below.

### Openflow 1.3

```rust
use tenjin::{example, openflow::ofp13::ControllerFrame13};

fn main() {
    let controller = example::Controller13::new();
    controller.listener("127.0.0.1:6633");
}
```

### Openflow 1.0


```rust
use tenjin::{example, openflow::ofp10::ControllerFrame10};

fn main() {
    let controller = example::Controller10::new();
    controller.listener("127.0.0.1:6633");
}
```

## Mininet

Mininet is a network emulator to create virtual networks for rapid prototyping of Software-Defined.
Using mininet for testing this SDN Framework.

### Run Mininet with Openflow 1.3

```bash
sudo mn --controller=remote,ip=127.0.0.1 --mac --switch=ovsk,protocols=OpenFlow13 --topo=tree,2
```

### Run Mininet with Openflow 1.0

```bash
sudo mn --controller=remote,ip=127.0.0.1 --mac --switch=ovsk,protocols=OpenFlow10 --topo=tree,2
```

## Learning resources

- [Openflow 1.3 Document](https://opennetworking.org/wp-content/uploads/2014/10/openflow-spec-v1.3.0.pdf)
- [rust_ofp](https://github.com/baxtersa/rust_ofp)
- [awesome-sdn](https://github.com/sdnds-tw/awesome-sdn)
- [ryu](https://github.com/faucetsdn/ryu)
- [learn-sdn-with-ryu](https://github.com/knetsolutions/learn-sdn-with-ryu)
