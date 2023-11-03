use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128, Coin, Timestamp};

use injective_cosmwasm::auction::query::QueryCurrentAuctionBasketResponse;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum AdminExecuteMsg {
    UpdateAdmin { new_admin: String },
    CreateDebtToken { subdenom: String, expiry: Timestamp },
    CreateMarket { base_denom: String, quote_denom: String },
    RegisterMarket { base_currency: String, quote_currency: String, market_id: String },
    MintDenom { mint_data: Coin },
    /// Send a market order to be executed by the exchange module, to manage available liquidity 
    MarketOrder { market_id: String, is_buy_order: bool },
}

#[cw_serde]
pub enum ExecuteMsg {
    /// Mint a token using another as collateral  
    Mint { target_denom: String, quantity: Uint128 },
    /// Liquidate an undercollaterized position 
    Liquidate { target_id: String },
    /// Redeem bond asset and swap back to usdt 
    RedeemDebtAsset { },
    /// Repay debt position
    Repay { position_id: u64}, 
    Admin(AdminExecuteMsg),
}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetAdminResponse)]
    GetAdmin {},

    // GetCount returns the current count as a json-encoded number
    #[returns(SampleQueryResponse)]
    SampleQuery {},

    #[returns(QueryCurrentAuctionBasketResponse)]
    GetCurrentAuctionBasket{}
}

// We define a custom struct for each query response
#[cw_serde]
pub struct SampleQueryResponse {
    pub value: bool,
}

#[cw_serde]
pub struct GetAdminResponse {
    pub admin: Option<Addr>,
}


