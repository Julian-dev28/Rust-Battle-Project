#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, map, Address, Env, Error, Map, Symbol, Vec,
};

/// Enum representing keys for data storage.
///
/// # Variants
///
/// * `Player` - The key for a player.
/// * `Battle` - The key for a battle.
/// * `Players` - The key for the list of players.
/// * `Battles` - The key for the list of battles.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Player(Address),
    Battle(Symbol),
    Players,
    Battles,
}

/// Struct representing player statistics.
///
/// # Fields
///
/// * `player_address` - The address of the player.
/// * `health` - The health of the player.
/// * `attack` - The attack of the player.
/// * `defense` - The defense of the player.
/// * `in_battle` - A boolean indicating whether the player is in a battle.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlayerStat {
    pub player_address: Address,
    pub health: u32,
    pub attack: u32,
    pub defense: u32,
    pub in_battle: bool,
}

/// Struct representing a battle.
///
/// # Fields
///
/// * `battle_status` - The status of the battle.
/// * `name` - The name of the battle.
/// * `players` - The players in the battle.
/// * `moves` - The moves made by the players in the battle.
/// * `winner` - The winner of the battle.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Battle {
    pub battle_status: u64,
    pub name: Symbol,
    pub players: Map<Address, u64>,
    pub moves: Map<Address, u64>,
    pub winner: Address,
}

/// Enum representing battle statuses.
///
/// # Variants
///
/// * `Pending` - The battle is pending.
/// * `Started` - The battle has started.
/// * `Ended` - The battle has ended.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum BattleStatus {
    Pending = 0,
    Started = 1,
    Ended = 2,
}

/// Contract for handling battles.
#[contract]
pub struct BattleContract;

/// Implementation of the BattleContract.
#[contractimpl]
impl BattleContract {
    /// Adds a player to the battle.
    ///
    /// # Arguments
    ///
    /// * `env` - The contract execution environment.
    /// * `user` - The address of the player to add.
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

    /// Sets the player statistics for a given player.
    ///
    /// # Arguments
    ///
    /// * `env` - The contract execution environment.
    /// * `user` - The address of the player.
    /// * `player_stat` - The PlayerStat struct containing the player's statistics.
    fn set_player_stats(env: Env, user: Address, player_stat: PlayerStat) -> Result<(), Error> {
        env.storage()
            .instance()
            .set(&DataKey::Player(user), &player_stat);
        env.storage().instance().bump(100, 100);
        Ok(())
    }

    /// Gets the player statistics for a given player.
    ///
    /// # Arguments
    ///
    /// * `env` - The contract execution environment.
    /// * `user` - The address of the player.
    ///
    /// # Returns
    ///
    /// A PlayerStat struct containing the player's statistics.
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

    /// Sets the list of players in the battle.
    ///
    /// # Arguments
    ///
    /// * `env` - The contract execution environment.
    /// * `players` - The Vec<Address> containing the addresses of the players.
    fn set_players(env: Env, players: Vec<Address>) {
        env.storage().instance().set(&DataKey::Players, &players);
        env.storage().instance().bump(100, 100);
    }

    /// Gets the list of players in the battle.
    ///
    /// # Arguments
    ///
    /// * `env` - The contract execution environment.
    ///
    /// # Returns
    ///
    /// A Vec<Address> containing the addresses of the players.
    pub fn get_players(env: Env) -> Vec<Address> {
        env.storage()
            .instance()
            .get(&DataKey::Players)
            .unwrap_or(Vec::new(&env))
    }

