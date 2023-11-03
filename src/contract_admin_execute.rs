use cosmwasm_std::{Coin, DepsMut, Env, MessageInfo, Response, Timestamp};
use injective_cosmwasm::{
    create_mint_tokens_msg, create_new_denom_msg, InjectiveMsgWrapper, InjectiveQueryWrapper,
};

use crate::{
    msg::AdminExecuteMsg,
    state::{ADMIN, DEBT_EXPIRATION, MARKET_IDS},
    ContractError,
};

pub fn route_admin_execute(
    deps: DepsMut<InjectiveQueryWrapper>,
    env: Env,
    info: MessageInfo,
    msg: AdminExecuteMsg,
) -> Result<Response<InjectiveMsgWrapper>, ContractError> {
    // validate caller is admin
    if !match ADMIN.load(deps.storage) {
        Ok(admin) => admin == info.sender,
        Err(_) => false,
    } {
        return Err(ContractError::Unauthorized {});
    }

    match msg {
        AdminExecuteMsg::UpdateAdmin { new_admin } => update_admin(deps, new_admin),
        AdminExecuteMsg::CreateDebtToken { subdenom, expiry } => {
            execute_create_debt_token(deps, env, subdenom, expiry)
        }
        AdminExecuteMsg::MintDenom { mint_data } => execute_mint_denom(deps, env, info, mint_data),
        AdminExecuteMsg::CreateMarket {
            base_denom,
            quote_denom,
        } => execute_create_market(deps, env, base_denom, quote_denom),
        AdminExecuteMsg::RegisterMarket {
            base_currency,
            quote_currency,
            market_id,
        } => execute_register_market(deps, base_currency, quote_currency, market_id),
        AdminExecuteMsg::MarketOrder {
            market_id,
            is_buy_order,
        } => execute_market_order(deps, market_id, is_buy_order),
    }
}

fn execute_market_order(
    deps: DepsMut<InjectiveQueryWrapper>,
    market_id: String,
    is_buy_order: bool,
) -> Result<Response<InjectiveMsgWrapper>, ContractError> {
    return Ok(Response::new());
}

fn execute_register_market(
    deps: DepsMut<InjectiveQueryWrapper>,
    base_currency: String,
    quote_currency: String,
    market_id: String,
) -> Result<Response<InjectiveMsgWrapper>, ContractError> {
    MARKET_IDS.save(deps.storage, (base_currency, quote_currency), &market_id)?;

    return Ok(Response::new());
}

fn execute_create_market(
    _deps: DepsMut<InjectiveQueryWrapper>,
    _env: Env,
    _base_denom: String,
    _quote_denom: String,
) -> Result<Response<InjectiveMsgWrapper>, ContractError> {
    panic!("UNSUPPORTED - Must create market using client");

    //return Ok(Response::new());
}

/// Call the token factory module to create a new debt token
/// From docs: The tokenfactory module allows any account to create a new token with the name factory/{creator address}/{subdenom}.
fn execute_create_debt_token(
    deps: DepsMut<InjectiveQueryWrapper>,
    env: Env,
    subdenom: String,
    expiry: Timestamp,
) -> Result<Response<InjectiveMsgWrapper>, ContractError> {
    // register expiry
    let denom_name = format!("factory/{}/{}", env.contract.address.to_string(), subdenom);
    DEBT_EXPIRATION.save(deps.storage, denom_name, &expiry)?;

    let new_denom_msg = create_new_denom_msg(env.contract.address.to_string(), subdenom);

    return Ok(Response::new().add_message(new_denom_msg));
}

fn execute_mint_denom(
    _deps: DepsMut<InjectiveQueryWrapper>,
    env: Env,
    info: MessageInfo,
    mint_data: Coin,
) -> Result<Response<InjectiveMsgWrapper>, ContractError> {
    // create mint msg & broadcast
    let mint_msg = create_mint_tokens_msg(env.contract.address, mint_data, info.sender.to_string());

    return Ok(Response::new().add_message(mint_msg));
}

fn update_admin(
    deps: DepsMut<InjectiveQueryWrapper>,
    new_admin: String,
) -> Result<Response<InjectiveMsgWrapper>, ContractError> {
    let new_admin = deps.api.addr_validate(&new_admin)?;

    ADMIN.update(deps.storage, |_| -> Result<_, ContractError> {
        return Ok(new_admin);
    })?;

    return Ok(Response::new());
}
