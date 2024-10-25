use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;
use crate::state::{DAOMetadata, DAOProperty};

#[cw_serde]
pub struct InstantiateMsg {
    pub metadata: DAOMetadata,
    pub admins: Vec<Addr>,
    pub default_royalty_fee: Option<u64>,
}

#[cw_serde]
pub enum ExecuteMsg {
    LaunchProperty {
        data: DAOProperty,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
