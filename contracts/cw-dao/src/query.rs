use cosmwasm_std::{Deps, StdResult, Addr};

use crate::state::{Config, DAOMetadata, DAOProperty, DAOStats, CONFIG, DAO_METADATA, DAO_PROPERTIES, DAO_STATS};

pub fn query_config(deps: Deps) -> StdResult<Config> {
    CONFIG.load(deps.storage)
}

pub fn query_metadata(deps: Deps) -> StdResult<DAOMetadata> {
    Ok(DAO_METADATA.load(deps.storage)?)
}

pub fn query_property_contract_code_id(deps: Deps) -> StdResult<Option<u64>> {
    Ok(CONFIG.load(deps.storage)?.property_contract_code_id)
}

pub fn query_all_properties(deps: Deps) -> StdResult<Vec<DAOProperty>> {
    let limit = 100;

    DAO_PROPERTIES
        .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
        .take(limit)
        .map(|item| item.map(|(address, mut property)| {
            // set the property contract address as a field instead of a map key
            property.property_contract_address = Some(address);
            property
        }))
        .collect::<StdResult<Vec<DAOProperty>>>()
}

pub fn query_property(deps: Deps, id: Addr) -> StdResult<Option<DAOProperty>> {
    Ok(DAO_PROPERTIES.may_load(deps.storage, id)?)
}

pub fn query_stats(deps: Deps) -> StdResult<DAOStats> {
    Ok(DAO_STATS.load(deps.storage)?)
}
