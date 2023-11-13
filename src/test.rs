#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
fn test_enum() {
    let env = Env::default();
    let contract_id = env.register_contract(None, BattleContract);
    let client = BattleContractClient::new(&env, &contract_id);
    let user_1 = Address::random(&env);
    let user_2 = Address::random(&env);

    // User 1 increment series
    assert_eq!(client.increase_health(&user_1, &1), 1);
    assert_eq!(
        client.get_player_stats(&user_1),
        PlayerStat {
            health: 1,
            attack: 0,
            defense: 0,
            in_battle: false,
        }
    );

    assert_eq!(client.increase_health(&user_1, &2), 3);
    assert_eq!(
        client.get_player_stats(&user_1),
        PlayerStat {
            health: 3,
            attack: 0,
            defense: 0,
            in_battle: false,
        }
    );

    // User 2 increment series
    assert_eq!(client.increase_health(&user_2, &10), 10);
    assert_eq!(
        client.get_player_stats(&user_2),
        PlayerStat {
            health: 10,
            attack: 0,
            defense: 0,
            in_battle: false,
        }
    );

    assert_eq!(client.increase_health(&user_2, &10), 20);
    assert_eq!(
        client.get_player_stats(&user_2),
        PlayerStat {
            health: 20,
            attack: 0,
            defense: 0,
            in_battle: false,
        }
    );
}

#[test]
fn test_decrement() {
    let env = Env::default();
    let contract_id = env.register_contract(None, BattleContract);
    let client = BattleContractClient::new(&env, &contract_id);
    let user_1 = Address::random(&env);

    assert_eq!(client.increase_health(&user_1, &10), 10);
    assert_eq!(
        client.get_player_stats(&user_1),
        PlayerStat {
            health: 10,
            attack: 0,
            defense: 0,
            in_battle: false,
        }
    );

    assert_eq!(client.decrease_health(&user_1, &9), 1);
    assert_eq!(
        client.get_player_stats(&user_1),
        PlayerStat {
            health: 1,
            attack: 0,
            defense: 0,
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

    // assert_eq!(client.increase_health(&user_2, &10), 10);
    // assert_eq!(
    //     client.get_player_stats(&user_2),
    //     PlayerStat {
    //         health: 10,
    //         attack: 0,
    //         defense: 0,
    //         in_battle: false,
    //     }
    // );

    // assert_eq!(client.decrease_health(&user_1, &9), 1);
    // assert_eq!(
    //     client.get_player_stats(&user_1),
    //     PlayerStat {
    //         health: 1,
    //         attack: 0,
    //         defense: 0,
    //         in_battle: false,
    //     }
    // );

    // assert_eq!(client.decrease_health(&user_2, &9), 1);
    // assert_eq!(
    //     client.get_player_stats(&user_2),
    //     PlayerStat {
    //         health: 1,
    //         attack: 0,
    //         defense: 0,
    //         in_battle: false,
    //     }
    // );
}
