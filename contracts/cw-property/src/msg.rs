use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin, Uint128};
use cw20::Cw20ReceiveMsg;
use cw404::msg::{QueryMsg as Cw404QueryMsg, ExecuteMsg as Cw404ExecuteMsg};

#[cw_serde]
pub struct InstantiateMsg {
    pub name: String,
    pub symbol: String,
    pub context: Option<String>,
    pub total_shares: Uint128,
    pub price_per_share: Coin,
    pub estimated_monthly_income: Coin,
    pub estimated_apy: u64,
    pub status: String,
    pub subcategory: String,
    pub image_uri: String,
    pub royalty_fee: u64,
    pub property_contract_address: Option<String>,
}

#[cw_serde]
pub enum ExecuteMsg {
    // inherit from cw404
    Cw404ExecuteMsg(Cw404ExecuteMsg),

    // property specific execute
    ListShares {
        amount: Uint128,
    },
    BuyShares {
        amount: Uint128,
        from: Option<Addr>,
    },
    ClaimPayout {},
    ReceivePayment(Cw20ReceiveMsg),
    ReceivePaymentNative {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // inherit from cw404
    #[returns(Option<()>)]
    Cw404QueryMsg(Cw404QueryMsg),
    
    // property specific queries
    #[returns(Option<()>)]
    GetPropertyDetails {},
    #[returns(Option<()>)]
    GetShareHolders {},
    #[returns(Option<()>)]
    GetShareBalance { id: Addr },
    #[returns(Option<()>)]
    OutstandingShares {},
}
