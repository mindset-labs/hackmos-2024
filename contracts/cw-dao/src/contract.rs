#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult};

use crate::error::ContractError;
use crate::execute::{execute_launch_property, execute_set_property_contract_code_id, execute_update_admins};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::query::{query_all_properties, query_config, query_property, query_property_contract_code_id, query_stats};
use crate::reply::{reply_instantiate_property, ReplyMessageId};
use crate::state::{Config, DAOStats, CONFIG, DAO_METADATA, DAO_STATS};

const CONTRACT_NAME: &str = "crates.io:cw-dao";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let config = Config {
        admins: msg.admins,
        owner: info.sender,
        default_royalty_fee: msg.default_royalty_fee.unwrap_or(100), // default to 1%
        property_contract_code_id: msg.property_contract_code_id,
    };

    // validations
    config.validate_royalty_fee()?;
    msg.metadata.validate()?;

    // save state
    DAO_METADATA.save(deps.storage, &msg.metadata)?;
    CONFIG.save(deps.storage, &config)?;
    DAO_STATS.save(deps.storage, &DAOStats::default())?;

    Ok(Response::default()
        .add_attribute("action", "instantiate")
        .add_attribute("contract_address", env.contract.address.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let response = match msg {
        ExecuteMsg::SetPropertyContractCodeId { code_id } => execute_set_property_contract_code_id(deps, info, code_id)?,
        ExecuteMsg::LaunchProperty { data } => execute_launch_property(deps, env, info, data)?,
        ExecuteMsg::UpdateAdmins { add, remove } => execute_update_admins(deps, info, add, remove)?,
    };
    Ok(response)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetConfig {} => to_json_binary(&query_config(deps)?),
        QueryMsg::GetPropertyContractCodeId {} => to_json_binary(&query_property_contract_code_id(deps)?),
        QueryMsg::GetAllProperties {} => to_json_binary(&query_all_properties(deps)?),
        QueryMsg::GetProperty { id } => to_json_binary(&query_property(deps, id)?),
        QueryMsg::GetStats {} => to_json_binary(&query_stats(deps)?),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, env: Env, msg: Reply) -> Result<Response, ContractError> {
    match ReplyMessageId::try_from(msg.id) {
        Ok(ReplyMessageId::InstantiateProperty) => Ok(reply_instantiate_property(deps, msg)?),
        Err(e) => Err(e),
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
    use cosmwasm_std::{coins, Addr, Coin, Uint128};
    use cw_multi_test::{error::AnyResult, App, ContractWrapper, Executor};

    use crate::msg::ExecuteMsg;
    use crate::state::DAOProperty;

    /// 
    /// A wrapper around the code ID of the DAO contract
    /// 
    pub struct DaoCodeId(u64);

    impl DaoCodeId {
        pub fn store_code(app: &mut App) -> Self {
            let contract = ContractWrapper::new(
                crate::contract::execute,
                crate::contract::instantiate,
                crate::contract::query,
            ).with_reply(crate::contract::reply);
            let code_id = app.store_code(Box::new(contract));
            Self(code_id)
        }

        #[allow(clippy::too_many_arguments)]
        pub fn instantiate(
            self,
            app: &mut App,
            sender: Addr,
            label: &str,
        ) -> AnyResult<DaoContract> {
            DaoContract::instantiate(app, self, sender, label)
        }
    }

    pub struct DaoContract(Addr);

    impl DaoContract {
        pub fn addr(&self) -> Addr {
            self.0.clone()
        }
    
        #[allow(clippy::too_many_arguments)]
        #[track_caller]
        pub fn instantiate(
            app: &mut App,
            code_id: DaoCodeId,
            sender: Addr,
            label: &str,
        ) -> AnyResult<Self> {
            let init_msg = crate::msg::InstantiateMsg {
                admins: vec![],
                metadata: crate::state::DAOMetadata {
                    name: "test".to_string(),
                    description: Some("test".to_string()),
                    // links: vec![],
                    symbol: Some("test".to_string()),
                    image_uri: None,
                    category: crate::state::DAOCategory::Other,
                    category_other: Some("Car Rental".to_string()),
                },
                default_royalty_fee: None,
                property_contract_code_id: None,
            };
            app.instantiate_contract(code_id.0, sender, &init_msg, &[], label, None)
                .map(Self::from)
        }
    }

    impl From<Addr> for DaoContract {
        fn from(value: Addr) -> Self {
            Self(value)
        }
    }

    /// 
    /// A wrapper around the property contract (cw-property) to store the code ID
    /// 
    pub struct PropertyCodeId(u64);

    impl PropertyCodeId {
        pub fn store_code(app: &mut App) -> Self {
            let contract = ContractWrapper::new(
                cw_property::contract::execute,
                cw_property::contract::instantiate,
                cw_property::contract::query,
            );
            let code_id = app.store_code(Box::new(contract));
            Self(code_id)
        }
    }

    /// 
    /// Mock app
    /// 
    fn mock_app() -> App {
        App::new(|router, _api, storage| {
            router
                .bank
                .init_balance(
                    storage,
                    &Addr::unchecked("owner"),
                    coins(1_000_000, "utoken"),
                )
                .unwrap();
        })
    }

    #[test]
    fn basic_initialization() {
        let mut app = mock_app();
        let owner = app.api().addr_make(&"owner".to_string());
        let dao_code_id = DaoCodeId::store_code(&mut app);
        dao_code_id.instantiate(&mut app, owner.clone(), "test").unwrap();
    }

    #[test]
    fn owner_or_admins_can_update_admins() {
        let mut app = mock_app();
        let owner = app.api().addr_make(&"owner".to_string());
        let dao_code_id = DaoCodeId::store_code(&mut app);
        let dao_addr = dao_code_id.instantiate(&mut app, owner.clone(), "test").unwrap();

        // owner can add a new admin
        let new_admin = app.api().addr_make(&"new_admin".to_string());
        app.execute_contract(
            owner.clone(),
            dao_addr.addr(),
            &ExecuteMsg::UpdateAdmins { add: vec![new_admin.clone()], remove: vec![] },
            &[],
        )
        .unwrap();

        // the new admin can add another (3rd) admin
        let new_admin2 = app.api().addr_make(&"new_admin2".to_string());
        app.execute_contract(
            new_admin.clone(),
            dao_addr.addr(),
            &ExecuteMsg::UpdateAdmins { add: vec![new_admin2.clone()], remove: vec![] },
            &[],
        )
        .unwrap();
    }

    #[test]
    fn non_owner_cannot_update_admins() {
        let mut app = mock_app();
        let owner = app.api().addr_make(&"owner".to_string());
        let dao_code_id = DaoCodeId::store_code(&mut app);
        let dao_addr = dao_code_id.instantiate(&mut app, owner.clone(), "test").unwrap();

        let new_admin = app.api().addr_make(&"new_admin".to_string());
        app.execute_contract(
            new_admin.clone(),
            dao_addr.addr(),
            &ExecuteMsg::UpdateAdmins { add: vec![new_admin.clone()], remove: vec![] },
            &[],
        )
        .unwrap_err();
    }

    #[test]
    fn can_set_the_property_contract_code_id() {
        let mut app = mock_app();
        let owner = app.api().addr_make(&"owner".to_string());
        let dao_code_id = DaoCodeId::store_code(&mut app);
        let dao_addr = dao_code_id.instantiate(&mut app, owner.clone(), "test").unwrap();
        let property_code_id = PropertyCodeId::store_code(&mut app);

        app.execute_contract(
            owner.clone(),
            dao_addr.addr(),
            &ExecuteMsg::SetPropertyContractCodeId { code_id: property_code_id.0 },
            &[],
        )
        .unwrap();
    }

    #[test]
    fn can_launch_a_property_contract() {
        let mut app = mock_app();
        let owner = app.api().addr_make(&"owner".to_string());
        let dao_code_id = DaoCodeId::store_code(&mut app);
        let dao_addr = dao_code_id.instantiate(&mut app, owner.clone(), "test").unwrap();
        let property_code_id = PropertyCodeId::store_code(&mut app);

        app.execute_contract(
            owner.clone(),
            dao_addr.addr(),
            &ExecuteMsg::SetPropertyContractCodeId { code_id: property_code_id.0 },
            &[],
        )
        .unwrap();

        let result = app.execute_contract(
            owner.clone(),
            dao_addr.addr(),
            &ExecuteMsg::LaunchProperty { data: DAOProperty {
                price_per_share: Coin::new(100u128, "utoken"),
                estimated_monthly_income: Coin::new(100u128, "utoken"),
                estimated_apy: 100,
                total_shares: 100,
                status: "Pending".to_string(),
                subcategory: "Other".to_string(),
                image_uri: "".to_string(),
                royalty_fee: 100,
                property_contract_address: None,
            } },
            &[],
        )
        .unwrap();
    }
}
