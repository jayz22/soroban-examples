#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Symbol, U256, Vec};

#[contract]
pub struct PoseidonContract;

#[contractimpl]
impl PoseidonContract {
    /// Computes the Poseidon hash of the input field elements.
    /// Uses the BN254 field by default.
    pub fn poseidon(env: Env, inputs: Vec<U256>) -> U256 {
        let field = Symbol::new(&env, "BN254");
        env.crypto().poseidon_hash(&inputs, field)
    }

    /// Computes the Poseidon2 hash of the input field elements.
    /// Uses the BN254 field by default.
    pub fn poseidon2(env: Env, inputs: Vec<U256>) -> U256 {
        let field = Symbol::new(&env, "BN254");
        env.crypto().poseidon2_hash(&inputs, field)
    }
}

#[cfg(test)]
mod test;