    /// Creates a battle.
    ///
    /// # Arguments
    ///
    /// * `env` - The contract execution environment.
    /// * `name` - The name of the battle.
    /// * `user` - The address of the player creating the battle.
    pub fn create_battle(
        env: Env,
        name: Symbol,
        user: Address,
    ) -> (Result<(), Error>, Result<(), Error>) {
        user.require_auth();
        // Todo user must be registered
        let contract_id = env.current_contract_address();
        env.storage().instance().set(
            &DataKey::Battle(name.clone()),
            &Battle {
                battle_status: 0,
                name: name.clone(),
                players: map![&env, (user.clone(), 1), (contract_id.clone(), 2)],
                moves: map![&env, (user.clone(), 0), (contract_id.clone(), 0)],
                winner: contract_id.clone(),
            },
        );

        let mut player = Self::get_player_stats(env.clone(), user.clone());
        assert!(!player.in_battle, "Player already in battle");
        player.in_battle = true;
        let mut battles = Self::get_battles(env.clone());
        battles.push_back(name.clone());

        let response: (Result<(), Error>, Result<(), Error>) = (
            Self::set_player_stats(env.clone(), user.clone(), player),
            Self::set_battles(env.clone(), battles),
        );
        response
    }

    /// Creates an auto battle.
    ///
    /// # Arguments
    ///
    /// * `env` - The contract execution environment.
    /// * `name` - The name of the battle.
    /// * `user` - The address of the player creating the battle.
    pub fn create_auto_battle(env: Env, name: Symbol, user: Address) -> Result<(), Error> {
        user.require_auth();
        let contract_id = env.current_contract_address();
        env.storage().instance().set(
            &DataKey::Battle(name.clone()),
            &Battle {
                battle_status: 1,
                name: name.clone(),
                players: map![&env, (user.clone(), 1), (contract_id.clone(), 2)],
                moves: map![&env, (user.clone(), 0), (contract_id.clone(), 0)],
                winner: contract_id.clone(),
            },
        );

        let mut battles = Self::get_battles(env.clone());
        battles.push_back(name.clone());
        Self::set_battles(env.clone(), battles)
    }

    /// Joins a battle.
    ///
    /// # Arguments
    ///
    /// * `env` - The contract execution environment.
    /// * `name` - The name of the battle.
    /// * `user` - The address of the player joining the battle.
    pub fn join_battle(
        env: Env,
        name: Symbol,
        user: Address,
    ) -> (Result<(), Error>, Result<(), Error>) {
        user.require_auth();
        let contract_id = env.current_contract_address();
        let mut battle = Self::get_battle(env.clone(), name.clone());
        assert!(battle.battle_status == 0, "Battle already started");
        let mut player = Self::get_player_stats(env.clone(), user.clone());
        assert!(!player.in_battle, "Player already in battle");

        let players = battle.players.clone();
        let player_1 = players.keys().get(0).unwrap_or(contract_id.clone());
        battle.players = map![&env, (player_1.clone(), 1), (user.clone(), 2)];
        battle.moves = map![&env, (player_1.clone(), 0), (user.clone(), 0)];
        battle.battle_status = 1;
        player.in_battle = true;

        // battle = Battle {
        //     battle_status: 1,
        //     name: name.clone(),
        //     players: map![&env, (player_1, 1), (user.clone(), 2)],
        //     moves: map![&env, (player_1, 0), (user.clone(), 0)],
        //     winner: contract_id.clone(),
        // };
        let response: (Result<(), Error>, Result<(), Error>) = (
            Self::set_battle(env.clone(), name.clone(), battle.clone()),
            Self::set_player_stats(env.clone(), user.clone(), player.clone()),
        );

        assert!(response == (Ok(()), Ok(())), "Error joining battle");

        response
    }

    /// Joins an auto battle.
    ///
    /// # Arguments
    ///
    /// * `env` - The contract execution environment.
    /// * `name` - The name of the battle.
    /// * `user` - The address of the player joining the battle.
    pub fn challenge_bot(env: Env, user: Address, name: Symbol) -> Result<(), Error> {
        let mut battle = Self::get_battle(env.clone(), name.clone());
        assert!(battle.battle_status == 0, "Battle already started");
        let contract_id = env.current_contract_address();

        battle.players = map![&env, (user.clone(), 1), (contract_id.clone(), 2)];
        battle.battle_status = 1;

        Self::set_battle(env.clone(), name.clone(), battle)
    }

