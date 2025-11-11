#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, crypto::bn254::{Fr, G1Affine, G2Affine}, BytesN, Env, Vec, U256
};

#[derive(Clone)]
#[contracttype]
pub struct MockProof {
    pub g1: Vec<BytesN<64>>,
    pub g2: Vec<BytesN<128>>,
}

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn verify_pairing(env: Env, proof: MockProof) -> bool {
        let mut g1_points = Vec::new(&env);
        for bytes in proof.g1.iter() {
            g1_points.push_back(G1Affine::from_bytes(bytes));
        }
        
        let mut g2_points = Vec::new(&env);
        for bytes in proof.g2.iter() {
            g2_points.push_back(G2Affine::from_bytes(bytes));
        }
        
        env.crypto().bn254().pairing_check(g1_points, g2_points)
    }

    pub fn g1_add(a: BytesN<64>, b: BytesN<64>) -> BytesN<64> {
        let a = G1Affine::from_bytes(a);
        let b = G1Affine::from_bytes(b);
        (a + b).to_bytes()
    }

    pub fn g1_mul(p: BytesN<64>, s: U256) -> BytesN<64> {
        let p = G1Affine::from_bytes(p);
        let s = Fr::from(s);
        (p * s).to_bytes()
    }
}

#[cfg(test)]
mod test;
