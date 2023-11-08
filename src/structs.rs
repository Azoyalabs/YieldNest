use cosmwasm_std::{Addr, Coin, Timestamp, Uint128};
use injective_math::FPDecimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Status of a debt token: Live if not expired
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum DebtTokenStatus {
    Live,
    Expired,
}

impl DebtTokenStatus {
    pub fn from_expiration(expiration_time: Timestamp, curr_time: Timestamp) -> Self {
        return match expiration_time > curr_time {
            true => DebtTokenStatus::Live,
            false => DebtTokenStatus::Expired,
        };
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DebtTokenData {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SampleStruct {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BorrowPositionRecord {
    pub coin: Coin,
    pub rate: Uint128,
    pub expiry: Timestamp,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MintPositionRecord {
    pub collateral_asset: Coin,
    pub minted_asset: Coin,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MintPositionRecordWithCollateralRatio {
    pub position_id: u64,
    pub minter: Addr,
    pub collateral_asset: Coin,
    pub minted_asset: Coin,
    pub collateral_ratio: FPDecimal,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DebtTokenRecord {
    pub denom: String,
    pub expiry: Timestamp,
    pub market_record: MarketRecord,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MarketRecord {
    pub ticker: String,
    pub market_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RegisteredMarketRecord {
    pub base_currency: String,
    pub quote_currency: String,
    pub market_id: String,
}
