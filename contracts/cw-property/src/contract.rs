#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::execute::execute_buy_shares;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, CONFIG};

const CONTRACT_NAME: &str = "crates.io:cw-property";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // save the property config
    CONFIG.save(deps.storage, &Config {
        price_per_share: msg.price_per_share,
        estimated_monthly_income: msg.estimated_monthly_income,
        estimated_apy: msg.estimated_apy,
        status: msg.status,
        subcategory: msg.subcategory,
        image_uri: msg.image_uri,
        royalty_fee: msg.royalty_fee,
    })?;

    // instantiate the base cw404 contract internally
    cw404::instantiate::instantiate(deps.storage, env.clone(), info.clone(), cw404::msg::InstantiateMsg {
        name: msg.name,
        symbol: msg.symbol,
        decimals: 2,
        total_native_supply: msg.total_shares,
        minter: None,
    })?;
    // give full allowance to the property contract
    cw404::execute::approve_all(deps, env.clone(), info.clone(), env.contract.address.to_string())?;

    Ok(Response::default()
        .add_attribute("action", "instantiate")
        .add_attribute("context", msg.context.unwrap_or_default()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let response = match msg {
        // handle all the base cw404 execution
        ExecuteMsg::Cw404ExecuteMsg(msg) => cw404::contract::execute(deps, env, info, msg)?,
        // handle property specific execution
        ExecuteMsg::ListShares { amount } => {
            unimplemented!()
        },
        ExecuteMsg::BuyShares { amount } => execute_buy_shares(deps, env, info, amount)?,
        ExecuteMsg::ReceivePayment(msg) => {
            unimplemented!()
        }
        ExecuteMsg::ReceivePaymentNative {} => {
            unimplemented!()
        }
        ExecuteMsg::ClaimPayout {} => {
            unimplemented!()
        }
    };
    Ok(response)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    let response = match msg {
        // handle all the base cw404 queries
        QueryMsg::Cw404QueryMsg(msg) => cw404::contract::query(deps, env, msg)?,
        // handle custom property queries
        QueryMsg::GetPropertyDetails {} => {
            unimplemented!()
        }
        QueryMsg::GetShareHolders {} => {
            unimplemented!()
        }
        QueryMsg::GetShareBalance { id } => {
            unimplemented!()
        }
        QueryMsg::OutstandingShares {} => {
            unimplemented!()
        }
    };
    Ok(response)
}

#[cfg(test)]
mod tests {}
