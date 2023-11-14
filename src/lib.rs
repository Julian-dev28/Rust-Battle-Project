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
    pub player_address: Address,
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
                player_address: user.clone(),
                health: 100,
                attack: 10,
                defense: 10,
                in_battle: false,
            },
        );

        let mut players: Vec<Address> = Self::get_players(env.clone());
        players.push_back(user.clone());
        Self::set_players(env.clone(), players);
    }

    pub fn get_players(env: Env) -> Vec<Address> {
        env.storage()
            .instance()
            .get(&DataKey::Players)
            .unwrap_or(Vec::new(&env))
    }

    pub fn set_players(env: Env, players: Vec<Address>) {
        env.storage().instance().set(&DataKey::Players, &players);
    }

    pub fn get_player_stats(env: Env, user: Address) -> PlayerStat {
        env.storage()
            .instance()
            .get(&DataKey::Player(user))
            .unwrap_or(PlayerStat {
                player_address: env.current_contract_address(),
                health: 0,
                attack: 0,
                defense: 0,
                in_battle: false,
            })
    }

    pub fn set_player_stats(env: Env, user: Address, player_stat: PlayerStat) {
        env.storage()
            .instance()
            .set(&DataKey::Player(user), &player_stat);
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

        let mut player = Self::get_player_stats(env.clone(), user.clone());
        assert!(!player.in_battle, "Player already in battle");
        player.in_battle = true;
        Self::set_player_stats(env.clone(), user.clone(), player);

        let mut battles = Self::get_battles(env.clone());
        battles.push_back(name.clone());
        Self::set_battles(env.clone(), battles);
    }

    pub fn create_auto_battle(env: Env, name: Symbol, user: Address) {
        user.require_auth();
        let contract_id = env.current_contract_address();
        env.storage().instance().set(
            &DataKey::Battle(name.clone()),
            &Battle {
                battle_status: 1,
                name: name.clone(),
                players: vec![&env, user.clone(), contract_id.clone()],
                moves: Vec::new(&env),
                winner: contract_id.clone(),
            },
        );

        let mut battles = Self::get_battles(env.clone());
        battles.push_back(name.clone());
        Self::set_battles(env.clone(), battles);
    }

    pub fn join_battle(env: Env, name: Symbol, user: Address) {
        user.require_auth();
        let mut battle = Self::get_battle(env.clone(), name.clone());
        assert!(battle.battle_status == 0, "Battle already started");
        let mut player = Self::get_player_stats(env.clone(), user.clone());
        assert!(!player.in_battle, "Player already in battle");

        battle.players.push_back(user.clone());
        battle.battle_status = 1;
        player.in_battle = true;

        Self::set_battle(env.clone(), name.clone(), battle);
        Self::set_player_stats(env.clone(), user.clone(), player);
    }

    pub fn challenge_bot(env: Env, name: Symbol) {
        let mut battle = Self::get_battle(env.clone(), name.clone());
        assert!(battle.battle_status == 0, "Battle already started");
        let contract_id = env.current_contract_address();

        battle.players.push_back(contract_id.clone());
        battle.battle_status = 1;

        Self::set_battle(env.clone(), name.clone(), battle);
    }

    pub fn get_battle(env: Env, name: Symbol) -> Battle {
        env.storage()
            .instance()
            .get(&DataKey::Battle(name.clone()))
            .unwrap_or(Battle {
                battle_status: 0,
                name: name.clone(),
                players: Vec::new(&env),
                moves: Vec::new(&env),
                winner: env.current_contract_address(),
            })
    }

    pub fn set_battle(env: Env, name: Symbol, battle: Battle) {
        env.storage()
            .instance()
            .set(&DataKey::Battle(name.clone()), &battle);
    }

    pub fn get_battles(env: Env) -> Vec<Symbol> {
        env.storage()
            .instance()
            .get(&DataKey::Battles)
            .unwrap_or(Vec::new(&env))
    }

    pub fn set_battles(env: Env, battles: Vec<Symbol>) {
        env.storage().instance().set(&DataKey::Battles, &battles);
    }

    pub fn increase_health(env: Env, user: Address, incr: u32) -> u32 {
        // Get the current count.
        let mut player_stat = Self::get_player_stats(env.clone(), user.clone());

        // Increment the count.
        player_stat.health += incr;

        // Save the count.
        Self::set_player_stats(env.clone(), user.clone(), player_stat.clone());

        // Return the count to the caller.
        player_stat.health
    }

    pub fn decrease_health(env: Env, user: Address, decr: u32) -> u32 {
        // Get the current count.
        let mut player_stat = Self::get_player_stats(env.clone(), user.clone());

        // Increment the count.
        player_stat.health -= decr;

        // Save the count.
        Self::set_player_stats(env.clone(), user.clone(), player_stat.clone());

        // Return the count to the caller.
        player_stat.health
    }

    pub fn increase_attack(env: Env, user: Address, incr: u32) -> u32 {
        // Get the current count.
        let mut player_stat = Self::get_player_stats(env.clone(), user.clone());

        // Increment the count.
        player_stat.attack += incr;

        // Save the count.
        Self::set_player_stats(env.clone(), user.clone(), player_stat.clone());

        // Return the count to the caller.
        player_stat.attack
    }

    pub fn decrease_attack(env: Env, user: Address, decr: u32) -> u32 {
        // Get the current count.
        let mut player_stat = Self::get_player_stats(env.clone(), user.clone());

        // Increment the count.
        player_stat.attack -= decr;

        // Save the count.
        Self::set_player_stats(env.clone(), user.clone(), player_stat.clone());

        // Return the count to the caller.
        player_stat.attack
    }

    pub fn increase_defense(env: Env, user: Address, incr: u32) -> u32 {
        // Get the current count.
        let mut player_stat = Self::get_player_stats(env.clone(), user.clone());

        // Increment the count.
        player_stat.defense += incr;

        // Save the count.
        Self::set_player_stats(env.clone(), user.clone(), player_stat.clone());

        // Return the count to the caller.
        player_stat.defense
    }

    pub fn decrease_defense(env: Env, user: Address, decr: u32) -> u32 {
        // Get the current count.
        let mut player_stat = Self::get_player_stats(env.clone(), user.clone());

        // Increment the count.
        player_stat.defense -= decr;

        // Save the count.
        Self::set_player_stats(env.clone(), user.clone(), player_stat.clone());

        // Return the count to the caller.
        player_stat.defense
    }
}

#[cfg(test)]
mod test;
