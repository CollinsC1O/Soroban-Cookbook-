#![cfg_attr(not(test), no_std)]

use soroban_sdk::Env;

pub fn measure<F>(env: &Env, f: F) -> u64
where
    F: FnOnce(&Env),
{
    let mut budget = env.cost_estimate().budget();
    budget.reset_tracker();
    f(env);
    budget.cpu_instruction_cost()
}
#[cfg(test)]
mod test;