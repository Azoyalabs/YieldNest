use cosmwasm_std::{BankMsg, Coin, CosmosMsg, DepsMut, Env, MessageInfo, Response, Uint128};
use injective_cosmwasm::{
    create_burn_tokens_msg, create_mint_tokens_msg, InjectiveMsgWrapper, InjectiveQuerier,
    InjectiveQueryWrapper,
};

use crate::{
    msg::ExecuteMsg,
    state::{
        COLLATERAL_RATIO, LIQUIDATION_FEE_PCT, MARKET_IDS, MINT_POSITIONS, TRACKER_MINT_ID,
        USER_MINT_POSITIONS,
    },
    structs::{DebtTokenStatus, MintPositionRecord},
    utils::{validate_debt_token, validate_single_fund},
    ContractError,
};

pub fn route_execute(
    deps: DepsMut<InjectiveQueryWrapper>,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<InjectiveMsgWrapper>, ContractError> {
    match msg {
        ExecuteMsg::Mint {
            target_denom,
            quantity,
        } => execute_mint(deps, env, info, target_denom, quantity),
        ExecuteMsg::Liquidate { position_id } => execute_liquidate(deps, env, info, position_id),
        ExecuteMsg::Repay { position_id } => execute_repay(deps, env, info, position_id),
        //ExecuteMsg::RedeemDebtAsset {} => execute_redeem(deps, env, info),

        // shouldn't happen here
        ExecuteMsg::Admin(_) => return Err(ContractError::Never {}),
    }
}

/*
/// Redeem debt token, receive debt token and send usdt back
fn execute_redeem(
    deps: DepsMut<InjectiveQueryWrapper>,
    env: Env,
    info: MessageInfo,
) -> Result<Response<InjectiveMsgWrapper>, ContractError> {
    // validate funds
    let user_funds = validate_single_fund(info.funds)?;

    // is it a known debt token and has the debt token expired?
    match DEBT_EXPIRATION.load(deps.storage, user_funds.denom.clone()) {
        Ok(expiration_time) => {
            if expiration_time < env.block.time {
                return Err(ContractError::Std(StdError::GenericErr {
                    msg: "Token has not expired yet".to_string(),
                }));
            }
        }
        Err(_) => {
            // not a known debt token
            return Err(ContractError::Std(StdError::GenericErr {
                msg: "Unknown token".to_string(),
            }));
        }
    }

    // redeem equivalent amount of usdt
    let redeem_funds_msg = BankMsg::Send {
        to_address: info.sender.into_string(),
        amount: vec![Coin {
            amount: user_funds.amount,
            denom: "usdt".to_string(),
        }],
    };

    // burn debt token

    return Ok(
        Response::new().add_message(redeem_funds_msg)
    );
}
*/

/// Repay a mint position
fn execute_repay(
    deps: DepsMut<InjectiveQueryWrapper>,
    env: Env,
    info: MessageInfo,
    position_id: u64, //target_denom: String,
) -> Result<Response<InjectiveMsgWrapper>, ContractError> {
    // validate funds
    let user_funds = validate_single_fund(info.funds)?;

    // get data of the debt position
    let (minter, mut debt_position_data) = match MINT_POSITIONS.load(deps.storage, position_id) {
        Ok(pos) => pos,
        Err(_) => return Err(ContractError::InvalidPositionId {}),
    };

    // check if match debt token issued for repayment
    if debt_position_data.minted_asset.denom.ne(&user_funds.denom) {
        return Err(ContractError::MismatchDenomRedeem {
            denom_debt: debt_position_data.minted_asset.denom,
        });
    }

    // check if not sending too much
    if user_funds.amount > debt_position_data.minted_asset.amount {
        return Err(ContractError::InvalidFunds {});
    }

    // modify debt data and prep settlement messages
    let mut msgs: Vec<CosmosMsg<InjectiveMsgWrapper>> = vec![];

    // burn debt token
    let burn_msg = create_burn_tokens_msg(env.contract.address, user_funds.clone());
    msgs.push(burn_msg);

    // update debt_position data
    debt_position_data.minted_asset.amount -= user_funds.amount;
    if debt_position_data.minted_asset.amount == Uint128::zero() {
        // if fully refunded, delete position
        MINT_POSITIONS.remove(deps.storage, position_id);

        // and create refund collateral message
        msgs.push(CosmosMsg::Bank(BankMsg::Send {
            to_address: minter.into_string(),
            amount: vec![debt_position_data.collateral_asset.clone()],
        }));
    } else {
        // else update debt data
        MINT_POSITIONS.save(deps.storage, position_id, &(minter, debt_position_data))?;
    }

    return Ok(Response::new().add_messages(msgs));
}

/// Minting debt token
fn execute_mint(
    deps: DepsMut<InjectiveQueryWrapper>,
    env: Env,
    info: MessageInfo,
    target_denom: String,
    quantity_to_mint: Uint128,
) -> Result<Response<InjectiveMsgWrapper>, ContractError> {
    // validate deposit
    let user_funds = validate_single_fund(info.funds)?;

    // require that deposit is in inj
    if user_funds.denom != "inj" {
        return Err(ContractError::InjRequiredMint {});
    }

    // check if valid target denom (denom emitted by this contract, and token has not expired )
    if let DebtTokenStatus::Expired {} =
        validate_debt_token(deps.storage, target_denom.clone(), env.block.time)?
    {
        return Err(ContractError::TokenHasExpired {});
    }

    // value inj deposit in usdt
    let querier = InjectiveQuerier::new(&deps.querier);
    let market_id_inj_usdt =
        MARKET_IDS.load(deps.storage, ("inj".to_string(), ("usdt".to_string())))?;
    let midprice_inj_usdt = match querier
        .query_spot_market_mid_price_and_tob(&market_id_inj_usdt.as_str())
        .unwrap()
        .mid_price
    {
        None => return Err(ContractError::NoValidMidpriceFound {}),
        Some(val) => val,
    };

    let deposit_value_usdt = midprice_inj_usdt.mul(user_funds.amount.u128() as i128);

    // get market id for the debt token
    // need to register market ids? coz it's a hash, not a string from the denoms
    let debt_market_id =
        match MARKET_IDS.load(deps.storage, (target_denom.clone(), "usdt".to_string())) {
            Ok(val) => val,
            Err(_) => return Err(ContractError::UnknownMarket {}),
        };

    // get exchange midprice
    let midprice_debt_market = match querier
        .query_spot_market_mid_price_and_tob(&debt_market_id.as_str())
        .unwrap()
        .mid_price
    {
        None => return Err(ContractError::NoValidMidpriceFound {}),
        Some(val) => val,
    };

    let debt_value_usdt = midprice_debt_market.mul(quantity_to_mint.u128() as i128);

    // ensure that collateral is enough
    let required_collateral_ratio = COLLATERAL_RATIO.load(deps.storage)?;

    if debt_value_usdt / deposit_value_usdt > required_collateral_ratio {
        return Err(ContractError::InsufficientCollateral {
            required_collateral_ratio: required_collateral_ratio,
        });
    }

    // record in state
    let current_mint_id = TRACKER_MINT_ID.load(deps.storage)?;
    // first add the id of the position to the list of open positions for a user
    USER_MINT_POSITIONS.update(
        deps.storage,
        info.sender.clone(),
        |user_mint_positions| -> Result<_, ContractError> {
            let user_mint_positions = match user_mint_positions {
                Some(mut pos) => {
                    pos.push(current_mint_id);
                    pos
                }
                None => {
                    vec![current_mint_id]
                }
            };

            return Ok(user_mint_positions);
        },
    )?;
    // and the record it in the mint positions mapping
    let mint_position_record = MintPositionRecord {
        collateral_asset: user_funds.clone(),
        minted_asset: Coin {
            denom: target_denom.clone(),
            amount: quantity_to_mint,
        },
    };
    MINT_POSITIONS.save(
        deps.storage,
        current_mint_id,
        &(info.sender.clone(), mint_position_record),
    )?;

    // increment position id tracker
    TRACKER_MINT_ID.save(deps.storage, &(current_mint_id + 1))?;

    // send mint message
    let mint_msg_data = Coin {
        denom: target_denom,
        amount: quantity_to_mint,
    };
    let mint_msg =
        create_mint_tokens_msg(env.contract.address, mint_msg_data, info.sender.to_string());

    return Ok(Response::new().add_message(mint_msg));
}

/// Liquidate an undercollaterized debt position
fn execute_liquidate(
    deps: DepsMut<InjectiveQueryWrapper>,
    env: Env,
    info: MessageInfo,
    position_id: u64,
) -> Result<Response<InjectiveMsgWrapper>, ContractError> {
    // validate deposit
    let user_funds = validate_single_fund(info.funds)?;

    // get position at id
    let (minter, position_data) = match MINT_POSITIONS.load(deps.storage, position_id) {
        Err(_) => return Err(ContractError::InvalidPositionId {}),
        Ok(data) => data,
    };

    // check if has sent enough funds
    if user_funds != position_data.minted_asset {
        return Err(ContractError::MismatchFundsMintedAssetsInLiquidation {});
    }

    let querier = InjectiveQuerier::new(&deps.querier);

    // check if below collateralization ratio
    let collateral_usdt_market_id = MARKET_IDS.load(
        deps.storage,
        (
            position_data.collateral_asset.denom.clone(),
            "usdt".to_string(),
        ),
    )?;
    let midprice_collateral_usdt = match querier
        .query_spot_market_mid_price_and_tob(&collateral_usdt_market_id.as_str())
        .unwrap()
        .mid_price
    {
        None => return Err(ContractError::NoValidMidpriceFound {}),
        Some(val) => val,
    };
    let collateral_value =
        midprice_collateral_usdt.mul(position_data.collateral_asset.amount.u128() as i128);

    let debt_usdt_market_id = MARKET_IDS.load(
        deps.storage,
        (position_data.minted_asset.denom, "usdt".to_string()),
    )?;
    let midprice_debt_usdt = match querier
        .query_spot_market_mid_price_and_tob(&debt_usdt_market_id.as_str())
        .unwrap()
        .mid_price
    {
        None => return Err(ContractError::NoValidMidpriceFound {}),
        Some(val) => val,
    };
    let debt_value = midprice_debt_usdt.mul(position_data.minted_asset.amount.u128() as i128);

    // get collateral ratio expected
    let collateral_ratio = COLLATERAL_RATIO.load(deps.storage)?;

    // check if can liquidate
    if debt_value / collateral_value <= collateral_ratio {
        return Err(ContractError::PositionEnoughCapitalNoLiquidation {});
    }

    // process liquidation
    // compute collateral from minted, fee for liquidator and amount to sent back to user
    let collateral_from_minted: Uint128 = (debt_value / midprice_collateral_usdt).to_u256().try_into().unwrap();
    
    let fee_liquidator_pct = LIQUIDATION_FEE_PCT.load(deps.storage)?;
    let fee_liquidator_amount: Uint128 = fee_liquidator_pct
        .mul(position_data.collateral_asset.amount.u128() as i128)
        .to_u256()
        .try_into()
        .unwrap();

    let collateral_sent_back = position_data.collateral_asset.amount - collateral_from_minted - fee_liquidator_amount;

    // prep messages
    let mut msgs: Vec<CosmosMsg<InjectiveMsgWrapper>> = vec![];

    // send fee + liquidated collateral to liquidator
    msgs.push(
        BankMsg::Send {
            to_address: info.sender.to_string(),
            amount: vec![Coin {
                amount: collateral_from_minted + fee_liquidator_amount,
                denom: position_data.collateral_asset.denom.clone(),
            }],
        }
        .into(),
    );

    // return rest of collateral to minter
    msgs.push(
        BankMsg::Send {
            to_address: minter.clone().into_string(),
            amount: vec![Coin {
                amount: collateral_sent_back,
                denom: position_data.collateral_asset.denom.clone(),
            }],
        }
        .into(),
    );

    // burn debt token
    msgs.push(create_burn_tokens_msg(
        env.contract.address,
        user_funds.clone(),
    ));

    // now write to state
    // delete record position
    MINT_POSITIONS.remove(deps.storage, position_id);
    // remove from list of user positions
    USER_MINT_POSITIONS.update(
        deps.storage,
        minter,
        |user_positions| -> Result<_, ContractError> {
            let user_positions = user_positions
                .unwrap()
                .into_iter()
                .filter(|curr_id| curr_id.ne(&position_id))
                .collect();

            return Ok(user_positions);
        },
    )?;

    return Ok(Response::new().add_messages(msgs));
}
