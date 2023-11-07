use std::collections::HashMap;

use cosmwasm_std::{to_json_binary, Addr, Binary, Deps, Env, StdResult};
use erased_serde::Serialize;
use injective_cosmwasm::{InjectiveQuerier, InjectiveQueryWrapper};
use injective_math::FPDecimal;

use crate::{
    msg::{
        GetAdminResponse, GetDebtTokensResponse, GetUserMintPositionsResponse,
        GetUserMintPositionsWithCollateralResponse, QueryMsg,
    },
    state::{ADMIN, DEBT_EXPIRATION, MARKET_IDS, MINT_POSITIONS, USER_MINT_POSITIONS},
    structs::{DebtTokenRecord, MintPositionRecord, MintPositionRecordWithCollateralRatio},
    ContractError,
};

pub fn route_query(
    deps: Deps<InjectiveQueryWrapper>,
    _env: Env,
    msg: QueryMsg,
) -> StdResult<Binary> {
    let res: Box<dyn Serialize> = match msg {
        QueryMsg::GetAdmin {} => get_admin(deps),
        QueryMsg::GetUserMintPositions { user_address } => {
            get_user_mint_positions(deps, user_address)
        }
        QueryMsg::GetUserMintPositionsWithCollateral { user_address } => {
            get_user_mint_positions_with_collateral_ratio(deps, user_address)
        }
        QueryMsg::GetDebtTokens {} => get_debt_tokens(deps),
    };

    return Ok(to_json_binary(&res)?);
}

fn get_admin(deps: Deps<InjectiveQueryWrapper>) -> Box<dyn Serialize> {
    return Box::new(GetAdminResponse {
        admin: ADMIN.load(deps.storage).ok(),
    });
}

fn get_debt_tokens(deps: Deps<InjectiveQueryWrapper>) -> Box<dyn Serialize> {
    let debt_tokens: Vec<DebtTokenRecord> = DEBT_EXPIRATION
        .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
        .filter_map(|val| match val {
            Err(_) => None,
            Ok((denom, expiry)) => Some(DebtTokenRecord { denom, expiry }),
        })
        .collect();

    return Box::new(GetDebtTokensResponse {
        tokens: debt_tokens,
    });
}

fn get_user_mint_positions(
    deps: Deps<InjectiveQueryWrapper>,
    user_address: Addr,
) -> Box<dyn Serialize> {
    let id_positions = match USER_MINT_POSITIONS.load(deps.storage, user_address) {
        Ok(pos) => pos,
        Err(_) => vec![],
    };

    let positions = id_positions
        .iter()
        .map(|id_pos| MINT_POSITIONS.load(deps.storage, *id_pos).unwrap().1)
        .collect();

    return Box::new(GetUserMintPositionsResponse {
        mint_positions: positions,
    });
}

fn get_user_mint_positions_with_collateral_ratio(
    deps: Deps<InjectiveQueryWrapper>,
    user_address: Addr,
) -> Box<dyn Serialize> {
    let id_positions = match USER_MINT_POSITIONS.load(deps.storage, user_address) {
        Ok(pos) => pos,
        Err(_) => vec![],
    };

    let positions: Vec<MintPositionRecord> = id_positions
        .iter()
        .map(|id_pos| MINT_POSITIONS.load(deps.storage, *id_pos).unwrap().1)
        .collect();

    // store exchange rates to avoid repeated queries for the same pair
    let mut exchange_rates: HashMap<(String, String), FPDecimal> = HashMap::new();

    // get exchange rate inj usdt
    let inj_usdt_market_id = MARKET_IDS
        .load(deps.storage, ("inj".to_string(), "usdt".to_string()))
        .unwrap();

    let querier = InjectiveQuerier::new(&deps.querier);
    let midprice_inj_usdt = querier
        .query_spot_market_mid_price_and_tob(&inj_usdt_market_id.as_str())
        .unwrap()
        .mid_price
        .unwrap();

    let mut positions_with_collateral_ratio: Vec<MintPositionRecordWithCollateralRatio> = vec![];
    for i in 0..positions.len() {
        // is the exchange rate in the hashmap?
        let price_debt_usdt = match exchange_rates
            .get(&(positions[i].minted_asset.denom.clone(), "usdt".to_string()))
        {
            Some(rate) => *rate,
            None => {
                let market_id = MARKET_IDS
                    .load(
                        deps.storage,
                        (positions[i].minted_asset.denom.clone(), "usdt".to_string()),
                    )
                    .unwrap();

                let debt_usdt_rate = querier
                    .query_spot_market_mid_price_and_tob(&market_id.as_str())
                    .unwrap()
                    .mid_price
                    .unwrap();

                exchange_rates.insert(
                    (positions[i].minted_asset.denom.clone(), "usdt".to_string()),
                    debt_usdt_rate,
                );

                debt_usdt_rate
            }
        };

        // now compute collateral ratio
        let collateral_ratio = (price_debt_usdt
            .mul(positions[i].minted_asset.amount.u128() as i128))
            / (midprice_inj_usdt.mul(positions[i].collateral_asset.amount.u128() as i128));

        // return pos
        positions_with_collateral_ratio.push(MintPositionRecordWithCollateralRatio {
            collateral_asset: positions[i].collateral_asset.clone(),
            minted_asset: positions[i].minted_asset.clone(),
            collateral_ratio: collateral_ratio,
        });
    }

    return Box::new(GetUserMintPositionsWithCollateralResponse {
        mint_positions: positions_with_collateral_ratio,
    });
}