    /// Sets a battle.
    ///
    /// # Arguments
    ///
    /// * `env` - The contract execution environment.
    /// * `name` - The name of the battle.
    /// * `battle` - The Battle struct containing the battle information.
    fn set_battle(env: Env, name: Symbol, battle: Battle) -> Result<(), Error> {
        env.storage()
            .instance()
            .set(&DataKey::Battle(name.clone()), &battle);

        env.storage().instance().bump(100, 100);
        Ok(())
    }

    /// Gets a battle.
    ///
    /// # Arguments
    ///
    /// * `env` - The contract execution environment.
    /// * `name` - The name of the battle.
    ///
    /// # Returns
    ///
    /// A Battle struct containing the battle information.
    pub fn get_battle(env: Env, name: Symbol) -> Battle {
        let contract_id = env.current_contract_address();
        env.storage()
            .instance()
            .get(&DataKey::Battle(name.clone()))
            .unwrap_or(Battle {
                battle_status: 0,
                name: name.clone(),
                players: map![&env, (contract_id.clone(), 1), (contract_id.clone(), 2)],
                moves: map![&env, (contract_id.clone(), 0), (contract_id.clone(), 0)],
                winner: env.current_contract_address(),
            })
    }

    /// Sets the list of battles.
    ///
    /// # Arguments
    ///
    /// * `env` - The contract execution environment.
    /// * `battles` - The Vec<Symbol> containing the names of the battles.
    fn set_battles(env: Env, battles: Vec<Symbol>) -> Result<(), Error> {
        env.storage().instance().set(&DataKey::Battles, &battles);
        env.storage().instance().bump(100, 100);
        Ok(())
    }

    /// Gets the list of battles.
    ///
    /// # Arguments
    ///
    /// * `env` - The contract execution environment.
    ///
    /// # Returns
    ///
    /// A Vec<Symbol> containing the names of the battles.
    pub fn get_battles(env: Env) -> Vec<Symbol> {
        env.storage()
            .instance()
            .get(&DataKey::Battles)
            .unwrap_or(Vec::new(&env))
    }

    /// Handles player's attack or defend choice in a battle.
    ///
    /// # Arguments
    ///
    /// * `env` - The contract execution environment.
    /// * `user` - The address of the player making the choice.
    /// * `choice` - The choice made by the player.
    /// * `battle_name` - The name of the battle in which the choice is made.
    pub fn attack_or_defend_choice(env: Env, user: Address, choice: u64, battle_name: Symbol) {
        user.require_auth();
        let contract_id = env.current_contract_address();
        let battle = Self::get_battle(env.clone(), battle_name.clone());

        assert!(
            battle.battle_status == 1,
            "Battle not started. Please tell another player to join the battle"
        ); // Require that battle has started
        assert!(battle.battle_status != 2, "Battle has already ended"); // Require that battle has not ended
        assert!(
            battle.players.contains_key(user.clone()),
            "You are not in this battle"
        ); // Require that player is in the battle

        let _ = Self::register_player_move(env.clone(), user.clone(), choice, battle_name.clone());

        let battle = Self::get_battle(env.clone(), battle_name.clone());
        let moves_left = 2
            - (battle.moves.get(user.clone()).unwrap_or(0) == 0) as u64
            - (battle.moves.get(contract_id.clone()).unwrap_or(0) == 0) as u64;

        if moves_left == 0 {
            Self::await_battle_results(env.clone(), battle_name.clone(), user.clone());
        }
    }

