use cosmwasm_std::{to_json_binary, Addr, DepsMut, Env, MessageInfo, Response, StdResult, Storage, SubMsg, Uint128, WasmMsg};
use cw_property::{msg::InstantiateMsg as PropertyInstantiateMsg};
use crate::{assert::assert_admin_or_owner, reply::ReplyMessageId, state::{DAOProperty, CONFIG, DAO_PROPERTIES, DAO_PROPERTIES_DRAFT}, ContractError};

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
            name: data.name.clone(),
            symbol: data.symbol.clone(),
            total_shares: Uint128::from(10_000u128),
            context: Some(info.sender.to_string()),
            price_per_share: data.price_per_share.clone(),
            estimated_monthly_income: data.estimated_monthly_income.clone(),
            estimated_apy: data.estimated_apy,
            status: data.status.clone(),
            subcategory: data.subcategory.clone(),
            image_uri: data.image_uri.clone(),
            royalty_fee: data.royalty_fee,
            property_contract_address: None,
        })?,
        funds: vec![],
        label: "Property Contract Instantiation".to_string(),
        // set the property admin as the contract itself
        admin: Some(env.contract.address.to_string()),
    };

    // save the property data under the sender's address until the property contract is instantiated
    // and reply is received
    DAO_PROPERTIES_DRAFT.save(deps.storage, info.sender.to_string(), &data)?;

    // Wrap the instantiate message as a SubMsg, to capture its response
    let sub_msg = SubMsg::reply_on_success(instantiate_msg, ReplyMessageId::InstantiateProperty as u64);

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
