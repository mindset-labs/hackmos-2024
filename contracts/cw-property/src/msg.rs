use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128};
use cw20::Cw20ReceiveMsg;
use cw404::msg::{QueryMsg as Cw404QueryMsg, ExecuteMsg as Cw404ExecuteMsg};

#[cw_serde]
pub struct InstantiateMsg {
    pub context: Option<String>,
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
        id: String,
        amount: Uint128,
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
