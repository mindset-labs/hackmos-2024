#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128};
use cw20::{BalanceResponse, TokenInfoResponse};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{BALANCES, DECIMALS, MINTED, NAME, OWNER, SYMBOL, TOTAL_SUPPLY};
use crate::query::*;
use crate::execute::*;

const CONTRACT_NAME: &str = "crates.io:cw404";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let total_supply = msg.total_native_supply.u128() * ((10u128).pow(msg.decimals.into()));
    DECIMALS.save(deps.storage, &msg.decimals)?;
    TOTAL_SUPPLY.save(deps.storage, &Uint128::from(total_supply))?;
    MINTED.save(deps.storage, &Uint128::zero())?;
    NAME.save(deps.storage, &msg.name)?;
    SYMBOL.save(deps.storage, &msg.symbol)?;
    OWNER.save(deps.storage, &info.sender.to_string())?;
    BALANCES.save(deps.storage, &info.sender, &Uint128::from(total_supply))?;

    Ok(Response::new()
        .add_attribute("action", "mint")
        .add_attribute("to", info.sender.to_string())
        .add_attribute("amount", total_supply.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Approve {
            spender,
            token_id,
            expires: _,
        } => approve(deps, env, info, spender, token_id),
        ExecuteMsg::ApproveAll {
            operator,
            expires: _,
        } => approve_all(deps, env, info, operator),
        ExecuteMsg::IncreaseAllowance {
            spender,
            amount,
            expires: _expires,
        } => approve(deps, env, info, spender, amount),
        ExecuteMsg::RevokeAll { operator } => revoke_all(deps, env, info, operator),
        // This is the default implementation in erc404
        ExecuteMsg::TransferFrom {
            owner,
            recipient,
            amount,
        } => transfer_from(deps, env, info, owner, recipient, amount, None),
        // This is the default implementation in erc404
        ExecuteMsg::Transfer { recipient, amount } => transfer(
            deps,
            env,
            info.clone(),
            info.sender.to_string(),
            recipient,
            amount,
        ),
        // Added to ensure compatibility with cw721
        ExecuteMsg::TransferNft {
            recipient,
            token_id,
        } => transfer_from(
            deps,
            env,
            info.clone(),
            info.sender.to_string(),
            recipient,
            token_id,
            Some("transfer".to_string()),
        ),
        // Added to ensure compatibility with cw20
        ExecuteMsg::Send {
            contract,
            amount,
            msg,
        } => send(
            deps,
            env,
            info.clone(),
            info.sender.to_string(),
            contract,
            msg,
            amount,
        ),
        // Added to ensure compatibility with cw721
        ExecuteMsg::SendNft {
            contract,
            token_id,
            msg,
        } => send_nft(
            deps,
            env,
            info.clone(),
            info.sender.to_string(),
            contract,
            msg,
            token_id,
        ),
        // Additional feature added by dojo team to prevent accidental burning of CW721 tokens that a user may wish to keep (as cw20 transfers might burn tokens)
        ExecuteMsg::SetLock { token_id, state } => set_lock(deps, env, info, token_id, state),

        // Event functions
        ExecuteMsg::GenerateNftEvent {
            sender,
            recipient,
            token_id,
        } => generate_nft_event(deps, env, info.clone(), sender, recipient, token_id),
        ExecuteMsg::GenerateNftMintEvent {
            sender,
            recipient,
            token_id,
        } => generate_nft_mint_event(deps, env, info.clone(), sender, recipient, token_id),
        ExecuteMsg::GenerateNftBurnEvent { sender, token_id } => {
            generate_nft_burn_event(deps, env, info.clone(), sender, token_id)
        }

        // Auxillary functions
        ExecuteMsg::SetWhitelist { target, state } => set_whitelist(deps, env, info, target, state),
        ExecuteMsg::SetBaseTokenUri { uri } => set_base_token_uri(deps, env, info, uri),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Minter {} => to_json_binary(&minter(deps)?),
        QueryMsg::ContractInfo {} => to_json_binary(&contract_info(deps)?),
        QueryMsg::Balance { address } => {
            let user = deps.api.addr_validate(&address)?;
            let balance = BALANCES
                .may_load(deps.storage, &user)?
                .unwrap_or(Uint128::zero());

            to_json_binary(&BalanceResponse { balance })
        }
        QueryMsg::TokenInfo {} => {
            let name = NAME.load(deps.storage)?;
            let symbol = SYMBOL.load(deps.storage)?;
            let decimals = DECIMALS.load(deps.storage)?;
            let total_supply = TOTAL_SUPPLY.load(deps.storage)?;
            to_json_binary(&TokenInfoResponse {
                name,
                symbol,
                decimals,
                total_supply,
            })
        }
        QueryMsg::NftInfo { token_id } => to_json_binary(&nft_info(deps, token_id)?),
        QueryMsg::OwnerOf {
            token_id,
            include_expired,
        } => to_json_binary(&owner_of(
            deps,
            env,
            token_id,
            include_expired.unwrap_or(false),
        )?),
        // Allows us to view state of a user
        QueryMsg::UserInfo { address } => to_json_binary(&user_info(deps, env, address)?),
        QueryMsg::ExtendedInfo { token_id } => to_json_binary(&extended_info(deps, env, token_id)?),
        QueryMsg::Allowance { owner, spender } => {
            to_json_binary(&allowance(deps, env, owner, spender)?)
        }
        QueryMsg::IsLocked { token_id } => to_json_binary(&is_locked(deps, env, token_id)?),
        QueryMsg::AllNftInfo {
            token_id,
            include_expired,
        } => to_json_binary(&all_nft_info(
            deps,
            env,
            token_id,
            include_expired.unwrap_or(false),
        )?),
        QueryMsg::NumTokens {} => to_json_binary(&num_tokens(deps)?),
        QueryMsg::Tokens {
            owner,
            start_after,
            limit,
        } => to_json_binary(&tokens(deps, owner, start_after, limit)?),
        QueryMsg::AllTokens { start_after, limit } => {
            to_json_binary(&all_tokens(deps, start_after, limit)?)
        }
    }
}

#[cfg(test)]
mod tests {}
