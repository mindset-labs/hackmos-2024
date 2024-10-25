use cosmwasm_std::{Addr, DepsMut, Env, MessageInfo, Response, StdResult, Storage};

use crate::{assert::assert_admin_or_owner, state::{DAOProperty, CONFIG}, ContractError};


pub fn set_property_contract_code_id(deps: DepsMut, info: MessageInfo, code_id: u64) -> Result<Response, ContractError> {
    CONFIG.update(deps.storage, |mut config| -> StdResult<_> {
        config.property_contract_code_id = Some(code_id);
        Ok(config)
    })?;

    Ok(Response::default()
        .add_attribute("action", "set_property_contract_code_id"))
}

pub fn launch_property(deps: DepsMut, env: Env, info: MessageInfo, data: DAOProperty) -> Result<Response, ContractError> {
    unimplemented!()
}

pub fn update_admins(deps: DepsMut, info: MessageInfo, add: Vec<Addr>, remove: Vec<Addr>) -> Result<Response, ContractError> {
    assert_admin_or_owner(deps.storage, &info.sender)?;

    CONFIG.update(deps.storage, |mut config| -> StdResult<_> {
        // add new admins if they are not already in the list
        add.iter().for_each(|addr| {
            if !config.admins.contains(&addr) {
                config.admins.push(addr.clone());
            }
        });
        // remove some admins if requested
        remove.iter().for_each(|addr| { config.admins.retain(|a| a != addr) });
        Ok(config)
    })?;

    Ok(Response::default()
        .add_attribute("action", "update_admins"))
}
