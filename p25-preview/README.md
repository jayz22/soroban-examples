# Protocol 25 Preview Examples

Demonstrates Protocol 25 preview features in Soroban smart contracts:
- **BN254 elliptic curve operations** - point addition, scalar multiplication, pairing checks
- **Poseidon and Poseidon2 hash functions** - ZK-proof friendly hashing

Two workflows:
1. Deploy to Stellar Futurenet (recommended)
2. Run local Protocol 25 network (experimental)

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
│       │   ├── lib.rs    # Poseidon contract implementation, contains both Poseidon and Poseidon2 hash functions
│       │   └── test.rs   # Unit tests
│       └── Cargo.toml
├── Cargo.toml
└── README.md
```

## Building and Testing

**Build:**
```bash
cargo build --workspace --target wasm32v1-none --release
```
Or you can build individual contracts by `cd` and `make build` into its subdirectory

**Test:**
```bash
cargo test --workspace
```
Or you can test individual contracts by `cd` and `make test` into its subdirectory

## Encoding

**BN254 (uncompressed, big-endian):**
- G1 points: 64 bytes (32-byte x, 32-byte y)
- G2 points: 128 bytes (64-byte x, 64-byte y)
- Fr (scalar): U256/32 bytes

**Poseidon:** Works on BN254 scalar field (Fr)

Details: https://github.com/stellar/rs-soroban-sdk/blob/release/v25-preview/soroban-sdk/src/crypto/bn254.rs

---

## Workflow 1: Deploy to Futurenet (Recommended)

### Setup

**1. Run quickstart:**
```bash
docker run --rm -i --name stellar -p 8000:8000 stellar/quickstart:future --futurenet
```

**2. Configure network (temporary workaround for futurenet RPC):**
```bash
stellar network add futurenet \
  --rpc-url http://localhost:8000/soroban/rpc \
  --network-passphrase "Test SDF Future Network ; October 2022"
```

**3. Verify network health:**
```bash
stellar network health --network futurenet
```
Output:
```
✅ Healthy
ℹ️  Latest ledger: 1776379
```

**4. Create and fund account:**
```bash
stellar keys generate mykey
stellar keys fund mykey
stellar keys use mykey  # Set as default
```
Output:
```
✅ Account mykey funded on "Test SDF Future Network ; October 2022"
```

### Deploy Contracts

**Poseidon:**
```bash
stellar contract deploy \
  --wasm target/wasm32v1-none/release/poseidon.optimized.wasm \
  --alias poseidon \
  --network futurenet
```
Output:
```
ℹ️  Simulating install transaction…
ℹ️  Signing transaction: 0979c508e44f0732ec35d6e2f312bc503c0728581204124a99890e0051594762
🌎 Submitting install transaction…
ℹ️  Using wasm hash 28606db9cf9b023d7b542d1e1975fd4a6f557b656844657df0d8367ba61b9245
ℹ️  Simulating deploy transaction…
ℹ️  Transaction hash is 5010a291850c9351359d72b8b0024d39d713763368ecec4699787ae206f8c2a9
ℹ️  Signing transaction: 5010a291850c9351359d72b8b0024d39d713763368ecec4699787ae206f8c2a9
🌎 Submitting deploy transaction…
✅ Deployed!
CCJJ26NSVC676D43XY5UYYNSLGY5P3OLWFCMBZGLIYZZ6SNF2LOASKOQ
```

**BN254:**
```bash
stellar contract deploy \
  --wasm target/wasm32v1-none/release/bn254.wasm \
  --alias bn254 \
  --network futurenet
```
Output:
```
ℹ️  Simulating install transaction…
ℹ️  Signing transaction: 57f95811737427a66c245d07e6d824dc5e78fdecfcb7768afaf50c00c6fd06cd
🌎 Submitting install transaction…
ℹ️  Using wasm hash 9ead5ebfb25bcab9b3c5328bb9b6c685d2daa20b21f69642b3223a61a680a5ac
ℹ️  Simulating deploy transaction…
ℹ️  Transaction hash is 4b235b69a47f8c14bb323fabe6134600bbfb8bd96968d67f6d1966ddb6e8e569
ℹ️  Signing transaction: 4b235b69a47f8c14bb323fabe6134600bbfb8bd96968d67f6d1966ddb6e8e569
🌎 Submitting deploy transaction…
✅ Deployed!
CDIYWSYLOG23U3DATSCKASC65WQDIJ5NXJ55MEDCPXUXFHBQ5TZR46AT
```


### Invoke Contracts

**Poseidon:**
```bash
stellar contract invoke \
  --id poseidon \
  --network futurenet \
  --send=yes \
  -- poseidon --inputs '["3", "4"]'
