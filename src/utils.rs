use cosmwasm_std::{Coin, Storage, Timestamp};

use crate::{state::DEBT_EXPIRATION, structs::DebtTokenStatus, ContractError};

/*
/// Compute the price of a debt token according to discounted cash
/// Assume base value of 1
pub fn compute_valuation_debt_token_zero_coupon(curr_time: Timestamp, expiration_time: Timestamp, yield_rate: Decimal) {
    let base_price: u128 = 1;

    let present_value = Decimal::one() / (Decimal::one() + yield_rate).pow(exp);

    return present_value;
}
*/

/// Validate that there is a single Coin in funds sent with message and return Result<Coin>
pub fn validate_single_fund(funds: Vec<Coin>) -> Result<Coin, ContractError> {
    if funds.len() != 1 {
        return Err(ContractError::InvalidFunds {});
    } else {
        return Ok(funds[0].clone());
    }
}

/// Validate that the debt token was emitted by this contract and check if it has expired or not
pub fn validate_debt_token(
    storage: &mut dyn Storage,
    token_name: String,
    curr_time: Timestamp,
) -> Result<DebtTokenStatus, ContractError> {
    return match DEBT_EXPIRATION.load(storage, token_name) {
        Err(_) => Err(ContractError::UnknownToken {}),
        Ok(expiry) => Ok(DebtTokenStatus::from_expiration(expiry, curr_time)),
    };
}

/// Parse the individual components of the token name (creator, and ticker to expiry, duration, and underlying)
/// TokenFactory creates tokens with the following name: factory/{creator address}/{subdenom}.
pub fn parse_debt_token_components(denom_name: String, contract_address: String) {
    let raw_components: Vec<&str> = denom_name.split("/").collect();

    // must come from token factory module
    if raw_components[0].ne("factory") {}

    // must come from this contract
    if raw_components[1].ne(contract_address.as_str()) {}

    // parse expiry etc
}

//pub fn compute_collateralization_ratio(minted: Coin, collateral: Coin, deps: Deps) {}
