#![cfg(test)]
extern crate std;

use crate::{PoseidonContract, PoseidonContractClient};
use soroban_sdk::{bytesn, vec, Env, U256};

#[test]
fn test_poseidon2_hash() {
    let env = Env::default();
    let contract_id = env.register(PoseidonContract, ());
    let client = PoseidonContractClient::new(&env, &contract_id);

    // Input: 4 identical field elements
    // This test matches barretenberg test case for hashing 4 inputs
    let input_value = U256::from_be_bytes(
        &env,
        &bytesn!(
            &env,
            0x9a807b615c4d3e2fa0b1c2d3e4f56789fedcba9876543210abcdef0123456789
        )
        .into(),
    );
    let inputs = vec![
        &env,
        input_value.clone(),
        input_value.clone(),
        input_value.clone(),
        input_value,
    ];

    // Expected output from Aztec's implementation
    let expected = U256::from_be_bytes(
        &env,
        &bytesn!(
            &env,
            0x2f43a0f83b51a6f5fc839dea0ecec74947637802a579fa9841930a25a0bcec11
        )
        .into(),
    );

    let result = client.poseidon2(&inputs);
    assert_eq!(result, expected);

    std::println!("✓ Poseidon2 hash test passed");
}

#[test]
fn test_poseidon_hash_1_2() {
    let env = Env::default();
    let contract_id = env.register(PoseidonContract, ());
    let client = PoseidonContractClient::new(&env, &contract_id);

    // Input: [1, 2]
    // This test case matches circom hash([1, 2]) with t=3
    let inputs = vec![
        &env,
        U256::from_be_bytes(
            &env,
            &bytesn!(
                &env,
                0x0000000000000000000000000000000000000000000000000000000000000001
            )
            .into(),
        ),
        U256::from_be_bytes(
            &env,
            &bytesn!(
                &env,
                0x0000000000000000000000000000000000000000000000000000000000000002
            )
            .into(),
        ),
    ];

    // Expected output: 7853200120776062878684798364095072458815029376092732009249414926327459813530
    let expected = U256::from_be_bytes(
        &env,
        &bytesn!(
            &env,
            0x115cc0f5e7d690413df64c6b9662e9cf2a3617f2743245519e19607a4417189a
        )
        .into(),
    );

    let result = client.poseidon(&inputs);
    assert_eq!(result, expected);

    std::println!("✓ Poseidon hash [1, 2] test passed");
}

#[test]
fn test_poseidon_hash_3_4() {
    let env = Env::default();
    let contract_id = env.register(PoseidonContract, ());
    let client = PoseidonContractClient::new(&env, &contract_id);

    // Input: [3, 4]
    // This test case matches circom hash([3, 4]) with t=3
    let inputs = vec![
        &env,
        U256::from_be_bytes(
            &env,
            &bytesn!(
                &env,
                0x0000000000000000000000000000000000000000000000000000000000000003
            )
            .into(),
        ),
        U256::from_be_bytes(
            &env,
            &bytesn!(
                &env,
                0x0000000000000000000000000000000000000000000000000000000000000004
            )
            .into(),
        ),
    ];

    // Expected output: 14763215145315200506921711489642608356394854266165572616578112107564877678998
    let expected = U256::from_be_bytes(
        &env,
        &bytesn!(
            &env,
            0x20a3af0435914ccd84b806164531b0cd36e37d4efb93efab76913a93e1f30996
        )
        .into(),
    );

    let result = client.poseidon(&inputs);
    assert_eq!(result, expected);

    std::println!("✓ Poseidon hash [3, 4] test passed");
}

#[test]
fn test_poseidon_and_poseidon2_different() {
    let env = Env::default();
    let contract_id = env.register(PoseidonContract, ());
    let client = PoseidonContractClient::new(&env, &contract_id);

    // Use the same input for both functions
    let inputs = vec![
        &env,
        U256::from_be_bytes(
            &env,
            &bytesn!(
                &env,
                0x0000000000000000000000000000000000000000000000000000000000000001
            )
            .into(),
        ),
        U256::from_be_bytes(
            &env,
            &bytesn!(
                &env,
                0x0000000000000000000000000000000000000000000000000000000000000002
            )
            .into(),
        ),
    ];

    let poseidon_result = client.poseidon(&inputs);
    let poseidon2_result = client.poseidon2(&inputs);

    // The two hash functions should produce different results
    assert_ne!(poseidon_result, poseidon2_result);

    std::println!("✓ Poseidon and Poseidon2 produce different outputs");
}

