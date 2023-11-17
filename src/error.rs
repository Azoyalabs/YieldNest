use cosmwasm_std::StdError;
use injective_math::FPDecimal;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Never")]
    Never {},

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Message encoding failed")]
    MessageEncodingFailed {},

    #[error("Invalid Funds")]
    InvalidFunds {},

    #[error("Invalid denom - inj required for minting")]
    InjRequiredMint {},

    #[error("Unknown Token")]
    UnknownToken {},

    #[error("Unknown Market")]
    UnknownMarket {},

    #[error("Token has not expired yet")]
    TokenHasNotExpired {},

    #[error("Token has expired")]
    TokenHasExpired {},

    #[error("Insufficient collateral - must be at least {required_collateral_ratio}% - value collateral USD: {value_collateral_usd} - value debt usd: {value_debt_usd}")]
    InsufficientCollateral {
        required_collateral_ratio: FPDecimal,
        value_collateral_usd: FPDecimal,
        value_debt_usd: FPDecimal,
    },

    #[error("No valid midprice found")]
    NoValidMidpriceFound {},

    #[error("Invalid position id")]
    InvalidPositionId {},

    #[error("Mismatch denom for redeem - required {denom_debt}")]
    MismatchDenomRedeem { denom_debt: String },

    #[error("Position has enough collateral and cannot be liquidated")]
    PositionEnoughCapitalNoLiquidation {},

    #[error("Funds sent must match minted asset to liquidate a position")]
    MismatchFundsMintedAssetsInLiquidation {},
}
