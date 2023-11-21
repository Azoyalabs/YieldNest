use std::str::FromStr;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use injective_cosmwasm::{InjectiveMsgWrapper, InjectiveQueryWrapper};
use injective_math::FPDecimal;

use crate::contract_admin_execute::route_admin_execute;
use crate::contract_execute::route_execute;

use crate::contract_query::route_query;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};

use crate::error::ContractError;
use crate::state::{ADMIN, COLLATERAL_RATIO, LIQUIDATION_FEE_PCT, TRACKER_MINT_ID};

use cw2::set_contract_version;

// version info for migration info
const CONTRACT_NAME: &str = "AzoyaLabs:LendProtocol";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<InjectiveQueryWrapper>,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response<InjectiveMsgWrapper>, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    ADMIN.save(deps.storage, &info.sender)?;

    LIQUIDATION_FEE_PCT.save(
        deps.storage,
        &FPDecimal::from_str(&msg.liquidation_fee_pct).unwrap(),
    )?;

    COLLATERAL_RATIO.save(
        deps.storage,
        &FPDecimal::from_str(&msg.collateral_ratio).unwrap(),
    )?;

    TRACKER_MINT_ID.save(deps.storage, &0)?;

    return Ok(Response::new());
}

#[entry_point]
pub fn migrate(
    _deps: DepsMut<InjectiveQueryWrapper>,
    _env: Env,
    _msg: MigrateMsg,
) -> Result<Response<InjectiveMsgWrapper>, ContractError> {

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
