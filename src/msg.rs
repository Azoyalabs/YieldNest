use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin, Timestamp, Uint128};

use crate::structs::MintPositionRecord;

#[cw_serde]
pub struct InstantiateMsg {
    pub collateral_ratio: String,
    pub liquidation_fee_pct: String,
}

#[cw_serde]
pub enum AdminExecuteMsg {
    UpdateAdmin {
        new_admin: String,
    },
    CreateDebtToken {
        subdenom: String,
        expiry: Timestamp,
    },
    CreateMarket {
        base_denom: String,
        quote_denom: String,
    },
    RegisterMarket {
        base_currency: String,
        quote_currency: String,
        market_id: String,
    },
    MintDenom {
        mint_data: Coin,
    },
    /// Send a market order to be executed by the exchange module, to manage available liquidity
    MarketOrder {
        market_id: String,
        is_buy_order: bool,
    },
}

#[cw_serde]
pub enum ExecuteMsg {
    /// Mint a token using another as collateral  
    Mint {
        target_denom: String,
        quantity: Uint128,
    },
    /// Liquidate an undercollaterized position
    Liquidate {
        position_id: u64,
    },
    /// Redeem bond asset and swap back to usdt
    //RedeemDebtAsset {},
    /// Repay debt position
    Repay {
        position_id: u64,
    },
    Admin(AdminExecuteMsg),
}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetAdminResponse)]
    GetAdmin {},

    #[returns(GetUserMintPositionsResponse)]
    GetUserMintPositions { user_address: Addr },
}


#[cw_serde]
pub struct GetAdminResponse {
    pub admin: Option<Addr>,
}

// We define a custom struct for each query response
#[cw_serde]
pub struct GetUserMintPositionsResponse {
    pub mint_positions: Vec<MintPositionRecord>,
}
