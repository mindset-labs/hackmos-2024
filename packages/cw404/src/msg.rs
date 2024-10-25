use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{to_json_binary, Binary, CosmosMsg, StdResult, Uint128, WasmMsg};
use cw20::Cw20ReceiveMsg;
use cw_utils::Expiration;
use schemars::JsonSchema;

#[cw_serde]
pub struct InstantiateMsg {
    // Name of the NFT contract
    pub name: String,
    // Symbol of the NFT contract
    pub symbol: String,
    // Decimals of erc404 token
    pub decimals: u8,
    // Supply of NFTs max
    pub total_native_supply: Uint128,

    // The minter is the only one who can create new NFTs.
    // This is designed for a base NFT that is controlled by an external program
    // or contract. You will likely replace this with custom logic in custom NFTs
    pub minter: Option<String>,
}

// This is like Cw721ExecuteMsg but we add a Mint command for an owner
// to make this stand-alone. You will likely want to remove mint and
// use other control logic in any contract that inherits this.
#[cw_serde]
pub enum ExecuteMsg {
    // Transfer is a base message to move a token to another account without triggering actions
    TransferFrom {
        owner: String,
        recipient: String,
        amount: Uint128,
    },
    Transfer {
        recipient: String,
        amount: Uint128,
    },
    TransferNft {
        recipient: String,
        token_id: Uint128,
    },
    Send {
        contract: String,
        amount: Uint128,
        msg: Binary,
    },
    SendNft {
        contract: String,
        token_id: Uint128,
        msg: Binary,
    },
    IncreaseAllowance {
        spender: String,
        amount: Uint128,
        expires: Option<Expiration>,
    },
    // Allows operator to transfer / send the token from the owner's account.
    // If expiration is set, then this allowance has a time/height limit
    Approve {
        spender: String,
        token_id: Uint128,
        expires: Option<Expiration>,
    },
    // Allows operator to transfer / send any token from the owner's account.
    // If expiration is set, then this allowance has a time/height limit
    ApproveAll {
        operator: String,
        expires: Option<Expiration>,
    },
    // Remove previously granted ApproveAll permission
    RevokeAll {
        operator: String,
    },
    GenerateNftEvent {
        sender: String,
        recipient: String,
        token_id: Uint128,
    },
    GenerateNftMintEvent {
        sender: String,
        recipient: String,
        token_id: Uint128,
    },
    GenerateNftBurnEvent {
        sender: String,
        token_id: Uint128,
    },
    SetWhitelist {
        target: String,
        state: bool,
    },
    SetLock {
        token_id: Uint128,
        state: bool,
    },
    SetBaseTokenUri {
        uri: String,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // Return the owner of the given token, error if token does not exist
    #[returns(cw721::OwnerOfResponse)]
    OwnerOf {
        token_id: String,
        // unset or false will filter out expired approvals, you must set to true to see them
        include_expired: Option<bool>,
    },
    #[returns(bool)]
    IsLocked { token_id: String },
    #[returns(cw721::OwnerOfResponse)]
    UserInfo { address: String },

    #[returns(cw20::AllowanceResponse)]
    Allowance { owner: String, spender: String },

    #[returns(ExtendedInfoResponse)]
    ExtendedInfo { token_id: String },

    // Total number of tokens issued
    #[returns(cw721::NumTokensResponse)]
    NumTokens {},

    // With MetaData Extension.
    // Returns top-level metadata about the contract
    #[returns(cw721::ContractInfoResponse)]
    ContractInfo {},
    // With MetaData Extension.
    // Returns metadata about one particular token, based on *ERC721 Metadata JSON Schema*
    // but directly from the contract
    // TODO: replace String with a custom struct
    #[returns(cw721::NftInfoResponse<String>)]
    NftInfo { token_id: String },

    #[returns(cw20::BalanceResponse)]
    Balance { address: String },

    #[returns(cw20::TokenInfoResponse)]
    TokenInfo {},
    // With MetaData Extension.
    // Returns the result of both `NftInfo` and `OwnerOf` as one query as an optimization
    // for clients
    // TODO: replace String with a custom struct
    #[returns(cw721::AllNftInfoResponse<String>)]
    AllNftInfo {
        token_id: String,
        // unset or false will filter out expired approvals, you must set to true to see them
        include_expired: Option<bool>,
    },

    // With Enumerable extension.
    // Returns all tokens owned by the given address, [] if unset.
    #[returns(cw721::TokensResponse)]
    Tokens {
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    // With Enumerable extension.
    // Requires pagination. Lists all token_ids controlled by the contract.
    #[returns(cw721::TokensResponse)]
    AllTokens {
        start_after: Option<String>,
        limit: Option<u32>,
    },

    // Return the minter
    #[returns(MinterResponse)]
    Minter {},
}

// Shows who can mint these tokens
#[cw_serde]
pub struct MinterResponse {
    pub minter: Option<String>,
}

#[cw_serde]
pub struct UserInfoResponse {
    pub owned: Vec<Uint128>,
    pub balances: Uint128,
}

#[cw_serde]
pub struct ExtendedInfoResponse {
    pub owned_index: Uint128,
    pub owner_of: String,
}

/// Cw721ReceiveMsg should be de/serialized under `Receive()` variant in a ExecuteMsg
/// Updated to be used with cosmwasm_std::Binary v2.1.0
#[cw_serde]
pub struct Cw721ReceiveMsg {
    pub sender: String,
    pub token_id: String,
    pub msg: Binary,
}

impl Cw721ReceiveMsg {
    /// serializes the message
    pub fn into_binary(self) -> StdResult<Binary> {
        let msg = ReceiverExecuteMsg::ReceiveNft(self);
        to_json_binary(&msg)
    }

    /// creates a cosmos_msg sending this struct to the named contract
    pub fn into_cosmos_msg<T: Into<String>, C>(self, contract_addr: T) -> StdResult<CosmosMsg<C>>
    where
        C: Clone + std::fmt::Debug + PartialEq + JsonSchema,
    {
        let msg = self.into_binary()?;
        let execute = WasmMsg::Execute {
            contract_addr: contract_addr.into(),
            msg,
            funds: vec![],
        };
        Ok(execute.into())
    }
}

/// This is just a helper to properly serialize the above message.
/// The actual receiver should include this variant in the larger ExecuteMsg enum
#[cw_serde]
enum ReceiverExecuteMsg {
    ReceiveNft(Cw721ReceiveMsg),
    Receive(Cw20ReceiveMsg),
}
