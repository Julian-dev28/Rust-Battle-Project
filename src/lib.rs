#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, vec, Address, Env, String, Symbol, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Player(Address),
    Battle(Symbol),
    Players,
    Battles,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlayerStat {
    pub health: u32,
    pub attack: u32,
    pub defense: u32,
    pub in_battle: bool,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Battle {
    pub battle_status: u64,
    pub name: Symbol,
    pub players: Vec<Address>,
    pub moves: Vec<u64>,
    pub winner: Address,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum BattleStatus {
    Pending = 0,
    Started = 1,
    Ended = 2,
}

#[contract]
pub struct BattleContract;

#[contractimpl]
impl BattleContract {
    pub fn add_player(env: Env, user: Address) {
        user.require_auth();
        env.storage().instance().set(
            &DataKey::Player(user.clone()),
            &PlayerStat {
                health: 100,
                attack: 10,
                defense: 10,
                in_battle: false,
            },
        );

        let mut players: Vec<Address> = env
            .storage()
            .instance()
            .get(&DataKey::Players)
            .unwrap_or(Vec::new(&env));
        players.push_front(user.clone());
    }

    pub fn create_battle(env: Env, name: Symbol, user: Address) {
        user.require_auth();
        env.storage().instance().set(
            &DataKey::Battle(name.clone()),
            &Battle {
                battle_status: 0,
                name: name.clone(),
                players: vec![&env, user.clone()],
                moves: Vec::new(&env),
                winner: env.current_contract_address(),
            },
        );

        let mut battles: Vec<Symbol> = env
            .storage()
            .instance()
            .get(&DataKey::Battles)
            .unwrap_or(Vec::new(&env));

        let mut player: PlayerStat = env
            .storage()
            .instance()
            .get(&DataKey::Player(user.clone()))
            .unwrap_or(PlayerStat {
                health: 0,
                attack: 0,
                defense: 0,
                in_battle: false,
            });

        player.in_battle = true;
        battles.push_front(name.clone());
    }

    pub fn join_battle(env: Env, name: Symbol, user: Address) {
        user.require_auth();
        let mut battle: Battle = env
            .storage()
            .instance()
            .get(&DataKey::Battle(name.clone()))
            .unwrap();

        let mut player: PlayerStat = env
            .storage()
            .instance()
            .get(&DataKey::Player(user.clone()))
            .unwrap();

        assert!(battle.battle_status == 0, "Battle already started");

        battle.players.push_front(user.clone());
        battle.battle_status = 1;
        player.in_battle = true;
    }

    pub fn get_battle(env: Env, name: Symbol) -> Battle {
        env.storage()
            .instance()
            .get(&DataKey::Battle(name.clone()))
            .unwrap()
    }

    pub fn increase_health(env: Env, user: Address, incr: u32) -> u32 {
        // Get the current count.
        let mut player_stat = Self::get_player_stats(env.clone(), user.clone());

        // Increment the count.
        player_stat.health += incr;

        // Save the count.
        env.storage()
            .instance()
            .set(&DataKey::Player(user.clone()), &player_stat);

        // Return the count to the caller.
        player_stat.health
    }

    pub fn decrease_health(env: Env, user: Address, decr: u32) -> u32 {
        // Get the current count.
        let mut player_stat = Self::get_player_stats(env.clone(), user.clone());

        // Increment the count.
        player_stat.health -= decr;

        // Save the count.
        env.storage()
            .instance()
            .set(&DataKey::Player(user.clone()), &player_stat);

        // Return the count to the caller.
        player_stat.health
    }

    pub fn increase_attack(env: Env, user: Address, incr: u32) -> u32 {
        // Get the current count.
        let mut player_stat = Self::get_player_stats(env.clone(), user.clone());

        // Increment the count.
        player_stat.attack += incr;

        // Save the count.
        env.storage()
            .instance()
            .set(&DataKey::Player(user.clone()), &player_stat);

        // Return the count to the caller.
        player_stat.attack
    }

    pub fn decrease_attack(env: Env, user: Address, decr: u32) -> u32 {
        // Get the current count.
        let mut player_stat = Self::get_player_stats(env.clone(), user.clone());

        // Increment the count.
        player_stat.attack -= decr;

        // Save the count.
        env.storage()
            .instance()
            .set(&DataKey::Player(user.clone()), &player_stat);

        // Return the count to the caller.
        player_stat.attack
    }

    pub fn increase_defense(env: Env, user: Address, incr: u32) -> u32 {
        // Get the current count.
        let mut player_stat = Self::get_player_stats(env.clone(), user.clone());

        // Increment the count.
        player_stat.defense += incr;

        // Save the count.
        env.storage()
            .instance()
            .set(&DataKey::Player(user.clone()), &player_stat);

        // Return the count to the caller.
        player_stat.defense
    }

    pub fn decrease_defense(env: Env, user: Address, decr: u32) -> u32 {
        // Get the current count.
        let mut player_stat = Self::get_player_stats(env.clone(), user.clone());

        // Increment the count.
        player_stat.defense -= decr;

        // Save the count.
        env.storage()
            .instance()
            .set(&DataKey::Player(user.clone()), &player_stat);

        // Return the count to the caller.
        player_stat.defense
    }

    pub fn get_player_stats(env: Env, user: Address) -> PlayerStat {
        env.storage()
            .instance()
            .get(&DataKey::Player(user))
            .unwrap_or(PlayerStat {
                health: 0,
                attack: 0,
                defense: 0,
                in_battle: false,
            })
    }
}

#[cfg(test)]
mod test;
