use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin, Timestamp, Uint128};
use injective_math::FPDecimal;

use crate::structs::{
    DebtTokenRecord, MintPositionRecord, MintPositionRecordWithCollateralRatio,
    RegisteredMarketRecord,
};

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
    RegisterDebtToken {
        denom: String,
        expiry: Timestamp,
    },
    /*
    CreateMarket {
        base_denom: String,
        quote_denom: String,
    },
    */
    RegisterMarket {
        base_currency: String,
        quote_currency: String,
        market_id: String,
    },
    RemoveMarket {
        base_currency: String,
        quote_currency: String,
    },

    MintDenom {
        mint_data: Coin,
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

    #[returns(GetUserMintPositionsWithCollateralRatioResponse)]
    GetUserMintPositionsWithCollateralRatio { user_address: Addr },

    #[returns(GetMintPositionResponse)]
    GetMintPosition { position_id: u64 },

    #[returns(GetBatchMintPositionsResponse)]
    GetBatchMintPositions { start_id: u64, count: u64 },

    #[returns(GetDebtTokensResponse)]
    GetDebtTokens {},

    #[returns(GetProtocolSettingsResponse)]
    GetProtocolSettings {},

    #[returns(GetRegisteredMarketsResponse)]
    GetRegisteredMarkets {},
}

#[cw_serde]
pub struct GetRegisteredMarketsResponse {
    pub markets: Vec<RegisteredMarketRecord>,
}

#[cw_serde]
pub struct GetBatchMintPositionsResponse {
    pub positions: Vec<MintPositionRecordWithCollateralRatio>,
}

#[cw_serde]
pub struct GetMintPositionResponse {
    pub position: Option<MintPositionRecordWithCollateralRatio>,
}

#[cw_serde]
pub struct GetDebtTokensResponse {
    pub tokens: Vec<DebtTokenRecord>,
}

#[cw_serde]
pub struct GetAdminResponse {
    pub admin: Option<Addr>,
}

#[cw_serde]
pub struct GetProtocolSettingsResponse {
    pub collateral_ratio: FPDecimal,
    pub liquidation_fee_pct: FPDecimal,
}

// We define a custom struct for each query response
#[cw_serde]
pub struct GetUserMintPositionsResponse {
    pub mint_positions: Vec<MintPositionRecord>,
}

#[cw_serde]
pub struct GetUserMintPositionsWithCollateralRatioResponse {
    pub mint_positions: Vec<MintPositionRecordWithCollateralRatio>,
}
