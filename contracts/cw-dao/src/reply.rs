use std::io::Read;

use cosmwasm_std::{Addr, DepsMut, Reply, Response};

use crate::{state::{DAO_PROPERTIES, DAO_PROPERTIES_DRAFT}, ContractError};


pub enum ReplyMessageId {
    InstantiateProperty = 1,
}

impl TryFrom<u64> for ReplyMessageId {
    type Error = ContractError;

    fn try_from(id: u64) -> Result<Self, Self::Error> {
        match id {
            1 => Ok(ReplyMessageId::InstantiateProperty),
            _ => Err(ContractError::UnknownReplyMessageId {}),
        }
    }
}

pub fn reply_instantiate_property(deps: DepsMut, msg: Reply) -> Result<Response, ContractError> {
    // TODO: Optimize this, no need to clone the result and iterate over events twice

    // get the property contract address from the reply
    let property_contract_address_unchecked = msg.result.clone()
            .unwrap() // Assuming the instantiation was successful
            .events
            .iter()
            .flat_map(|event| event.attributes.iter())
            .find(|attr| attr.key == "_contract_address")
            .map(|attr| attr.value.clone())
            .expect("contract address should be present in reply");
    let property_contract_address = Addr::unchecked(&property_contract_address_unchecked);
    
    // validate the owner address
    let owner_address_unchecked = msg.result
            .unwrap() // Assuming the instantiation was successful
            .events
            .iter()
            .flat_map(|event| event.attributes.iter())
            .find(|attr| attr.key == "context")
            .map(|attr| attr.value.clone())
            .expect("context should be present in reply");
    
    let property_data = DAO_PROPERTIES_DRAFT.may_load(deps.storage, owner_address_unchecked)?
        .ok_or(ContractError::PropertyDataNotFound {})?;

    // save the property data under the property contract address
    DAO_PROPERTIES.save(deps.storage, property_contract_address, &property_data.clone())?;

    Ok(Response::default())
}
