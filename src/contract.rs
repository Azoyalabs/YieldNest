#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use injective_cosmwasm::{InjectiveQueryWrapper, InjectiveMsgWrapper};

use crate::contract_admin_execute::route_admin_execute;
use crate::contract_execute::route_execute;

use crate::contract_query::route_query;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};

use crate::error::ContractError;
use crate::state::ADMIN;

use cw2::set_contract_version;

const ENFORCE_ADMIN: bool = true;

// version info for migration info
const CONTRACT_NAME: &str = "AzoyaLabs:GroupBid";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<InjectiveQueryWrapper>,
    env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response<InjectiveMsgWrapper>, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    ADMIN.save(deps.storage, &info.sender)?;

    /*
    if ENFORCE_ADMIN {
        assert!(deps
            .querier
            .query_wasm_contract_info(env.contract.address)?
            .admin
            .is_some());
    }
    */

    return Ok(Response::new());
}

#[entry_point]
pub fn migrate(_deps: DepsMut<InjectiveQueryWrapper>, _env: Env, _msg: MigrateMsg) -> Result<Response<InjectiveMsgWrapper>, ContractError> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<InjectiveQueryWrapper>,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<InjectiveMsgWrapper>, ContractError> {
    match msg {
        // Admin
        ExecuteMsg::Admin(admin_msg) => route_admin_execute(deps, env, info, admin_msg),
        // Default
        _ => route_execute(deps, env, info, msg),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<InjectiveQueryWrapper>, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    return route_query(deps, env, msg);
}
