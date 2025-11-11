#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype,
    crypto::bn254::{Fr, G1Affine, G2Affine},
    Env, Vec,
};

#[derive(Clone)]
#[contracttype]
pub struct MockProof {
    pub g1: Vec<G1Affine>,
    pub g2: Vec<G2Affine>,
}

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn verify_pairing(env: Env, proof: MockProof) -> bool {
        env.crypto().bn254().pairing_check(proof.g1, proof.g2)
    }

    pub fn g1_add(a: G1Affine, b: G1Affine) -> G1Affine {
        a + b
    }

    pub fn g1_mul(p: G1Affine, s: Fr) -> G1Affine {
        p * s
    }
}

#[cfg(test)]
mod test;
