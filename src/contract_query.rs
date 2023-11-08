use std::collections::HashMap;

use cosmwasm_std::{to_json_binary, Addr, Binary, Deps, Env, StdResult};
use cw_storage_plus::Bound;
use erased_serde::Serialize;
use injective_cosmwasm::{InjectiveQuerier, InjectiveQueryWrapper};
use injective_math::FPDecimal;

use crate::{
    msg::{
        GetAdminResponse, GetBatchMintPositionsResponse, GetDebtTokensResponse,
        GetMintPositionResponse, GetProtocolSettingsResponse, GetRegisteredMarketsResponse,
        GetUserMintPositionsResponse, GetUserMintPositionsWithCollateralRatioResponse, QueryMsg,
    },
    state::{
        ADMIN, COLLATERAL_RATIO, DEBT_EXPIRATION, LIQUIDATION_FEE_PCT, MARKET_IDS, MINT_POSITIONS,
        USER_MINT_POSITIONS,
    },
    structs::{
        DebtTokenRecord, MarketRecord, MintPositionRecord, MintPositionRecordWithCollateralRatio,
        RegisteredMarketRecord,
    },
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
        QueryMsg::GetUserMintPositionsWithCollateralRatio { user_address } => {
            get_user_mint_positions_with_collateral_ratio(deps, user_address)
        }
        QueryMsg::GetMintPosition { position_id } => get_mint_position(deps, position_id),
        QueryMsg::GetDebtTokens {} => get_debt_tokens(deps),
        QueryMsg::GetProtocolSettings {} => get_protocol_settings(deps),
        QueryMsg::GetBatchMintPositions { start_id, count } => {
            get_batch_mint_positions(deps, start_id, count)
        }
        QueryMsg::GetRegisteredMarkets {} => get_registered_markets(deps),
    };

    return Ok(to_json_binary(&res)?);
}

fn get_registered_markets(deps: Deps<InjectiveQueryWrapper>) -> Box<dyn Serialize> {
    let registered_markets = MARKET_IDS
        .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
        .filter_map(|rec| match rec {
            Err(_) => None,
            Ok(((base_currency, quote_currency), market_id)) => Some(RegisteredMarketRecord {
                base_currency,
                quote_currency,
                market_id,
            }),
        })
        .collect();

    return Box::new(GetRegisteredMarketsResponse {
        markets: registered_markets,
    });
}

fn get_batch_mint_positions(
    deps: Deps<InjectiveQueryWrapper>,
    start_id: u64,
    count: u64,
) -> Box<dyn Serialize> {
    let positions: Vec<(u64, (Addr, MintPositionRecord))> = MINT_POSITIONS
        .range(
            deps.storage,
            Some(Bound::inclusive(start_id)),
            None,
            cosmwasm_std::Order::Ascending,
        )
        .filter_map(|elem| elem.ok())
        .take(count as usize)
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
        let price_debt_usdt = match exchange_rates.get(&(
            positions[i].1 .1.minted_asset.denom.clone(),
            "usdt".to_string(),
        )) {
            Some(rate) => *rate,
            None => {
                let market_id = MARKET_IDS
                    .load(
                        deps.storage,
                        (
                            positions[i].1 .1.minted_asset.denom.clone(),
                            "usdt".to_string(),
                        ),
                    )
                    .unwrap();

                let debt_usdt_rate = querier
                    .query_spot_market_mid_price_and_tob(&market_id.as_str())
                    .unwrap()
                    .mid_price
                    .unwrap();

                exchange_rates.insert(
                    (
                        positions[i].1 .1.minted_asset.denom.clone(),
                        "usdt".to_string(),
                    ),
                    debt_usdt_rate,
                );

                debt_usdt_rate
            }
        };

        // now compute collateral ratio
        let collateral_ratio = (price_debt_usdt
            .mul(positions[i].1 .1.minted_asset.amount.u128() as i128))
            / (midprice_inj_usdt.mul(positions[i].1 .1.collateral_asset.amount.u128() as i128));

        // return pos
        positions_with_collateral_ratio.push(MintPositionRecordWithCollateralRatio {
            position_id: positions[i].0,
            minter: positions[i].1 .0.clone(),
            collateral_asset: positions[i].1 .1.collateral_asset.clone(),
            minted_asset: positions[i].1 .1.minted_asset.clone(),
            collateral_ratio: collateral_ratio,
        });
    }

    return Box::new(GetBatchMintPositionsResponse {
        positions: positions_with_collateral_ratio,
    });
}

fn get_mint_position(deps: Deps<InjectiveQueryWrapper>, position_id: u64) -> Box<dyn Serialize> {
    let (minter, position_data) = match MINT_POSITIONS.load(deps.storage, position_id) {
        Err(_) => return Box::new(GetMintPositionResponse { position: None }),
        Ok(data) => data,
    };

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

    let market_id = MARKET_IDS
        .load(
            deps.storage,
            (position_data.minted_asset.denom.clone(), "usdt".to_string()),
        )
        .unwrap();

    let price_debt_usdt = querier
        .query_spot_market_mid_price_and_tob(&market_id.as_str())
        .unwrap()
        .mid_price
        .unwrap();

    // now compute collateral ratio
    let collateral_ratio = (price_debt_usdt.mul(position_data.minted_asset.amount.u128() as i128))
        / (midprice_inj_usdt.mul(position_data.collateral_asset.amount.u128() as i128));

    return Box::new(GetMintPositionResponse {
        position: Some(MintPositionRecordWithCollateralRatio {
            position_id: position_id,
            minter: minter,
            collateral_asset: position_data.collateral_asset,
            minted_asset: position_data.minted_asset,
            collateral_ratio: collateral_ratio,
        }),
    });
}

fn get_admin(deps: Deps<InjectiveQueryWrapper>) -> Box<dyn Serialize> {
    return Box::new(GetAdminResponse {
        admin: ADMIN.load(deps.storage).ok(),
    });
}

fn get_protocol_settings(deps: Deps<InjectiveQueryWrapper>) -> Box<dyn Serialize> {
    return Box::new(GetProtocolSettingsResponse {
        liquidation_fee_pct: LIQUIDATION_FEE_PCT.load(deps.storage).unwrap(),
        collateral_ratio: COLLATERAL_RATIO.load(deps.storage).unwrap(),
    });
}

fn get_debt_tokens(deps: Deps<InjectiveQueryWrapper>) -> Box<dyn Serialize> {
    let debt_tokens: Vec<DebtTokenRecord> = DEBT_EXPIRATION
        .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
        .filter_map(|val| match val {
            Err(_) => None,
            Ok((denom, expiry)) => {
                // get token market id
                let market_id = MARKET_IDS
                    .load(deps.storage, (denom.clone(), "usdt".to_string()))
                    .unwrap();
                let market_ticker = format!("{}/usdt", denom);
                Some(DebtTokenRecord {
                    denom,
                    expiry,
                    market_record: MarketRecord {
                        market_id: market_id,
                        ticker: market_ticker,
                    },
                })
            }
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
    let id_positions = match USER_MINT_POSITIONS.load(deps.storage, user_address.clone()) {
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
            position_id: id_positions[i] as u64,
            minter: user_address.clone(),
            collateral_asset: positions[i].collateral_asset.clone(),
            minted_asset: positions[i].minted_asset.clone(),
            collateral_ratio: collateral_ratio,
        });
    }

    return Box::new(GetUserMintPositionsWithCollateralRatioResponse {
        mint_positions: positions_with_collateral_ratio,
    });
}
