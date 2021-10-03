# Phu Quoc Dog Node

Phu Quoc Dog is a Substrate-based Polkadot parachain. Phu Quoc Dog is committed to providing easy-to-use blockchain infrastructure and creating a one-stop comprehensive infrastructure platform, which aims to reduce the threshold and cost for developers, while bringing them great returns.

PQD is the native token of the Polkadot network in a similar way that BTC is the native token of Bitcoin or Ether is the native token of the Ethereum blockchain.

## Getting Started

Follow the steps below to get started with the Node Template, or get it up and running right from your browser
in just a few clicks using [Playground](https://playground.substrate.dev/) :hammer_and_wrench:

### Using Nix

Install [nix](https://nixos.org/) and optionally [direnv](https://github.com/direnv/direnv) and [lorri](https://github.com/target/lorri) for a fully plug
and play experience for setting up the development environment. To get all the correct dependencies activate direnv `direnv allow` and lorri `lorri shell`.

### Rust Setup

First, complete the [basic Rust setup instructions](./docs/rust-setup.md).

### Run

Use Rust's native `cargo` command to build and launch the template node:

```sh
cargo run --release -- --dev --tmp

// cargo +nightly run --release --features=runtime-benchmarks
```

### Build

The `cargo run` command will perform an initial build. Use the following command to build the node
without launching it:

```sh
cargo build --release
```

## Run

The provided `cargo run` command will launch a temporary node and its state will be discarded after
you terminate the process. After the project has been built, there are other ways to launch the
node.

### Single-Node Development Chain

This command will start the single-node development chain with persistent state:

```bash
./target/release/phuquocdog-node --dev
```

Purge the development chain's state:

```bash
./target/release/phuquocdog-node purge-chain --dev
```

Start the development chain with detailed logging:

```bash
RUST_LOG=debug RUST_BACKTRACE=1 ./target/release/phuquocdog-node -lruntime=debug --dev
```

### Connect with Polkadot-JS Apps Front-end

Once the node template is running locally, you can connect it with **Polkadot-JS Apps** front-end
to interact with your chain. [Click here](https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9944) connecting the Apps to your local node template.


### Note

Build customSpecRaw: 

```
./target/release/phuquocdog-node build-spec --disable-default-bootnode --chain quark > customSpec.json

```

```
./target/release/phuquocdog-node build-spec --chain=customSpec.json --raw --disable-default-bootnode > customSpecRaw.json
```

