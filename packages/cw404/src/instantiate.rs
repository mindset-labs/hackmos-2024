use cosmwasm_std::{Env, MessageInfo, Response, Storage, Uint128};

use crate::{msg::InstantiateMsg, state::{DECIMALS, TOTAL_SUPPLY, MINTED, NAME, SYMBOL, OWNER, BALANCES}, ContractError};


pub fn instantiate(storage: &mut dyn Storage, _env: Env, info: MessageInfo, msg: InstantiateMsg) -> Result<u128, ContractError> {
    let total_supply = msg.total_native_supply.u128() * ((10u128).pow(msg.decimals.into()));
    DECIMALS.save(storage, &msg.decimals)?;
    TOTAL_SUPPLY.save(storage, &Uint128::from(total_supply))?;
    MINTED.save(storage, &Uint128::zero())?;
    NAME.save(storage, &msg.name)?;
    SYMBOL.save(storage, &msg.symbol)?;
    OWNER.save(storage, &info.sender.to_string())?;
    BALANCES.save(storage, &info.sender, &Uint128::from(total_supply))?;

    Ok(total_supply)
}
