use cosmwasm_std::{to_json_binary, Addr, DepsMut, Env, MessageInfo, Response, StdResult, Storage, SubMsg, WasmMsg};
use cw_property::{msg::InstantiateMsg as PropertyInstantiateMsg};
use crate::{assert::assert_admin_or_owner, reply::{ReplyMessageId}, state::{DAOProperty, CONFIG}, ContractError};

pub fn execute_set_property_contract_code_id(deps: DepsMut, info: MessageInfo, code_id: u64) -> Result<Response, ContractError> {
    assert_admin_or_owner(deps.storage, &info.sender)?;

    CONFIG.update(deps.storage, |mut config| -> StdResult<_> {
        config.property_contract_code_id = Some(code_id);
        Ok(config)
    })?;

    Ok(Response::default()
        .add_attribute("action", "set_property_contract_code_id"))
}

pub fn execute_launch_property(deps: DepsMut, env: Env, info: MessageInfo, data: DAOProperty) -> Result<Response, ContractError> {
    assert_admin_or_owner(deps.storage, &info.sender)?;
    
    let code_id = CONFIG.load(deps.storage)?.property_contract_code_id
        .ok_or(ContractError::PropertyCodeIdNotProvided {})?;

    let instantiate_msg = WasmMsg::Instantiate {
        code_id,
        msg: to_json_binary(&PropertyInstantiateMsg {
            // TODO: add property metadata
        })?,
        funds: vec![],
        label: "Property Contract Instantiation".to_string(),
        // set the property admin as the contract itself
        admin: Some(env.contract.address.to_string()),
    };

    // Wrap the instantiate message as a SubMsg, to capture its response
    let sub_msg = SubMsg::reply_on_success(instantiate_msg, ReplyMessageId::InstantiateProperty as u64)
        .with_payload(info.sender.as_bytes());

    Ok(Response::new()
        .add_submessage(sub_msg)
        .add_attribute("action", "instantiate_with_response"))
}

pub fn execute_update_admins(deps: DepsMut, info: MessageInfo, add: Vec<Addr>, remove: Vec<Addr>) -> Result<Response, ContractError> {
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