```
Output:
```
ℹ️  Signing transaction: 95799a42411b1269067e9064d4d129fe9de66ee9c72daebb77b84488e5c2f40e
"14763215145315200506921711489642608356394854266165572616578112107564877678998"
```

**BN254:**
```bash
stellar contract invoke \
  --id bn254 \
  --network futurenet \
  --send=yes \
  -- g1_add \
  --a 9a8bcafe92edd791297fee8ae890df72d075e6a8b37a3d9e474dda75a936be02d7128504ee96b037c076b9a283511bdc82e5b826c08fad6cbd18e941f2fc5ca8 \
  --b 9a8bcafe92edd791297fee8ae890df72d075e6a8b37a3d9e474dda75a936be02d7128504ee96b037c076b9a283511bdc82e5b826c08fad6cbd18e941f2fc5ca8
```
Output:
```
ℹ️  Signing transaction: 39e249b261c412e254a91c69af7dcc68b14cc515b61642130ea5ac010effb35c
"82d22f1a8fe791d291794afbff453e9e9e9d0a3661ecf04f0f44553895f2320315eb965ec4fa8e9044f27a2ca3455ca65dda88e5549828a4319b913d8dd9c09f"
```


### Fetch Transaction Details

**Poseidon example:**

Full transaction details:
```bash
stellar tx fetch --hash 95799a42411b1269067e9064d4d129fe9de66ee9c72daebb77b84488e5c2f40e --network futurenet
```
Output:
```json
{"tx":{"tx":{"source_account":"GBYKYERAJBF7NBOZWS6CBLXVDAXNN652ZMGRJWVI75VGNOQIWRMHTH5A","fee":60978,"seq_num":"7629584199581699","cond":"none","memo":"none","operations":[{"source_account":null,"body":{"invoke_host_function":{"host_function":{"invoke_contract":{"contract_address":"CCJJ26NSVC676D43XY5UYYNSLGY5P3OLWFCMBZGLIYZZ6SNF2LOASKOQ","function_name":"poseidon","args":[{"vec":[{"u256":"3"},{"u256":"4"}]}]}},"auth":[]}}}],"ext":{"v1":{"ext":"v0","resources":{"footprint":{"read_only":[{"contract_data":{"contract":"CCJJ26NSVC676D43XY5UYYNSLGY5P3OLWFCMBZGLIYZZ6SNF2LOASKOQ","key":"ledger_key_contract_instance","durability":"persistent"}},{"contract_code":{"hash":"28606db9cf9b023d7b542d1e1975fd4a6f557b656844657df0d8367ba61b9245"}}],"read_write":[]},"instructions":3925046,"disk_read_bytes":0,"write_bytes":0},"resource_fee":"60878"}}},"signatures":[{"hint":"08b45879","signature":"38c01171a8c1ccd9ddd271df1a5f751202efd6dad8387b145a0dbfe50ec74ec8df5529be51a6637cc81e70cd527e729ab1ec7f4ce8521dd2087fa0401e01ae03"}]}}
```

Transaction result only:
```bash
stellar tx fetch result --hash 95799a42411b1269067e9064d4d129fe9de66ee9c72daebb77b84488e5c2f40e --network futurenet
```
Output:
```json
{"fee_charged":"22208","result":{"tx_success":[{"op_inner":{"invoke_host_function":{"success":"1d445fd24aff3f260b0ef8f5a1f71145e80601451b11f7350400a52a521e81db"}}}]},"ext":"v0"}
```

Transaction metadata with events:
```bash
stellar tx fetch meta --hash 95799a42411b1269067e9064d4d129fe9de66ee9c72daebb77b84488e5c2f40e --network futurenet
```
Output:
```json
{"v4":{"ext":"v0","tx_changes_before":[{"state":{"last_modified_ledger_seq":1776630,"data":{"account":{"account_id":"GBYKYERAJBF7NBOZWS6CBLXVDAXNN652ZMGRJWVI75VGNOQIWRMHTH5A","balance":"99997438234","seq_num":"7629584199581698","num_sub_entries":0,"inflation_dest":null,"flags":0,"home_domain":"","thresholds":"01000000","signers":[],"ext":{"v1":{"liabilities":{"buying":"0","selling":"0"},"ext":{"v2":{"num_sponsored":0,"num_sponsoring":0,"signer_sponsoring_i_ds":[],"ext":{"v3":{"ext":"v0","seq_ledger":1776410,"seq_time":"1763404026"}}}}}}}},"ext":"v0"}},{"updated":{"last_modified_ledger_seq":1776630,"data":{"account":{"account_id":"GBYKYERAJBF7NBOZWS6CBLXVDAXNN652ZMGRJWVI75VGNOQIWRMHTH5A","balance":"99997438234","seq_num":"7629584199581699","num_sub_entries":0,"inflation_dest":null,"flags":0,"home_domain":"","thresholds":"01000000","signers":[],"ext":{"v1":{"liabilities":{"buying":"0","selling":"0"},"ext":{"v2":{"num_sponsored":0,"num_sponsoring":0,"signer_sponsoring_i_ds":[],"ext":{"v3":{"ext":"v0","seq_ledger":1776630,"seq_time":"1763405127"}}}}}}}},"ext":"v0"}}],"operations":[{"ext":"v0","changes":[],"events":[]}],"tx_changes_after":[],"soroban_meta":{"ext":{"v1":{"ext":"v0","total_non_refundable_resource_fee_charged":"21756","total_refundable_resource_fee_charged":"352","rent_fee_charged":"0"}},"return_value":{"u256":"14763215145315200506921711489642608356394854266165572616578112107564877678998"}},"events":[{"stage":"before_all_txs","event":{"ext":"v0","contract_id":"CB64D3G7SM2RTH6JSGG34DDTFTQ5CFDKVDZJZSODMCX4NJ2HV2KN7OHT","type_":"contract","body":{"v0":{"topics":[{"symbol":"fee"},{"address":"GBYKYERAJBF7NBOZWS6CBLXVDAXNN652ZMGRJWVI75VGNOQIWRMHTH5A"}],"data":{"i128":"60978"}}}}},{"stage":"after_all_txs","event":{"ext":"v0","contract_id":"CB64D3G7SM2RTH6JSGG34DDTFTQ5CFDKVDZJZSODMCX4NJ2HV2KN7OHT","type_":"contract","body":{"v0":{"topics":[{"symbol":"fee"},{"address":"GBYKYERAJBF7NBOZWS6CBLXVDAXNN652ZMGRJWVI75VGNOQIWRMHTH5A"}],"data":{"i128":"-38770"}}}}}],"diagnostic_events":[]}}
```

**BN254 example:**

Full transaction details:
```bash
stellar tx fetch --hash 39e249b261c412e254a91c69af7dcc68b14cc515b61642130ea5ac010effb35c --network futurenet
```
Output:
```json
{"tx":{"tx":{"source_account":"GBYKYERAJBF7NBOZWS6CBLXVDAXNN652ZMGRJWVI75VGNOQIWRMHTH5A","fee":53989,"seq_num":"7629584199581702","cond":"none","memo":"none","operations":[{"source_account":null,"body":{"invoke_host_function":{"host_function":{"invoke_contract":{"contract_address":"CDIYWSYLOG23U3DATSCKASC65WQDIJ5NXJ55MEDCPXUXFHBQ5TZR46AT","function_name":"g1_add","args":[{"bytes":"9a8bcafe92edd791297fee8ae890df72d075e6a8b37a3d9e474dda75a936be02d7128504ee96b037c076b9a283511bdc82e5b826c08fad6cbd18e941f2fc5ca8"},{"bytes":"9a8bcafe92edd791297fee8ae890df72d075e6a8b37a3d9e474dda75a936be02d7128504ee96b037c076b9a283511bdc82e5b826c08fad6cbd18e941f2fc5ca8"}]}},"auth":[]}}}],"ext":{"v1":{"ext":"v0","resources":{"footprint":{"read_only":[{"contract_data":{"contract":"CDIYWSYLOG23U3DATSCKASC65WQDIJ5NXJ55MEDCPXUXFHBQ5TZR46AT","key":"ledger_key_contract_instance","durability":"persistent"}},{"contract_code":{"hash":"9ead5ebfb25bcab9b3c5328bb9b6c685d2daa20b21f69642b3223a61a680a5ac"}}],"read_write":[]},"instructions":548875,"disk_read_bytes":0,"write_bytes":0},"resource_fee":"53889"}}},"signatures":[{"hint":"08b45879","signature":"9e15d11b8021e1cd8029284ca819066b435f29f3a8bb347215441f49a176eb2aafbc9b5c3a6c65982808cee1cb2c4e55abb15e870ce48a4973a8ac6ee4505505"}]}}
```


---

## Workflow 2: Local Standalone Network (Experimental)

Run your own standalone Protocol 25 network locally. This is experimental. Use Workflow 1 (Futurenet) if possible.

### Prerequisites

Follow this guide to set up a local Protocol 25 network:
https://gist.github.com/leighmcculloch/3c4fc1593fa77c0442843ecfba15ea9b

The guide covers:
- Setting up a local Stellar network with Protocol 25
- Creating and funding accounts
- Deploying contracts

### Deploy Contracts

After completing the setup guide:

```bash
# Deploy BN254
stellar contract deploy \
  --wasm target/wasm32v1-none/release/bn254.wasm \
  --alias bn254

# Deploy Poseidon
stellar contract deploy \
  --wasm target/wasm32v1-none/release/poseidon.wasm \
  --alias poseidon
```

---

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

### Invoking Poseidon Function

Hash two field elements `[1, 2]` with Poseidon:

```bash
stellar contract invoke --id poseidon -- poseidon \
  --inputs '[{"u256":"1"},{"u256":"2"}]'
```

**Expected output:**
```
"7853200120776062878684798364095072458815029376092732009249414926327459813530"
```

### Invoking Poseidon2 Function
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


## Notes

- Arguments are passed as raw bytes (`BytesN<64>`, `BytesN<128>`, `U256`) and converted to BN254 types inside the contract
- Future versions will add spec support for BN254 types, allowing direct type passing
