use cw20::{AllowanceResponse, BalanceResponse, TokenInfoResponse};

use cosmwasm_std::{to_json_binary, Binary, Deps, Env, StdResult, Uint128};

use cw721::{
    AllNftInfoResponse, Approval, ContractInfoResponse, NftInfoResponse, NumTokensResponse,
    OwnerOfResponse, TokensResponse,
};

use crate::msg::{ExtendedInfoResponse, MinterResponse, QueryMsg, UserInfoResponse};
use crate::state::{
    ALLOWANCE, BALANCES, BASE_TOKEN_URI, DECIMALS, GET_APPROVED, LOCKED, MINTED, NAME, OWNED,
    OWNED_INDEX, OWNER_OF, SYMBOL, TOTAL_SUPPLY,
};

const DEFAULT_LIMIT: u32 = 10;
const MAX_LIMIT: u32 = 1000;

pub fn contract_info(deps: Deps) -> StdResult<ContractInfoResponse> {
    let name = NAME.load(deps.storage)?;
    let symbol = SYMBOL.load(deps.storage)?;
    Ok(ContractInfoResponse { name, symbol })
}

pub fn num_tokens(deps: Deps) -> StdResult<NumTokensResponse> {
    let count = MINTED.may_load(deps.storage)?.unwrap_or(Uint128::zero());
    Ok(NumTokensResponse {
        count: count.u128() as u64,
    })
}

pub fn nft_info(deps: Deps, token_id: String) -> StdResult<cw721::NftInfoResponse> { // TODO: Check replace string with struct
    let base_uri = BASE_TOKEN_URI
        .may_load(deps.storage)?
        .unwrap_or("".to_string());
    Ok(NftInfoResponse {
        token_uri: Some(base_uri + &token_id),
        extension: None,
    })
}

pub fn owner_of(
    deps: Deps,
    _env: Env,
    token_id: String,
    _include_expired: bool,
) -> StdResult<OwnerOfResponse> {
    let owner = OWNER_OF
        .may_load(deps.storage, token_id)?
        .unwrap_or("".to_string());
    Ok(OwnerOfResponse {
        owner,
        approvals: vec![],
    })
}

pub fn user_info(deps: Deps, _env: Env, address: String) -> StdResult<UserInfoResponse> {
    let owned = OWNED
        .may_load(deps.storage, address.clone())?
        .unwrap_or(vec![]);
    let balances = BALANCES
        .may_load(deps.storage, &deps.api.addr_validate(&address)?)?
        .unwrap_or(Uint128::zero());
    Ok(UserInfoResponse { owned, balances })
}

pub fn extended_info(deps: Deps, _env: Env, token_id: String) -> StdResult<ExtendedInfoResponse> {
    let owned_index = OWNED_INDEX
        .may_load(deps.storage, token_id.clone())?
        .unwrap_or(Uint128::zero());
    let owner_of = OWNER_OF
        .may_load(deps.storage, token_id.clone())?
        .unwrap_or("".to_string());
    Ok(ExtendedInfoResponse {
        owned_index,
        owner_of,
    })
}

pub fn allowance(
    deps: Deps,
    _env: Env,
    owner: String,
    spender: String,
) -> StdResult<AllowanceResponse> {
    let allowance = ALLOWANCE
        .may_load(deps.storage, (owner, spender))?
        .unwrap_or(Uint128::zero());

    Ok(AllowanceResponse {
        allowance,
        expires: cw20::Expiration::Never {},
    })
}

pub fn is_locked(deps: Deps, _env: Env, token_id: String) -> StdResult<bool> {
    let locked = LOCKED.may_load(deps.storage, token_id)?.unwrap_or(false);
    Ok(locked)
}

pub fn tokens(
    deps: Deps,
    owner: String,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<TokensResponse> {
    let owner_addr = deps.api.addr_validate(&owner)?;
    let mut owned = OWNED
        .may_load(deps.storage, owner_addr.to_string())?
        .unwrap();

    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as u64;
    let mut start = start_after
        .clone()
        .unwrap_or("0".to_string())
        .parse::<u64>()
        .ok()
        .unwrap();
    start = if start_after.is_none() { 0 } else { start };
    let offset = if start_after.is_none() { 0 } else { 1 };

    owned.sort();

    let start_index = owned
        .iter()
        .position(|item| item.u128() as u64 == start)
        .unwrap_or(0)
        + offset;

    let end_index = (start_index + limit as usize).min(owned.len()); // Calculate end index

    let tokens = owned[start_index..end_index]
        .to_vec()
        .iter()
        .map(|item| item.to_string())
        .collect();

    Ok(TokensResponse { tokens })
}

pub fn all_tokens(
    deps: Deps,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<TokensResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as u64;
    let mut start = start_after
        .clone()
        .unwrap_or("0".to_string())
        .parse::<u64>()
        .ok()
        .unwrap();
    start = if start_after.is_none() { 0 } else { start + 1 };

    let minted = MINTED.may_load(deps.storage)?.unwrap_or(Uint128::zero());
    let end = if start + limit >= minted.u128() as u64 {
        minted.u128() as u64
    } else {
        start + limit
    };

    let mut tokens: Vec<String> = Vec::new();
    for i in start..end {
        tokens.push(i.to_string());
    }

    Ok(TokensResponse { tokens })
}

pub fn all_nft_info(
    deps: Deps,
    _env: Env,
    token_id: String,
    _include_expired: bool,
) -> StdResult<cw721::AllNftInfoResponse> { // TODO: Check replace string with struct
    let owner = OWNER_OF
        .may_load(deps.storage, token_id.clone())?
        .unwrap_or("".to_string());
    let spender = GET_APPROVED
        .may_load(deps.storage, token_id.clone())?
        .unwrap_or("".to_string());
    let info = nft_info(deps, token_id)?;
    let approvals = if spender.len() == 0 {
        vec![]
    } else {
        vec![Approval {
            /// Account that can transfer/send the token
            spender: spender.to_string(),
            /// When the Approval expires (maybe Expiration::never)
            expires: cw721::Expiration::Never {},
        }]
    };

    Ok(AllNftInfoResponse {
        access: OwnerOfResponse {
            owner: owner.to_string(),
            approvals,
        },
        info: NftInfoResponse {
            token_uri: info.token_uri,
            extension: info.extension,
        },
    })
}

pub fn minter(deps: Deps) -> StdResult<MinterResponse> {
    let minter = cw_ownable::get_ownership(deps.storage)?
        .owner
        .map(|a| a.into_string());

    Ok(MinterResponse { minter })
}