    /// A private function to await battle results.
    ///
    /// # Arguments
    ///
    /// * `env` - The contract execution environment.
    /// * `name` - The name of the battle.
    /// * `user` - The address of the user.
    fn await_battle_results(env: Env, name: Symbol, _user: Address) {
        let battle = Self::get_battle(env.clone(), name.clone());
        let user_1 = battle
            .players
            .keys()
            .get(0)
            .unwrap_or(env.current_contract_address());
        let user_2 = battle
            .players
            .keys()
            .get(1)
            .unwrap_or(env.current_contract_address());

        assert!(
            battle.moves.get(user_1.clone()).unwrap_or(0) != 0
                && battle.moves.get(user_2.clone()).unwrap_or(0) != 0,
            "Players have not made their moves yet"
        );
    }

    /// Registers a player's move in the battle.
    ///
    /// # Arguments
    ///
    /// * `env` - The contract execution environment.
    /// * `user` - The address of the player making the move.
    /// * `choice` - The choice made by the player.
    /// * `battle_name` - The name of the battle in which the move is made.
    ///
    /// # Returns
    ///
    /// An `Result<(), Error>` indicating the result of the operation.
    fn register_player_move(
        env: Env,
        user: Address,
        choice: u64,
        battle_name: Symbol,
    ) -> (Result<(), Error>, Result<(), Error>) {
        let mut battle = Self::get_battle(env.clone(), battle_name.clone());
        Map::set(&mut battle.moves, user.clone(), choice);

        let set_battle_result = Self::set_battle(env.clone(), battle_name.clone(), battle);
        let result: (Result<(), Error>, Result<(), Error>) = (set_battle_result, Ok(()));
        result
    }

    pub fn increase_health(env: Env, user: Address, incr: u32) -> u32 {
        // Get the current count.
        let mut player_stat = Self::get_player_stats(env.clone(), user.clone());

        // Increment the count.
        player_stat.health += incr;

        // Save the count.
        let _ = Self::set_player_stats(env.clone(), user.clone(), player_stat.clone());

        // Return the count to the caller.
        player_stat.health
    }

    pub fn decrease_health(env: Env, user: Address, decr: u32) -> u32 {
        // Get the current count.
        let mut player_stat = Self::get_player_stats(env.clone(), user.clone());

        // Increment the count.
        player_stat.health -= decr;

        // Save the count.
        let _ = Self::set_player_stats(env.clone(), user.clone(), player_stat.clone());

        // Return the count to the caller.
        player_stat.health
    }

    pub fn increase_attack(env: Env, user: Address, incr: u32) -> u32 {
        // Get the current count.
        let mut player_stat = Self::get_player_stats(env.clone(), user.clone());

        // Increment the count.
        player_stat.attack += incr;

        // Save the count.
        let _ = Self::set_player_stats(env.clone(), user.clone(), player_stat.clone());

        // Return the count to the caller.
        player_stat.attack
    }

    pub fn decrease_attack(env: Env, user: Address, decr: u32) -> u32 {
        // Get the current count.
        let mut player_stat = Self::get_player_stats(env.clone(), user.clone());

        // Increment the count.
        player_stat.attack -= decr;

        // Save the count.
        let _ = Self::set_player_stats(env.clone(), user.clone(), player_stat.clone());

        // Return the count to the caller.
        player_stat.attack
    }

    pub fn increase_defense(env: Env, user: Address, incr: u32) -> u32 {
        // Get the current count.
        let mut player_stat = Self::get_player_stats(env.clone(), user.clone());

        // Increment the count.
        player_stat.defense += incr;

        // Save the count.
        let _ = Self::set_player_stats(env.clone(), user.clone(), player_stat.clone());

        // Return the count to the caller.
        player_stat.defense
    }

    pub fn decrease_defense(env: Env, user: Address, decr: u32) -> u32 {
        // Get the current count.
        let mut player_stat = Self::get_player_stats(env.clone(), user.clone());

        // Increment the count.
        player_stat.defense -= decr;

        // Save the count.
        let _ = Self::set_player_stats(env.clone(), user.clone(), player_stat.clone());

        // Return the count to the caller.
        player_stat.defense
    }
}

#[cfg(test)]
mod test;
