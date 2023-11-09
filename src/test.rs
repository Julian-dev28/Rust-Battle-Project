#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
fn test_enum() {
    let env = Env::default();
    let contract_id = env.register_contract(None, IncrementContract);
    let client = IncrementContractClient::new(&env, &contract_id);
    let user_1 = Address::random(&env);
    let user_2 = Address::random(&env);

    // User 1 increment series
    assert_eq!(client.increment_enum(&user_1, &1), 1);
    assert_eq!(
        client.get_enum(&user_1),
        State {
            count: 1,
            last_incr: 1,
            last_decr: 0,
        }
    );

    assert_eq!(client.increment_enum(&user_1, &2), 3);
    assert_eq!(
        client.get_enum(&user_1),
        State {
            count: 3,
            last_incr: 2,
            last_decr: 0,
        }
    );

    // User 2 increment series
    assert_eq!(client.increment_enum(&user_2, &10), 10);
    assert_eq!(
        client.get_enum(&user_2),
        State {
            count: 10,
            last_incr: 10,
            last_decr: 0,
        }
    );

    assert_eq!(client.increment_enum(&user_2, &10), 20);
    assert_eq!(
        client.get_enum(&user_2),
        State {
            count: 20,
            last_incr: 10,
            last_decr: 0,
        }
    );
}

#[test]
fn test_decrement() {
    let env = Env::default();
    let contract_id = env.register_contract(None, IncrementContract);
    let client = IncrementContractClient::new(&env, &contract_id);
    let user_1 = Address::random(&env);

    assert_eq!(client.increment_enum(&user_1, &10), 10);
    assert_eq!(
        client.get_enum(&user_1),
        State {
            count: 10,
            last_incr: 10,
            last_decr: 0,
        }
    );

    assert_eq!(client.decrement_enum(&user_1, &9), 1);
    assert_eq!(
        client.get_enum(&user_1),
        State {
            count: 1,
            last_incr: 10,
            last_decr: 9,
        }
    );
}
