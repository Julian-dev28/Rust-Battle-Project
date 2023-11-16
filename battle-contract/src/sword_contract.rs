use crate::balance::{receive_balance, spend_balance};
use crate::storage_types::NFTDataKey;
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Error, String};

// This contract is meant to be used for educational purposes only.
pub trait NFTCollectionFactory {
    // Admin interface – privileged functions.
    fn initialize(env: Env, admin: Address, collection_name: String, collection_symbol: String);

    fn mint_nft(env: Env, to: Address, token_id: u32, amount: i128) -> Result<(), Error>; // Returns the address of the minted NFT

    fn melt_blade(env: Env, from: Address, token_id: u32) -> Result<(), Error>;

    // Descriptive Interface
    fn get_token_metadata(env: Env) -> TokenMetadata;

    fn check_nonnegative_amount(amount: i128);
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
// Metadata struct to hold NFT metadata, including descriptions and IPFS hashes.
pub struct TokenMetadata {
    token_uri: String, // IPFS hash or URL
    name: String,
    symbol: String,
}

#[contract]
pub struct SwordContract;

#[contractimpl]
impl NFTCollectionFactory for SwordContract {
    fn check_nonnegative_amount(amount: i128) {
        if amount < 0 {
            panic!("negative amount is not allowed: {}", amount)
        }
    }
    fn initialize(env: Env, admin: Address, collection_name: String, collection_symbol: String) {
        admin.require_auth();
        // Initialize the collection.
        let collection_metadata = TokenMetadata {
            token_uri: String::from_slice(&env, "https://example.com"),
            name: collection_name,
            symbol: collection_symbol,
        };
        env.storage().instance().set(&NFTDataKey::Admin, &admin);
        env.storage()
            .instance()
            .set(&NFTDataKey::Metadata, &collection_metadata);
    }

    fn mint_nft(env: Env, to: Address, token_id: u32, amount: i128) -> Result<(), Error> {
        Self::check_nonnegative_amount(amount);

        let new_token_uri: String = match token_id {
            1 => String::from_slice(&env, "https://example/token0"),
            2 => String::from_slice(&env, "https://example/token1"),
            3 => String::from_slice(&env, "https://example/token2"),
            _ => String::from_slice(&env, "https://example/token0"),
        };

        let _name: String = match token_id {
            1 => String::from_slice(&env, "Longsword"),
            2 => String::from_slice(&env, "Sabre"),
            3 => String::from_slice(&env, "Claymore"),
            _ => String::from_slice(&env, "Longsword"),
        };

        let _symbol: String = match token_id {
            1 => String::from_slice(&env, "LS"),
            2 => String::from_slice(&env, "S"),
            3 => String::from_slice(&env, "C"),
            _ => String::from_slice(&env, "LS"),
        };

        // Mint a new NFT.
        let nft_metadata: TokenMetadata = TokenMetadata {
            token_uri: new_token_uri,
            name: _name,
            symbol: _symbol,
        };
        let nft_metadata_key = NFTDataKey::Metadata;
        env.storage()
            .instance()
            .set(&nft_metadata_key, &nft_metadata);

        receive_balance(&env, to.clone(), token_id, amount);
        env.storage().instance().bump(100, 100);

        Ok(())
    }

    fn melt_blade(env: Env, from: Address, token_id: u32) -> Result<(), Error> {
        // Burn an NFT.
        spend_balance(&env, from, token_id);
        env.storage().instance().bump(100, 100);
        Ok(())
    }

    fn get_token_metadata(env: Env) -> TokenMetadata {
        // Get the metadata of an NFT.
        env.storage().instance().get(&NFTDataKey::Metadata).unwrap()
    }
}
