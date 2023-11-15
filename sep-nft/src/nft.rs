use crate::balance::{receive_balance, spend_balance};
use crate::storage_types::DataKey;
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Address, Env, String, Symbol, Vec,
};
const METADATA: Symbol = symbol_short!("METADATA");

pub trait NFTCollectionFactory {
    // Admin interface – privileged functions.
    fn initialize(env: Env, admin: Address, collection_name: String, collection_symbol: String);

    fn mint_nft(
        env: Env,
        to: Address,
        name: String,
        symbol: String,
        amount: i128,
        short_uri: String,
        detailed_uri: String,
        long_uri: String,
    ) -> Address; // Returns the address of the minted NFT

    fn return_to_valhalla(env: Env, from: Address, amount: i128);

    // Descriptive Interface
    fn get_metadata(env: Env) -> Metadata;

    fn get_collection_metadata(env: Env) -> CollectionMetadata;

    fn check_nonnegative_amount(amount: i128);
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
// Metadata struct to hold NFT metadata, including descriptions and IPFS hashes.
pub struct Metadata {
    short_description_uri: String, // IPFS hash or URL
    long_description_uri: String,  // IPFS hash or URL
    data_file_uri: String,         // IPFS hash or URL
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
// Metadata struct to hold NFT Collection metadata.
pub struct CollectionMetadata {
    name: String,
    symbol: String,
    nfts: Vec<Address>, // Addresses of all NFTs minted by this collection
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
            nfts: Vec::new(&env),
        };
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage()
            .instance()
            .set(&METADATA, &collection_metadata);
    }

    fn mint_nft(
        env: Env,
        to: Address,
        nft_name: String,
        nft_symbol: String,
        amount: i128,
        short_uri: String,
        detailed_uri: String,
        long_uri: String,
    ) -> Address {
        Self::check_nonnegative_amount(amount);
        // Mint a new NFT.
        let nft_metadata = Metadata {
            short_description_uri: short_uri,
            long_description_uri: detailed_uri,
            data_file_uri: long_uri,
        };
        let nft_metadata_key = DataKey::Metadata;
        env.storage()
            .instance()
            .set(&nft_metadata_key, &nft_metadata);

        let nfts_list = env
            .storage()
            .instance()
            .get::<Symbol, Vec<Address>>(&METADATA)
            .unwrap_or(Vec::new(&env));
        let collection_metadata = CollectionMetadata {
            name: nft_name,
            symbol: nft_symbol,
            nfts: nfts_list,
        };
        let collection_metadata_key = DataKey::CollectionMetadata;
        env.storage()
            .instance()
            .set(&collection_metadata_key, &collection_metadata);

        receive_balance(&env, to.clone(), amount);
        env.storage().instance().bump(100, 100);

        to
    }

    fn return_to_valhalla(env: Env, from: Address, amount: i128) {
        // Burn an NFT.
        from.require_auth();
        spend_balance(&env, from, amount);

        env.storage().instance().bump(100, 100);
    }

    fn get_metadata(env: Env) -> Metadata {
        // Get the metadata of an NFT.
        env.storage().instance().get(&DataKey::Metadata).unwrap()
    }

    fn get_collection_metadata(env: Env) -> CollectionMetadata {
        // Get the metadata of an NFT.
        env.storage().instance().get(&DataKey::Metadata).unwrap()
    }
}
