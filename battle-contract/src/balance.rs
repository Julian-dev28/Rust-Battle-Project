use crate::storage_types::{NFTDataKey, BALANCE_BUMP_AMOUNT};
use soroban_sdk::{map, Address, Env, Map};

pub fn read_balance(e: &Env, addr: Address) -> Map<u32, i128> {
    let key = NFTDataKey::Balance(addr);
    let token_balance_map: soroban_sdk::Map<u32, i128> = e
        .storage()
        .persistent()
        .get(&key)
        .unwrap_or(map![&e, (1, 0), (2, 0), (3, 0)]);
    token_balance_map
}

fn write_balance(e: &Env, addr: Address, token_id: u32, amount: i128) {
    let key = NFTDataKey::Balance(addr);
    let token_balance_map: soroban_sdk::Map<u32, i128> = e
        .storage()
        .persistent()
        .get(&key)
        .unwrap_or(map![&e, (1, 0), (2, 0), (3, 0)]);
    let _token_key = token_balance_map.keys().set(
        token_id.try_into().unwrap_or(0),
        amount.try_into().unwrap_or(0),
    );

    e.storage().persistent().set(&key, &token_balance_map);
    e.storage()
        .persistent()
        .bump(&key, BALANCE_BUMP_AMOUNT, BALANCE_BUMP_AMOUNT + 100);
}

pub fn receive_balance(e: &Env, addr: Address, token_id: u32, amount: i128) {
    let balance = read_balance(e, addr.clone());
    // if !is_authorized(e, addr.clone()) {
    //     panic!("can't receive when deauthorized");
    // }
    let balance_amount: Option<i128> = balance.get(token_id.try_into().unwrap_or(0));
    let new_balance_amount = balance_amount.unwrap_or(0) + amount;
    write_balance(e, addr, token_id, new_balance_amount);
}

pub fn spend_balance(e: &Env, addr: Address, token_id: u32) {
    // let balance = read_balance(e, addr.clone());
    // if !is_authorized(e, addr.clone()) {
    //     panic!("can't spend when deauthorized");
    // }
    // if balance < amount {
    //     panic!("insufficient balance");
    // }
    write_balance(e, addr, token_id, 0);
}

// pub fn is_authorized(e: &Env, addr: Address) -> bool {
//     let key = NFTDataKey::State(addr);
//     if let Some(state) = e.storage().persistent().get::<NFTDataKey, bool>(&key) {
//         state
//     } else {
//         true
//     }
// }

// pub fn write_authorization(e: &Env, addr: Address, is_authorized: bool) {
//     let key = NFTDataKey::State(addr);
//     e.storage().persistent().set(&key, &is_authorized);
// }
