#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::execute::{execute_instantiate_dao, execute_set_registry_code_id};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::reply::ReplyMessageId;
use crate::state::{Config, RegistryCodeId, CONFIG, REGISTRY};

const CONTRACT_NAME: &str = "crates.io:cw-registry";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    CONFIG.save(
        deps.storage,
        &Config {
            admins: msg.admins.unwrap_or_default(),
            owner: msg.owner.unwrap_or(info.sender.to_string()),
        },
    )?;

    match msg.code_ids {
        Some(code_ids) => REGISTRY.save(deps.storage, &code_ids)?,
        None => REGISTRY.save(deps.storage, &RegistryCodeId::default())?,
    }

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SetRegistryCodeId(code_ids) => execute_set_registry_code_id(deps.storage, code_ids),
        ExecuteMsg::InstantiateDao => execute_instantiate_dao(deps, info),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    let message_id = ReplyMessageId::try_from(msg.id)?;

    match message_id {
        ReplyMessageId::InstantiateDao => Ok(Response::default()),
        ReplyMessageId::InstantiateProperty => Ok(Response::default()),
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::{mock_dependencies, mock_env};
    use cosmwasm_std::{coin, coins, from_json, Addr, Uint128};
    use cw_dao::state::{DAOCategory, DAOMetadata};
    use cw_multi_test::error::AnyResult;
    use cw_multi_test::{App, AppResponse, Contract, ContractWrapper, Executor};
    use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
    use crate::state::RegistryCodeId;
    
    #[derive(Clone, Debug, Copy)]
    pub struct FactoryCodeId(u64);

    impl FactoryCodeId {
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
        ) -> AnyResult<FactoryContract> {
            FactoryContract::instantiate(app, self, sender, label)
        }
    }

    pub struct FactoryContract(Addr);

    impl FactoryContract {
        pub fn addr(&self) -> Addr {
            self.0.clone()
        }
    
        #[allow(clippy::too_many_arguments)]
        #[track_caller]
        pub fn instantiate(
            app: &mut App,
            code_id: FactoryCodeId,
            sender: Addr,
            label: &str,
        ) -> AnyResult<Self> {
            let init_msg = InstantiateMsg {
                admins: None,
                owner: Some(sender.to_string()),
                code_ids: None,
            };
            app.instantiate_contract(code_id.0, sender, &init_msg, &[], label, None)
                .map(Self::from)
        }
    }

    impl From<Addr> for FactoryContract {
        fn from(value: Addr) -> Self {
            Self(value)
        }
    }

    pub struct DaoCodeId(u64);

    impl DaoCodeId {
        pub fn store_code(app: &mut App) -> Self {
            let contract = ContractWrapper::new(
                cw_dao::contract::execute,
                cw_dao::contract::instantiate,
                cw_dao::contract::query,
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
            let init_msg = cw_dao::msg::InstantiateMsg {
                admins: vec![],
                metadata: DAOMetadata {
                    name: "test".to_string(),
                    description: Some("test".to_string()),
                    // links: vec![],
                    symbol: Some("test".to_string()),
                    image_uri: None,
                    category: DAOCategory::Other,
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
        let factory_code_id = FactoryCodeId::store_code(&mut app);
        let dao_code_id = DaoCodeId::store_code(&mut app);
        let factory_addr = factory_code_id.instantiate(&mut app, owner.clone(), "test").unwrap();

        app.execute_contract(
            owner.clone(),
            factory_addr.addr(),
            &ExecuteMsg::SetRegistryCodeId(RegistryCodeId {
                dao_contract_code_id: Some(dao_code_id.0),
                property_contract_code_id: None,
                dao_token_code_id: None,
                dao_voting_contract_code_id: None,
                dao_treasury_contract_code_id: None,
            }),
            &[],
        )
        .unwrap();

        app.execute_contract(
            owner.clone(),
            factory_addr.addr(),
            &ExecuteMsg::InstantiateDao,
            &[],
        ).unwrap();
    }
}
