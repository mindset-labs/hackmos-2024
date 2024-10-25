
use cosmwasm_std::{Addr, Storage};

use crate::ContractError;
use crate::state::CONFIG;

pub fn assert_one_admin(storage: &dyn Storage, sender: &Addr) -> Result<(), ContractError> {
    let config = CONFIG.load(storage)?;
    if !config.admins.contains(&sender) {
        return Err(ContractError::Unauthorized {});
    }
    Ok(())
}

pub fn assert_owner(storage: &dyn Storage, sender: &Addr) -> Result<(), ContractError> {
    let config = CONFIG.load(storage)?;
    if config.owner != sender {
        return Err(ContractError::Unauthorized {});
    }
    Ok(())
}

pub fn assert_admin_or_owner(storage: &dyn Storage, sender: &Addr) -> Result<(), ContractError> {
    let config = CONFIG.load(storage)?;
    // if the sender is not an admin and not the owner, return an error
    if !config.admins.contains(&sender) && config.owner != sender {
        return Err(ContractError::Unauthorized {});
    }
    Ok(())
}
