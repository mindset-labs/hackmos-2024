use cosmwasm_std::{Addr, Deps, Env, Response, StdResult, Uint128};
use cw404::state::{BALANCES, TOTAL_SUPPLY};
use crate::{msg::OutstandingSharesResponse, state::{Config, CONFIG}};

pub fn query_outstanding_shares(deps: Deps, env: Env) -> StdResult<OutstandingSharesResponse> {
    let non_issued_shares = BALANCES.load(deps.storage, &env.contract.address)?;
    let total_shares = TOTAL_SUPPLY.load(deps.storage)?;
    let outstanding_shares = total_shares.checked_sub(non_issued_shares).unwrap_or(Uint128::zero());

    Ok(OutstandingSharesResponse {
        outstanding_shares,
        total_shares,
        remaining_shares: non_issued_shares,
    })
}

pub fn query_share_balance(deps: Deps, _env: Env, address: Addr) -> StdResult<Uint128> {
    Ok(BALANCES.may_load(deps.storage, &address)?.unwrap_or_default())
}

pub fn query_property_details(deps: Deps, _env: Env) -> StdResult<Config> {
    Ok(CONFIG.load(deps.storage)?)
}