# P25 Preview Examples

This repository demonstrates how to use Protocol 25 preview features in Soroban smart contracts, including:
- **BN254 elliptic curve operations** (point addition, scalar multiplication, pairing checks)
- **Poseidon and Poseidon2 hash functions**

## Project Structure

This repository uses the recommended structure for a Soroban project:
```text
.
├── contracts
│   ├── bn254
│   │   ├── src
│   │   │   ├── lib.rs    # BN254 contract implementation
│   │   │   └── test.rs   # Unit tests
│   │   └── Cargo.toml
│   └── poseidon
│       ├── src
│       │   ├── lib.rs    # Poseidon contract implementation
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

## Building the Contracts

Build all contracts from the workspace root:

```bash
cargo build --workspace --target wasm32v1-none --release
```

Or build individual contracts:

```bash
cd contracts/bn254
make build

cd contracts/poseidon
make build
```

## Running Tests

Run all tests:

```bash
cargo test --workspace
```

Or test individual contracts:

```bash
cd contracts/bn254
cargo test

cd contracts/poseidon
cargo test
```

## Deploying the Contracts

After following the Protocol 25 setup guide, deploy the contracts:

```bash
# Deploy BN254 contract
stellar contract deploy \
  --wasm target/wasm32v1-none/release/bn254.wasm \
  --alias bn254

# Deploy Poseidon contract
stellar contract deploy \
  --wasm target/wasm32v1-none/release/poseidon.wasm \
  --alias poseidon
```

---

## BN254 Contract

### Invoking BN254 Functions

Here's an example of invoking the `g1_add` function, which adds two G1 points on the BN254 curve:

```bash
stellar contract invoke --id bn254 -- g1_add \
  --a 9a8bcafe92edd791297fee8ae890df72d075e6a8b37a3d9e474dda75a936be02d7128504ee96b037c076b9a283511bdc82e5b826c08fad6cbd18e941f2fc5ca8 \
  --b 9a8bcafe92edd791297fee8ae890df72d075e6a8b37a3d9e474dda75a936be02d7128504ee96b037c076b9a283511bdc82e5b826c08fad6cbd18e941f2fc5ca8
```

**Parameters:**
- `a`: BytesN<64> - First G1 point (uncompressed, 64 bytes)
- `b`: BytesN<64> - Second G1 point (uncompressed, 64 bytes)

**Returns:** BytesN<64> - The sum of the two points

---

## Poseidon Contract

The Poseidon contract provides two hash functions optimized for zero-knowledge proof systems:
- `poseidon` - Original Poseidon hash (compatible with circom/iden3)
- `poseidon2` - Poseidon2 hash (compatible with Aztec's barretenberg)

### Invoking Poseidon Hash

Hash two field elements `[1, 2]`:

```bash
stellar contract invoke --id poseidon -- poseidon \
  --inputs '[{"u256":"1"},{"u256":"2"}]'
```

**Expected output:**
```
"7853200120776062878684798364095072458815029376092732009249414926327459813530"
```

### Invoking Poseidon2 Hash
```bash
stellar contract invoke --id poseidon --source alice -- poseidon2 \
  --inputs '["1", "2"]'
```

**Expected output:**
```
"1594597865669602199208529098208508950092942746041644072252494753744672355203"
```

**Parameters:**
- `inputs`: Vec<U256> - Array of field elements to hash

**Returns:** U256 - The hash output

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
