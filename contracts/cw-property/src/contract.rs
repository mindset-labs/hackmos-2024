#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::execute::execute_buy_shares;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::query::{query_outstanding_shares, query_property_details, query_share_balance};
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
        owner: info.sender.clone(),
    })?;

    // instantiate the base cw404 contract internally
    cw404::instantiate::instantiate(deps.storage, env.clone(), MessageInfo {
        sender: env.contract.address, // set the DAO contract as the owner of all the shares initially
        funds: info.funds,
    }, cw404::msg::InstantiateMsg {
        name: msg.name,
        symbol: msg.symbol,
        decimals: 2,
        total_native_supply: msg.total_shares,
        minter: None,
    })?;
    // give full allowance to the property contract
    // cw404::execute::approve_all(deps, env.clone(), info.clone(), env.contract.address.to_string())?;

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
        ExecuteMsg::BuyShares { amount, from } => execute_buy_shares(deps, env, info, from, amount)?,
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
    match msg {
        // handle all the base cw404 queries
        QueryMsg::Cw404QueryMsg(msg) => Ok(cw404::contract::query(deps, env, msg)?),
        // handle custom property queries
        QueryMsg::GetPropertyDetails {} => to_json_binary(&query_property_details(deps, env)?),
        QueryMsg::GetShareHolders {} => {
            unimplemented!()
        }
        QueryMsg::GetShareBalance { address } => to_json_binary(&query_share_balance(deps, env, address)?),
        QueryMsg::OutstandingShares {} => to_json_binary(&query_outstanding_shares(deps, env)?),
    }
}

/// 
/// 
/// 
/// 
/// Tests
/// 
/// 
/// 
/// 
#[cfg(test)]
mod tests {
    use cosmwasm_std::{coin, coins, Addr, Coin, Uint128};
    use cw_multi_test::{error::AnyResult, App, ContractWrapper, Executor};

    use crate::msg::{ExecuteMsg, OutstandingSharesResponse};

    /// 
    /// A wrapper around the code ID of the DAO contract
    /// 
    pub struct PropertyCodeId(u64);

    impl PropertyCodeId {
        pub fn store_code(app: &mut App) -> Self {
            let contract = ContractWrapper::new(
                crate::contract::execute,
                crate::contract::instantiate,
                crate::contract::query,
            );
            let code_id = app.store_code(Box::new(contract));
            Self(code_id)
        }

        #[allow(clippy::too_many_arguments)]
        pub fn instantiate(
            self,
            app: &mut App,
            sender: Addr,
            label: &str,
        ) -> AnyResult<PropertyContract> {
            PropertyContract::instantiate(app, self, sender, label)
        }
    }

    pub struct PropertyContract(Addr);

    impl PropertyContract {
        pub fn addr(&self) -> Addr {
            self.0.clone()
        }
    
        #[allow(clippy::too_many_arguments)]
        #[track_caller]
        pub fn instantiate(
            app: &mut App,
            code_id: PropertyCodeId,
            sender: Addr,
            label: &str,
        ) -> AnyResult<Self> {
            let init_msg = crate::msg::InstantiateMsg {
                name: "test".to_string(),
                symbol: "test".to_string(),
                context: None,
                total_shares: Uint128::from(100u128),
                price_per_share: Coin::new(100u128, "utoken"),
                estimated_monthly_income: Coin::new(100u128, "utoken"),
                estimated_apy: 100,
                status: "pending".to_string(),
                subcategory: "real_estate".to_string(),
                image_uri: "".to_string(),
                royalty_fee: 100,
                property_contract_address: None,
            };
            app.instantiate_contract(code_id.0, sender, &init_msg, &[], label, None)
                .map(Self::from)
        }
    }

    impl From<Addr> for PropertyContract {
        fn from(value: Addr) -> Self {
            Self(value)
        }
    }

    /// 
    /// Mock app
    /// 
    fn mock_app() -> App {
        App::new(|router, api, storage| {
            router
                .bank
                .init_balance(
                    storage,
                    &api.addr_make("owner"),
                    coins(100_000_000, "utoken"),
                )
                .unwrap();

            router
                .bank
                .init_balance(
                    storage,
                    &api.addr_make("buyer"),
                    coins(100_000_000, "utoken"),
                )
                .unwrap();
        })
    }

    #[test]
    fn basic_initialization() {
        let mut app = mock_app();
        let owner = app.api().addr_make("owner");
        let property_code_id = PropertyCodeId::store_code(&mut app);
        let property_contract = property_code_id.instantiate(&mut app, owner.clone(), "test").unwrap();
    
        let balance: cw20::BalanceResponse = app.wrap().query_wasm_smart(
            property_contract.addr(),
            &crate::msg::QueryMsg::Cw404QueryMsg(cw404::msg::QueryMsg::Balance {
                address: property_contract.addr().to_string(),
            }),
        ).unwrap();

        // assert that the balance is reflected to the contract
        assert_eq!(balance.balance, Uint128::from(100 * 10u128.pow(2u32)));
    }

    #[test]
    fn buy_shares() {
        let mut app = mock_app();
        let owner = app.api().addr_make("owner");
        let buyer = app.api().addr_make("buyer");
        let property_code_id = PropertyCodeId::store_code(&mut app);
        let property_contract = property_code_id.instantiate(&mut app, owner.clone(), "test").unwrap();

        let balance: cw20::BalanceResponse = app.wrap().query_wasm_smart(
            property_contract.addr(),
            &crate::msg::QueryMsg::Cw404QueryMsg(cw404::msg::QueryMsg::Balance {
                address: property_contract.addr().to_string(),
            }),
        ).unwrap();

        // assert that the balance is reflected to the contract
        assert_eq!(balance.balance, Uint128::from(100 * 10u128.pow(2u32)));

        app.execute_contract(
            buyer.clone(),
            property_contract.addr().clone(),
            &ExecuteMsg::BuyShares { amount: Uint128::from(1u128), from: Some(property_contract.addr()) },
            &[
                coin(1000, "utoken"),
            ],
        ).unwrap();

        let balance_updated: cw20::BalanceResponse = app.wrap().query_wasm_smart(
            property_contract.addr(),
            &crate::msg::QueryMsg::Cw404QueryMsg(cw404::msg::QueryMsg::Balance {
                address: buyer.to_string(),
            }),
        ).unwrap();

        assert_eq!(balance_updated.balance, Uint128::from(10u128.pow(2u32)));
    }

    #[test]
    fn check_available_shares() {
        let mut app = mock_app();
        let owner = app.api().addr_make("owner");
        let buyer = app.api().addr_make("buyer");
        let property_code_id = PropertyCodeId::store_code(&mut app);
        let property_contract = property_code_id.instantiate(&mut app, owner.clone(), "test").unwrap();

        app.execute_contract(
            buyer.clone(),
            property_contract.addr().clone(),
            &ExecuteMsg::BuyShares { amount: Uint128::from(50u128), from: None },
            &[
                coin(10_000, "utoken"),
            ],
        ).unwrap();

        // the user just bought 10 shares, so there should be 90 shares left
        let response: OutstandingSharesResponse = app.wrap().query_wasm_smart(
            property_contract.addr(),
            &crate::msg::QueryMsg::OutstandingShares {},
        ).unwrap();

        println!("{:?}", response);
        assert_eq!(response.remaining_shares, Uint128::from(50 * 10u128.pow(2u32)));
    }
}

