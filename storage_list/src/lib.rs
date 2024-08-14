#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Vec, Bytes, Val, FromVal};

#[derive(Clone, Debug)]
#[contracttype]
pub enum DataKey {
    List,
}

#[contracttype]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ProposalStatus {
    /// The proposal exists and voting has not been closed
    Open = 0,
    /// The proposal was voted for. If the proposal is executable, the timelock begins once this state is reached.
    Successful = 1,
    /// The proposal was voted against
    Defeated = 2,
    /// The proposal did not reach quorum before the voting period ended, or was stalled out during the grace period.
    Expired = 3,
    /// The proposal has been executed
    Executed = 4,
    /// The proposal has been canceled
    Canceled = 5,
}

#[contract]
pub struct Test;

#[contractimpl]
impl Test {
    pub fn set_list(env: Env, list: Vec<u32>) {
        env.storage().persistent().set(&DataKey::List, &list);
    }

    pub fn get_list(env: Env) -> Vec<u32> {
        env.storage()
            .persistent()
            .get(&DataKey::List)
            .unwrap_or(Vec::new(&env))
    }

    pub fn forgety1(env: Env) -> u32 {
        let val: Val = Val::from_void().to_val();
        let b = Bytes::from_val(&env, &val);
        b.get(0).unwrap() as u32
    }
}