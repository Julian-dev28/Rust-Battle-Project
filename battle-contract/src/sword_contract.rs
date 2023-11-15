use crate::balance::{receive_balance, spend_balance};
use crate::storage_types::NFTDataKey;
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String};

// This contract is meant to be used for educational purposes only.
pub trait NFTCollectionFactory {
    // Admin interface – privileged functions.
    fn initialize(env: Env, admin: Address, collection_name: String, collection_symbol: String);

    fn mint_nft(
        env: Env,
        to: Address,
        name: String,
        symbol: String,
        token_id: u32,
        amount: i128,
        token_uri: String,
    ) -> Address; // Returns the address of the minted NFT

    fn melt_blade(env: Env, from: Address, token_id: u32, amount: i128);

    // Descriptive Interface
    fn get_token_metadata(env: Env) -> TokenMetadata;

    fn get_collection_metadata(env: Env) -> CollectionMetadata;

    fn check_nonnegative_amount(amount: i128);
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
// Metadata struct to hold NFT metadata, including descriptions and IPFS hashes.
pub struct TokenMetadata {
    token_uri: String, // IPFS hash or URL
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
// Metadata struct to hold NFT Collection metadata.
pub struct CollectionMetadata {
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
        let collection_metadata = CollectionMetadata {
            name: collection_name,
            symbol: collection_symbol,
        };
        env.storage().instance().set(&NFTDataKey::Admin, &admin);
        env.storage()
            .instance()
            .set(&NFTDataKey::Metadata, &collection_metadata);
    }

    fn mint_nft(
        env: Env,
        to: Address,
        nft_name: String,
        nft_symbol: String,
        token_id: u32,
        amount: i128,
        token_uri: String,
    ) -> Address {
        Self::check_nonnegative_amount(amount);
        // Mint a new NFT.
        let nft_metadata: TokenMetadata = TokenMetadata {
            token_uri: token_uri,
        };
        let nft_metadata_key = NFTDataKey::Metadata;
        env.storage()
            .instance()
            .set(&nft_metadata_key, &nft_metadata);

        let _collection_meta_data: CollectionMetadata = env
            .storage()
            .instance()
            .get(&NFTDataKey::CollectionMetadata)
            .unwrap();

        let collection_metadata = CollectionMetadata {
            name: nft_name,
            symbol: nft_symbol,
        };
        let collection_metadata_key = NFTDataKey::CollectionMetadata;
        env.storage()
            .instance()
            .set(&collection_metadata_key, &collection_metadata);

        receive_balance(&env, to.clone(), token_id, amount);
        env.storage().instance().bump(100, 100);

        to
    }

    fn melt_blade(env: Env, from: Address, token_id: u32, amount: i128) {
        // Burn an NFT.
        from.require_auth();
        spend_balance(&env, from, token_id, amount);

        env.storage().instance().bump(100, 100);
    }

    fn get_token_metadata(env: Env) -> TokenMetadata {
        // Get the metadata of an NFT.
        env.storage().instance().get(&NFTDataKey::Metadata).unwrap()
    }

    fn get_collection_metadata(env: Env) -> CollectionMetadata {
        // Get the metadata of an NFT.
        env.storage().instance().get(&NFTDataKey::Metadata).unwrap()
    }
}
