use cosmwasm_std::{Addr, Timestamp};
use cw_storage_plus::{Item, Map};
use injective_math::FPDecimal;

use crate::structs::MintPositionRecord;

pub const ADMIN: Item<Addr> = Item::new("admin");

/// Map from denoms to market ids
/// For example, factory/this_contract/test/usdt to its market id for queries and execute   
pub const MARKET_IDS: Map<(String, String), String> = Map::new("market_ids");

//pub const COLLATERAL_RATIOS: Map<String, Uint128> = Map::new("collateral_ratios");
pub const COLLATERAL_RATIO: Item<FPDecimal> = Item::new("collateral_ratio");

pub const LIQUIDATION_FEE_PCT: Item<FPDecimal> = Item::new("liquidation_fee_pct");


/// Unique per position ID
pub const TRACKER_MINT_ID: Item<u64> = Item::new("tracker_mint_id");

/// Tracking positions
pub const MINT_POSITIONS: Map<u64, (Addr, MintPositionRecord)> = Map::new("mint_positions");
pub const USER_MINT_POSITIONS: Map<Addr, Vec<u64>> = Map::new("user_mint_positions");

/// Tracking the expiration timestamp of debt tokens  
pub const DEBT_EXPIRATION: Map<String, Timestamp> = Map::new("debt_expiration");

