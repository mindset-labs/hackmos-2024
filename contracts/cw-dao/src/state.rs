use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin, Uint128};
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Config {
    pub admins: Vec<Addr>,
    pub owner: Addr,
    pub default_royalty_fee: u64, // as a percentage, 2 decimals (100 = 1%)
}

pub const CONFIG: Item<Config> = Item::new("config");

#[cw_serde]
pub enum DAOCategory {
    RealEstate,
    Retail,
    Hospitality,
    Other,
}

#[cw_serde]
pub struct DAOMetadata {
    pub name: String,
    pub symbol: Option<String>,
    pub description: Option<String>,
    pub image_uri: Option<String>,
    pub category: DAOCategory,
    pub category_other: Option<String>,
}

// DAO general metadata, set on instantiation
pub const DAO_METADATA: Item<DAOMetadata> = Item::new("dao_metadata");

#[cw_serde]
pub struct DAOStats {
    pub num_properties: u64,
    pub portfolio_value: u128,
    pub num_investors: u64,
    pub apy: u64,
}

// DAO stats, new entries added when property contracts (cw-property / cw-404) are initialized via the DAO contract
pub const DAO_STATS: Item<DAOStats> = Item::new("dao_stats");

#[cw_serde]
pub struct DAOProperty {
    pub price_per_share: Coin,
    pub estimated_monthly_income: Coin,
    pub estimated_apy: u64, // as a percentage, 2 decimals (100 = 1%)
    pub total_shares: u64,
    pub status: String,
    pub subcategory: String,
    pub image_uri: String,
    pub royalty_fee: u64, // as a percentage, 2 decimals (100 = 1%)
}

// Map of property contract address to DAOProperty configs
pub const DAO_PROPERTIES: Map<Addr, DAOProperty> = Map::new("dao_properties");
