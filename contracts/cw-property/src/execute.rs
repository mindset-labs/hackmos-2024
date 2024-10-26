use cosmwasm_std::{Addr, Coin, DepsMut, Env, MessageInfo, Response, StdResult, Uint128};
use cw404::state::BALANCES;

use crate::{state::CONFIG, ContractError};


/// Buy shares from the property contract directly (or from the DAO directly)
/// not the marketplace.
pub fn execute_buy_shares(deps: DepsMut, env: Env, info: MessageInfo, from: Option<Addr>, amount: Uint128) -> Result<Response, ContractError> {
    // check payment
    let funds: Vec<Coin> = info.funds.clone();
    
    if funds.is_empty() {
        return Err(ContractError::NoFundsSent {});
    }

    let config = CONFIG.load(deps.storage)?;
    let required_amount = amount * config.price_per_share.amount;

    let amount: Uint128 = info
        .funds
        .iter()
        .find(|coin| coin.denom == config.price_per_share.denom)
        .map(|coin| coin.amount)
        .unwrap_or_else(Uint128::zero);

    if amount.is_zero() {
        return Err(ContractError::NoFundsSent {});
    } else if amount < required_amount {
        return Err(ContractError::InsufficientFunds {
            required: required_amount,
        });
    }

    // default to the owner (parent contract // cw-dao) address if no specific seller is provided
    // let from_addr = from.unwrap_or(env.contract.address.clone());
    
    // call the cw404 transfer_from function
    // cw404::execute::transfer(deps, env.clone(), info.clone(), env.contract.address.clone().to_string(), info.sender.to_string(), amount)?;

    BALANCES.update(deps.storage, &env.contract.address.clone(), |balance| -> StdResult<_> {
        Ok(balance.unwrap_or_default().checked_sub(amount)?)
    })?;

    BALANCES.update(deps.storage, &info.sender, |balance| -> StdResult<_> {
        Ok(balance.unwrap_or_default().checked_add(amount)?)
    })?;

    Ok(Response::new())
}