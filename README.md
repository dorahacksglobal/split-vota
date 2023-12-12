# Split Bills on Vota


## Quick Start

[Setup Rust](https://rustup.rs/)

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add wasm32-unknown-unknown
```

Run tests

```sh
cargo test
```

## Scripts Entry

### initialize

Initialize the contract.

### split

split bills.

## Publish

build

```sh
cargo wasm
```

optimize

```sh
cargo run-script optimize
```

check

```sh
cargo run-script check
```

deploy

```sh
archwayd tx wasm store ./artifacts/split-vota.wasm --gas auto --gas-prices $(archwayd q rewards estimate-fees 1 --node 'https://rpc.constantine.archway.tech:443' --output json | jq -r '.gas_unit_price | (.amount + .denom)') --gas-adjustment 1.4 --from test-key --chain-id constantine-3 --node https://rpc.constantine.archway.tech:443 --broadcast-mode sync --output json -y
```

Generate Typescript SDK

```sh
cargo schema

cosmwasm-ts-codegen generate \
--plugin client \
--schema ./schema \
--out ./ts \
--name split-vota \
--no-bundle
```
