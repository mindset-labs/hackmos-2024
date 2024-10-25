use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{to_json_binary, Addr, CosmosMsg, StdResult, WasmMsg};

use crate::{msg::ExecuteMsg, state::{Config, DAOCategory, DAOMetadata}, ContractError};

/// CwTemplateContract is a wrapper around Addr that provides a lot of helpers
/// for working with this.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct CwTemplateContract(pub Addr);

impl CwTemplateContract {
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

    pub fn call<T: Into<ExecuteMsg>>(&self, msg: T) -> StdResult<CosmosMsg> {
        let msg = to_json_binary(&msg.into())?;
        Ok(WasmMsg::Execute {
            contract_addr: self.addr().into(),
            msg,
            funds: vec![],
        }
        .into())
    }
}

impl Config {
    pub fn validate_royalty_fee(&self) -> Result<(), ContractError> {
        if self.default_royalty_fee > 10_000 {
            return Err(ContractError::InvalidRoyaltyFee {});
        }
        Ok(())
    }
}

impl DAOMetadata {
    pub fn validate(&self) -> Result<(), ContractError> {
        if self.category == DAOCategory::Other && self.category_other.is_none() {
            return Err(ContractError::InvalidCategory {});
        }
        Ok(())
    }
}
