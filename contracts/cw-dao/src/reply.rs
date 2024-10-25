use cosmwasm_std::{Addr, DepsMut, Reply, Response};

use crate::ContractError;


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
    let property_contract_address = msg.result
            .unwrap() // Assuming the instantiation was successful
            .events
            .iter()
            .flat_map(|event| event.attributes.iter())
            .find(|attr| attr.key == "_contract_address")
            .map(|attr| attr.value.clone())
            .expect("contract address should be present in reply");

    Ok(Response::default())
}
