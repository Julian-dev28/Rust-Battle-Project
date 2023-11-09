#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Player(Address),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct State {
    pub count: u32,
    pub last_incr: u32,
    pub last_decr: u32,
}

#[contract]
pub struct IncrementContract;

#[contractimpl]
impl IncrementContract {
    pub fn increment_enum(env: Env, user: Address, incr: u32) -> u32 {
        // Get the current count.
        let mut state = Self::get_enum(env.clone(), user.clone());

        // Increment the count.
        state.count += incr;
        state.last_incr = incr;

        // Save the count.
        env.storage()
            .instance()
            .set(&DataKey::Player(user.clone()), &state);

        // Return the count to the caller.
        state.count
    }

    pub fn decrement_enum(env: Env, user: Address, decr: u32) -> u32 {
        // Get the current count.
        let mut state = Self::get_enum(env.clone(), user.clone());

        // Increment the count.
        state.count -= decr;
        state.last_decr = decr;

        // Save the count.
        env.storage()
            .instance()
            .set(&DataKey::Player(user.clone()), &state);

        // Return the count to the caller.
        state.count
    }

    pub fn get_enum(env: Env, user: Address) -> State {
        env.storage()
            .instance()
            .get(&DataKey::Player(user))
            .unwrap_or(State {
                count: 0,
                last_incr: 0,
                last_decr: 0,
            })
    }
}

#[cfg(test)]
mod test;
