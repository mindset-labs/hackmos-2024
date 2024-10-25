use cosmwasm_schema::{cw_serde, QueryResponses};

use crate::state::RegistryCodeId;

#[cw_serde]
pub struct InstantiateMsg {
    pub admins: Option<Vec<String>>,
    pub owner: Option<String>,
    pub code_ids: Option<RegistryCodeId>,
}

#[cw_serde]
pub enum ExecuteMsg {
    SetRegistryCodeId(RegistryCodeId),
    InstantiateDao,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
