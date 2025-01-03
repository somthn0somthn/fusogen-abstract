use cw_storage_plus::{Item, Map};
use cosmwasm_std::Addr;
use abstract_app::objects::TruncatedChainId;

#[cosmwasm_schema::cw_serde]

/// Initial implementation of Fusogen state.
/// Currently supports basic claim verification between source and destination chains.
/// Future versions will integrate full token locking and minting functionality on XION.
/// The `CONFIG` defines the source and destination chains for Fusogen's IBC interactions.
/// - `source_chain`: The chain where token locks are initiated.
/// - `destination`: The chain where new DAO tokens will be minted.
pub struct Config {
    pub source_chain: TruncatedChainId,
    pub destination_chain: TruncatedChainId,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const CLAIMED_LOCKS: Map<&Addr, ()> = Map::new("locked");



