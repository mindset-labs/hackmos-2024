use cosmwasm_schema::cw_serde;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{to_json_binary, Addr, Binary, CosmosMsg, StdResult, Uint128, WasmMsg};
use cw_storage_plus::{Item, Map};

pub const OWNER: Item<String> = Item::new("owner");

pub const NAME: Item<String> = Item::new("name");
pub const SYMBOL: Item<String> = Item::new("symbol");
pub const BASE_TOKEN_URI: Item<String> = Item::new("base_token_uri");
pub const DECIMALS: Item<u8> = Item::new("decimals");
pub const TOTAL_SUPPLY: Item<Uint128> = Item::new("total_supply");
pub const MINTED: Item<Uint128> = Item::new("minted");
pub const WHITELIST: Map<String, bool> = Map::new("whitelist");
/// Approval in native representation
pub const GET_APPROVED: Map<String, String> = Map::new("get_approved");
/// Allowance of user in fractional representation
pub const ALLOWANCE: Map<(String, String), Uint128> = Map::new("cw20_allowance");
pub const BALANCES: Map<&Addr, Uint128> = Map::new("balance");
/// Owner of a tokenID in native representation
pub const OWNER_OF: Map<String, String> = Map::new("owner_of");
/// Array of owned ids in native representation
pub const OWNED: Map<String, Vec<Uint128>> = Map::new("owned");
/// @dev Tracks indices for the _owned mapping
pub const OWNED_INDEX: Map<String, Uint128> = Map::new("owned_index");
pub const APPROVED_FOR_ALL: Map<(String, String), bool> = Map::new("approved_for_all");

/// Additional features
/// @dev prevents being burnt due to transfers made in mistake
pub const LOCKED: Map<String, bool> = Map::new("locked");
