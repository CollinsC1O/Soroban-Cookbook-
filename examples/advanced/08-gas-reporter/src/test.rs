use super::*;
use soroban_sdk::{contract, contractimpl, Env};

#[contract]
struct MeteredWork;

#[contractimpl]
impl MeteredWork {
    pub fn run(env: Env) {
        for _ in 0..100 {
            let _ = env.ledger().timestamp();
        }
    }
}

#[test]
fn test_measure() {
    let env = Env::default();
    let contract_id = env.register_contract(None, MeteredWork);
    let client = MeteredWorkClient::new(&env, &contract_id);
    let count = measure(&env, |_| client.run());
    assert!(count > 0);
}