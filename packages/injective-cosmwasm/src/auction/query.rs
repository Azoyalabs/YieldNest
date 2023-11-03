use cosmwasm_std::{Coin, Addr};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

//use crate::wasmx::types::RegisteredContract;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct QueryCurrentAuctionBasketResponse {
    pub amount: Vec<Coin>,
    pub auction_round: u64,
    pub auction_closing_time: i64,
    pub highest_bidder: Addr,
    pub highest_bid_amount: String
}

