use cosmwasm_std::{Coin, Uint128, Timestamp};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};



/// Status of a debt token: Live if not expired 
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum DebtTokenStatus {
    Live,
    Expired
}

impl DebtTokenStatus {
    pub fn from_expiration(expiration_time: Timestamp, curr_time: Timestamp) -> Self {
        return match expiration_time > curr_time {
            true => DebtTokenStatus::Live,
            false => DebtTokenStatus::Expired
        };
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DebtTokenData {
    
}




#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SampleStruct {}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BorrowPositionRecord {
    pub coin: Coin,
    pub rate: Uint128,
    pub expiry: Timestamp
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MintPositionRecord {
    pub collateral_asset: Coin,
    pub minted_asset: Coin
}