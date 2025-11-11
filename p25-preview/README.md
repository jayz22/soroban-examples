# P25 Preview Examples

This example demonstrates how to use P25 preview features in Soroban smart contracts, including bn254 and Poseidon (soon to come).

## Project Structure

This repository uses the recommended structure for a Soroban project:
```text
.
├── contracts
│   └── bn254
│       ├── src
│       │   ├── lib.rs    # Contract implementation
│       │   └── test.rs   # Unit tests
│       └── Cargo.toml
├── Cargo.toml
└── README.md
```

## Prerequisites

To use Protocol 25 preview features, please refer to:
https://gist.github.com/leighmcculloch/3c4fc1593fa77c0442843ecfba15ea9b

This guide covers:
- Setting up a local Stellar network with Protocol 25
- Creating and funding accounts
- Deploying contracts

## Building the Contract

Build the contract from the `contracts/bn254` directory:

```bash
cd contracts/bn254
make build
```

Or using cargo directly:

```bash
cargo build --target wasm32v1-none --release
```

## Running Tests

Run the unit tests:

```bash
cd contracts/bn254
cargo test
```

## Deploying the Contract

After following the Protocol 25 setup guide, deploy the contract:

```bash
stellar contract deploy \
  --wasm target/wasm32v1-none/release/bn254.wasm \
  --alias bn254
```

## Invoking Contract Functions

Here's an example of invoking the `g1_add` function, which adds two G1 points on the BN254 curve:

```bash
stellar contract invoke --id bn254 -- g1_add \
  --a 9a8bcafe92edd791297fee8ae890df72d075e6a8b37a3d9e474dda75a936be02d7128504ee96b037c076b9a283511bdc82e5b826c08fad6cbd18e941f2fc5ca8 \
  --b 9a8bcafe92edd791297fee8ae890df72d075e6a8b37a3d9e474dda75a936be02d7128504ee96b037c076b9a283511bdc82e5b826c08fad6cbd18e941f2fc5ca8
```

The function takes two parameters:
- `a`: BytesN<64> - First G1 point (uncompressed, 64 bytes)
- `b`: BytesN<64> - Second G1 point (uncompressed, 64 bytes)

And returns BytesN<64> - The sum of the two points.

## Encoding Rules

All BN254 points are encoded in **uncompressed format**:

- **G1 points**: 64 bytes (32 bytes x-coordinate + 32 bytes y-coordinate)
- **G2 points**: 128 bytes (64 bytes x-coordinate + 64 bytes y-coordinate)
- **Fr**: `U256`/32-bytes

All coordinates are in **big-endian** byte order.

For detailed encoding specifications, see docs in:
https://github.com/stellar/rs-soroban-sdk/blob/release/v25-preview/soroban-sdk/src/crypto/bn254.rs

## Notes

- Arguments are passed as raw bytes (`BytesN<64>`, `BytesN<128>`, `U256`) and converted to BN254 types inside the contract
- Future versions will add spec support for BN254 types, allowing direct type passing
