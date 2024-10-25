use cosmwasm_std::{Addr, DepsMut, Env, MessageInfo, Response, StdResult};

use crate::{state::{DAOProperty, CONFIG}, ContractError};


pub fn set_property_contract_code_id(deps: DepsMut, info: MessageInfo, code_id: u64) -> Result<Response, ContractError> {
    CONFIG.update(deps.storage, |mut config| -> StdResult<_> {
        config.property_contract_code_id = Some(code_id);
        Ok(config)
    })?;

    Ok(Response::default())
}

pub fn launch_property(deps: DepsMut, env: Env, info: MessageInfo, data: DAOProperty) -> Result<Response, ContractError> {
    unimplemented!()
}

pub fn update_admins(deps: DepsMut, info: MessageInfo, add: Vec<Addr>, remove: Vec<Addr>) -> Result<Response, ContractError> {
    unimplemented!()
}
