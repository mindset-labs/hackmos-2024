use cosmwasm_std::{Coin, DepsMut, Env, MessageInfo, Response, Uint128};
use cw404::state::DECIMALS;

use crate::{state::CONFIG, ContractError};


/// Buy shares from the property contract directly (or from the DAO directly)
/// not the marketplace.
pub fn execute_buy_shares(deps: DepsMut, env: Env, info: MessageInfo, amount: Uint128) -> Result<Response, ContractError> {
    // check payment
    let funds: Vec<Coin> = info.funds.clone();
    
    if funds.is_empty() {
        return Err(ContractError::NoFundsSent {});
    }

    let price_per_share = CONFIG.load(deps.storage)?.price_per_share;
    let decimals = DECIMALS.load(deps.storage)?;
    let required_amount = amount * price_per_share.amount * Uint128::from((10u128).pow(decimals.into()));
    let amount: Uint128 = info
        .funds
        .iter()
        .find(|coin| coin.denom == price_per_share.denom)
        .map(|coin| coin.amount)
        .unwrap_or_else(Uint128::zero);

    if amount.is_zero() {
        return Err(ContractError::NoFundsSent {});
    } else if amount < required_amount {
        return Err(ContractError::InsufficientFunds {
            required: required_amount,
        });
    }
    
    // call the cw404 transfer_from function
    let dao_contract_address = env.contract.address.to_string();
    let buyer_address = info.sender.to_string();
    cw404::execute::transfer_from(deps, env, info, dao_contract_address, buyer_address, amount, None)?;
    Ok(Response::new())
}