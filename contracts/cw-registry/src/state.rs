use cosmwasm_schema::cw_serde;
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Config {
    pub admins: Vec<String>,
    pub owner: String,
}

#[cw_serde]
#[derive(Default)]
pub struct RegistryCodeId {
    pub dao_contract_code_id: Option<u64>,
    pub property_contract_code_id: Option<u64>,
    pub dao_token_code_id: Option<u64>,
    pub dao_voting_contract_code_id: Option<u64>,
    pub dao_treasury_contract_code_id: Option<u64>,
}

#[cw_serde]
pub struct RegistryContracts {
    pub dao: String,
    pub properties: Vec<String>,
    pub token: String,
    pub voting: String,
    pub treasury: String,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const REGISTRY: Item<RegistryCodeId> = Item::new("registry_code_id");

// Map of DAO contracts addresses to their internal contracts addresses
pub const REGISTRY_CONTRACTS: Map<String, RegistryContracts> = Map::new("registry_contracts");
