use cosmwasm_std::{to_json_binary, DepsMut, MessageInfo, Response, Storage, SubMsg, WasmMsg};
use cw_dao::msg::InstantiateMsg as DaoInstantiateMsg;

use crate::{
    reply::ReplyMessageId,
    state::{RegistryCodeId, REGISTRY},
    ContractError,
};

pub fn execute_set_registry_code_id(
    storage: &mut dyn Storage,
    code_ids: RegistryCodeId,
) -> Result<Response, ContractError> {
    REGISTRY.save(storage, &code_ids)?;
    Ok(Response::default())
}

pub fn execute_instantiate_dao(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let code_id = REGISTRY.load(deps.storage)?;

    if code_id.dao_contract_code_id.is_none() {
        return Err(ContractError::CodeIdNotProvided {
            contract_type: "DAO",
        });
    }

    let instantiate_msg = WasmMsg::Instantiate {
        code_id: code_id.dao_contract_code_id.unwrap(),
        msg: to_json_binary(&DaoInstantiateMsg {
            name: "DAO".to_string(),
            symbol: "DAO".to_string(),
        })?,
        funds: vec![],
        label: "DAO Contract Instantiation".to_string(),
        admin: None,
    };

    // Wrap the instantiate message as a SubMsg, to capture its response
    let sub_msg = SubMsg::reply_on_success(instantiate_msg, ReplyMessageId::InstantiateDao as u64)
        .with_payload(info.sender.as_bytes());

    Ok(Response::new()
        .add_submessage(sub_msg)
        .add_attribute("action", "instantiate_with_response"))
}
