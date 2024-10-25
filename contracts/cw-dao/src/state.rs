use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint128;
use cw_storage_plus::{Item, Map};

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
    pub symbol: String,
    pub description: String,
    pub image_uri: String,
    pub category: DAOCategory,
    pub category_other: Option<String>,
}

pub const DAO_METADATA: Item<DAOMetadata> = Item::new("dao_metadata");

#[cw_serde]
pub struct DAOStats {
    pub num_properties: u64,
    pub portfolio_value: u128,
    pub num_investors: u64,
    pub apy: u64,
}

pub const DAO_STATS: Item<DAOStats> = Item::new("dao_stats");

#[cw_serde]
pub struct DAOProperty {
    pub price_per_share: Uint128,
    pub estimated_monthly_income: Uint128,
    pub estimated_apy: u64,
    pub total_shares: u64,
    pub status: String,
    pub subcategory: String,
    pub image_uri: String,
}

pub const DAO_PROPERTIES: Map<String, DAOProperty> = Map::new("dao_properties");
