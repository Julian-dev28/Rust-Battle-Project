#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
fn test_enum() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, BattleContract);
    let client = BattleContractClient::new(&env, &contract_id);
    let user_1 = Address::random(&env);
    let user_2 = Address::random(&env);
    client.add_player(&user_1.clone());
    client.add_player(&user_2.clone());
    // User 1 increment series
    assert_eq!(client.increase_health(&user_1, &1), 101);
    assert_eq!(
        client.get_player_stats(&user_1),
        PlayerStat {
            player_address: user_1.clone(),
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
            health: 120,
            attack: 10,
            defense: 10,
            in_battle: false,
        }
    );
}

#[test]
fn test_decrement() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, BattleContract);
    let client = BattleContractClient::new(&env, &contract_id);
    let user_1 = Address::random(&env);
    client.add_player(&user_1);

    assert_eq!(client.increase_health(&user_1, &10), 110);
    assert_eq!(
        client.get_player_stats(&user_1),
        PlayerStat {
            player_address: user_1.clone(),
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
            health: 101,
            attack: 10,
            defense: 10,
            in_battle: false,
        }
    );
}

#[test]
fn test_create_and_join_battle() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, BattleContract);
    let client = BattleContractClient::new(&env, &contract_id);
    let battle_name = Symbol::new(&env, "Constantine");
    let user_1 = Address::random(&env);
    let user_2 = Address::random(&env);

    client.create_battle(&battle_name, &user_1);
    assert_eq!(
        client.get_battle(&battle_name),
        Battle {
            battle_status: 0,
            name: battle_name.clone(),
            players: vec![&env, user_1.clone()],
            moves: Vec::new(&env),
            winner: contract_id.clone(),
        }
    );
    assert_eq!(client.get_player_stats(&user_1).in_battle, true);

    client.join_battle(&battle_name, &user_2);
    assert_eq!(
        client.get_battle(&battle_name),
        Battle {
            battle_status: 1,
            name: battle_name.clone(),
            players: vec![&env, user_1.clone(), user_2.clone()],
            moves: Vec::new(&env),
            winner: contract_id.clone(),
        }
    );
}
