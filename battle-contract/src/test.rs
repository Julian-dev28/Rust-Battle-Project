#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};

mod battle {
    soroban_sdk::contractimport!(
        file = "../target/wasm32-unknown-unknown/release/battle.optimized.wasm"
    );
}

// Helper function to set up the testing environment
fn setup_test() -> (
    Env,
    Address,
    Address,
    Address,
    BattleContractClient<'static>,
) {
    let env = Env::default();
    env.mock_all_auths();
    env.budget().reset_unlimited();
    let contract_id = env.register_contract(None, BattleContract);
    // let contract_id: Address = env.register_contract_wasm(None, battle::WASM);
    let user_1 = Address::random(&env);
    let user_2 = Address::random(&env);
    let client = BattleContractClient::new(&env, &contract_id);
    (env, contract_id, user_1, user_2, client)
}

#[test]
fn test_enum() {
    let (_env, _contract_id, user_1, user_2, client) = setup_test();

    client.add_player(&user_1.clone());
    client.add_player(&user_2.clone());

    // User 1 increment series
    assert_eq!(client.increase_health(&user_1, &1), 101);
    assert_eq!(
        client.get_player_stats(&user_1),
        PlayerStat {
            player_address: user_1.clone(),
            sword_class: 0,
            health: 101,
            attack: 10,
            defense: 10,
            in_battle: false,
        }
    );

    assert_eq!(client.increase_health(&user_1, &2), 103);
    assert_eq!(
        client.get_player_stats(&user_1),
        PlayerStat {
            player_address: user_1.clone(),
            sword_class: 0,
            health: 103,
            attack: 10,
            defense: 10,
            in_battle: false,
        }
    );

    // User 2 increment series
    assert_eq!(client.increase_health(&user_2, &10), 110);
    assert_eq!(
        client.get_player_stats(&user_2),
        PlayerStat {
            player_address: user_2.clone(),
            sword_class: 0,
            health: 110,
            attack: 10,
            defense: 10,
            in_battle: false,
        }
    );

    assert_eq!(client.increase_health(&user_2, &10), 120);
    assert_eq!(
        client.get_player_stats(&user_2),
        PlayerStat {
            player_address: user_2.clone(),
            sword_class: 0,
            health: 120,
            attack: 10,
            defense: 10,
            in_battle: false,
        }
    );
}

#[test]
fn test_decrement() {
    let (env, _contract_id, _user_1, _user_2, client) = setup_test();
    let user_1 = Address::random(&env);
    client.add_player(&user_1);

    assert_eq!(client.increase_health(&user_1, &10), 110);
    assert_eq!(
        client.get_player_stats(&user_1),
        PlayerStat {
            player_address: user_1.clone(),
            sword_class: 0,
            health: 110,
            attack: 10,
            defense: 10,
            in_battle: false,
        }
    );

    assert_eq!(client.decrease_health(&user_1, &9), 101);
    assert_eq!(
        client.get_player_stats(&user_1),
        PlayerStat {
            player_address: user_1.clone(),
            sword_class: 0,
            health: 101,
            attack: 10,
            defense: 10,
            in_battle: false,
        }
    );
}

#[test]
fn create_player() {
    let (_env, _contract_id, user_1, _user_2, client) = setup_test();
    client.add_player(&user_1);
    assert_eq!(
        client.get_player_stats(&user_1),
        PlayerStat {
            player_address: user_1.clone(),
            sword_class: 0,
            health: 100,
            attack: 10,
            defense: 10,
            in_battle: false,
        }
    );
}

#[test]
fn create_and_join_battle() {
    let (env, contract_id, user_1, user_2, client) = setup_test();
    let battle_name = Symbol::new(&env, "Constantine");

    // Step 1: Create the battle with user_1
    client.add_player(&user_1);
    assert_eq!(
        client.create_battle(&battle_name, &user_1),
        (Ok(()), Ok(()))
    );

    // Verify the battle state after creation
    let expected_battle_after_create = Battle {
        battle_status: 0,
        name: battle_name.clone(),
        players: map![&env, (user_1.clone(), 1), (contract_id.clone(), 2)],
        moves: map![&env, (user_1.clone(), 0), (contract_id.clone(), 0)],
        winner: contract_id.clone(),
    };
    assert_eq!(
        client.get_battle(&battle_name),
        expected_battle_after_create.clone()
    );
    assert_eq!(client.get_player_stats(&user_1).in_battle, true);

    // Step 2: Join the battle with user_2

    client.add_player(&user_2.clone());
    assert_eq!(
        client.join_battle(&battle_name, &user_2.clone()),
        (Ok(()), Ok(()))
    );

    assert_eq!(client.get_player_stats(&user_2).in_battle, true);
    let player_1 = client
        .get_battle(&battle_name)
        .players
        .keys()
        .get(0)
        .unwrap();

    let expected_battle_after_join = Battle {
        battle_status: 1,
        name: battle_name.clone(),
        players: map![&env, (player_1.clone(), 1), (user_2.clone(), 2)],
        moves: map![&env, (player_1.clone(), 0), (user_2.clone(), 0)],
        winner: contract_id.clone(),
    };
    assert_eq!(
        client.get_battle(&battle_name),
        expected_battle_after_join.clone()
    );
}
