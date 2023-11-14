#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, map, vec, Address, Env, Map, String, Symbol, Vec,
};

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
    pub players: Map<Address, u64>,
    pub moves: Map<Address, u64>,
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
                players: map![&env, (user.clone(), 1), (contract_id.clone(), 2)],
                moves: map![&env, (user.clone(), 0), (contract_id.clone(), 0)],
                winner: contract_id.clone(),
            },
        );

        let mut battles = Self::get_battles(env.clone());
        battles.push_back(name.clone());
        Self::set_battles(env.clone(), battles);
    }

    pub fn join_battle(env: Env, name: Symbol, user: Address) {
        user.require_auth();
        let contract_id = env.current_contract_address();
        let mut battle = Self::get_battle(env.clone(), name.clone());
        assert!(battle.battle_status == 0, "Battle already started");
        let mut player = Self::get_player_stats(env.clone(), user.clone());
        assert!(!player.in_battle, "Player already in battle");

        let mut players = battle.players.clone();
        players.get(user.clone()).unwrap_or(0);
        let player_1 = players.keys().get(0).unwrap_or(contract_id.clone());
        players = map![&env, (player_1, 1), (user.clone(), 2)];
        battle.battle_status = 1;
        player.in_battle = true;

        Self::set_battle(env.clone(), name.clone(), battle);
        Self::set_player_stats(env.clone(), user.clone(), player);
    }

    pub fn challenge_bot(env: Env, user: Address, name: Symbol) {
        let mut battle = Self::get_battle(env.clone(), name.clone());
        assert!(battle.battle_status == 0, "Battle already started");
        let contract_id = env.current_contract_address();

        battle.players = map![&env, (user.clone(), 1), (contract_id.clone(), 2)];
        battle.battle_status = 1;

        Self::set_battle(env.clone(), name.clone(), battle);
    }

    pub fn get_battle(env: Env, name: Symbol) -> Battle {
        let contract_id = env.current_contract_address();
        env.storage()
            .instance()
            .get(&DataKey::Battle(name.clone()))
            .unwrap_or(Battle {
                battle_status: 0,
                name: name.clone(),
                players: map![&env, (1, contract_id.clone()), (2, contract_id.clone())],
                moves: map![&env, (contract_id.clone(), 0), (contract_id.clone(), 0)],
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

        Self::register_player_move(env.clone(), user.clone(), choice, battle_name.clone());

        let battle = Self::get_battle(env.clone(), battle_name.clone());
        let moves_left = 2
            - (battle.moves.get(user.clone()).unwrap_or(0) == 0) as u64
            - (battle.moves.get(contract_id.clone()).unwrap_or(0) == 0) as u64;

        // if (moves_left == 0) {
        //     Self::await_battle_results(env.clone(), battle_name.clone());
        // }
    }

    fn register_player_move(
        env: Env,
        user: Address,
        choice: u64,
        battle_name: Symbol,
    ) -> Result<(), String> {
        let mut battle = Self::get_battle(env.clone(), battle_name.clone());
        Map::set(&mut battle.moves, user.clone(), choice);
        Self::set_battle(env.clone(), battle_name.clone(), battle);
        Ok(())
    }

    //     function attackOrDefendChoice(uint8 _choice, string memory _battleName) external {
    //     Battle memory _battle = getBattle(_battleName);

    //     require(
    //         _battle.battleStatus == BattleStatus.STARTED,
    //         "Battle not started. Please tell another player to join the battle"
    //     ); // Require that battle has started
    //     require(
    //         _battle.battleStatus != BattleStatus.ENDED,
    //         "Battle has already ended"
    //     ); // Require that battle has not ended
    //     require(
    //       msg.sender == _battle.players[0] || msg.sender == _battle.players[1],
    //       "You are not in this battle"
    //     ); // Require that player is in the battle

    //     require(_battle.moves[_battle.players[0] == msg.sender ? 0 : 1] == 0, "You have already made a move!");

    //     _registerPlayerMove(_battle.players[0] == msg.sender ? 0 : 1, _choice, _battleName);

    //     _battle = getBattle(_battleName);
    //     uint _movesLeft = 2 - (_battle.moves[0] == 0 ? 0 : 1) - (_battle.moves[1] == 0 ? 0 : 1);
    //     emit BattleMove(_battleName, _movesLeft == 1 ? true : false);

    //     if(_movesLeft == 0) {
    //       _awaitBattleResults(_battleName);
    //     }
    //   }
}

#[cfg(test)]
mod test;
