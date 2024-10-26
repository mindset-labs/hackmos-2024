use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::Item;

#[cw_serde]
pub struct Config {
    pub price_per_share: Coin,
    pub estimated_monthly_income: Coin,
    pub estimated_apy: u64,
    pub status: String,
    pub subcategory: String,
    pub image_uri: String,
    pub royalty_fee: u64,
    pub owner: Addr,
}

pub const CONFIG: Item<Config> = Item::new("config");
