use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;
use crate::state::{Config, DAOMetadata, DAOProperty, DAOStats};

#[cw_serde]
pub struct InstantiateMsg {
    pub admins: Vec<Addr>,
    pub metadata: DAOMetadata,
    pub default_royalty_fee: Option<u64>,
    pub property_contract_code_id: Option<u64>,
}

#[cw_serde]
pub enum ExecuteMsg {
    SetPropertyContractCodeId { code_id: u64 },
    LaunchProperty { data: DAOProperty },
    UpdateAdmins { add: Vec<Addr>, remove: Vec<Addr> },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Config)]
    GetConfig {},
    #[returns(DAOMetadata)]
    GetMetadata {},
    #[returns(Option<u64>)]
    GetPropertyContractCodeId {},
    #[returns(Vec<DAOProperty>)]
    GetAllProperties {},
    #[returns(Option<DAOProperty>)]
    GetProperty { id: Addr },
    #[returns(DAOStats)]
    GetStats {},
}
