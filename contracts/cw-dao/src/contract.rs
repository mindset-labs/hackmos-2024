#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult};

use crate::error::ContractError;
use crate::execute::{launch_property, set_property_contract_code_id, update_admins};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::query::{query_all_properties, query_config, query_property, query_property_contract_code_id, query_stats};
use crate::state::{Config, DAOStats, CONFIG, DAO_METADATA, DAO_STATS};

const CONTRACT_NAME: &str = "crates.io:cw-dao";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let config = Config {
        admins: msg.admins,
        owner: info.sender,
        default_royalty_fee: msg.default_royalty_fee.unwrap_or(100), // default to 1%
        property_contract_code_id: msg.property_contract_code_id,
    };

    // validations
    config.validate_royalty_fee()?;
    msg.metadata.validate()?;

    // save state
    DAO_METADATA.save(deps.storage, &msg.metadata)?;
    CONFIG.save(deps.storage, &config)?;
    DAO_STATS.save(deps.storage, &DAOStats::default())?;

    Ok(Response::default()
        .add_attribute("action", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let response = match msg {
        ExecuteMsg::SetPropertyContractCodeId { code_id } => set_property_contract_code_id(deps, info, code_id)?,
        ExecuteMsg::LaunchProperty { data } => launch_property(deps, env, info, data)?,
        ExecuteMsg::UpdateAdmins { add, remove } => update_admins(deps, info, add, remove)?,
    };
    Ok(response)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetConfig {} => to_json_binary(&query_config(deps)?),
        QueryMsg::GetPropertyContractCodeId {} => to_json_binary(&query_property_contract_code_id(deps)?),
        QueryMsg::GetAllProperties {} => to_json_binary(&query_all_properties(deps)?),
        QueryMsg::GetProperty { id } => to_json_binary(&query_property(deps, id)?),
        QueryMsg::GetStats {} => to_json_binary(&query_stats(deps)?),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, env: Env, msg: Reply) -> Result<Response, ContractError> {
    unimplemented!()
}

#[cfg(test)]
mod tests {}
