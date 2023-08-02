# Test contract based on polkadot

## Usage

### Set up Polkadot network [ local network ]

Step 1. Download substrate-contracts-node , url : https://github.com/paritytech/substrate-contracts-node/releases/tag/v0.29.0

Step 2. Run the contract node under dev mode and allow external rpc

```sh
./substrate-contracts-node -dev --rpc-external
```

**_NOTE:_** Reference https://github.com/paritytech/substrate-contracts-node

### Set up contracu-ui

Step 1. Clone from repo and modify localnetwork ip what you want

Step 2. Work under project path and yarn start

**_NOTE:_** Reference https://github.com/paritytech/contracts-ui

### Build contract

**_NOTE:_** The environment dependence was written on Cargo.toml

```sh
cargo contract build
```

### Deploy contract

Step 1. Vist contract-ui and upload contract file which is including .contract attachment name such as "whitelist_contract.contract"
