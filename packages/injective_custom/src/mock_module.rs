use std::str::FromStr;

use cosmwasm_std::{
    testing::MockApi, to_json_binary, Addr, Empty, GovMsg, IbcMsg, IbcQuery, MemoryStorage,
    Storage, 
};
use cosmwasm_storage::prefixed;
use cw_multi_test::{
    App, AppBuilder, AppResponse, BankKeeper, DistributionKeeper, 
    FailingModule, Module, StakeKeeper, WasmKeeper,
};
use cw_storage_plus::Map;
use cw_utils::NativeBalance;
use injective_cosmwasm::{
    InjectiveMsg, InjectiveQuery, InjectiveQueryWrapper, InjectiveRoute,
    MarketMidPriceAndTOBResponse,
};

use anyhow::Result as AnyResult;
use injective_math::FPDecimal;

use crate::errors::InjectiveCustomError;

use sha2::{Digest, Sha256};

pub type InjectiveApp = App<
    BankKeeper,
    MockApi,
    MemoryStorage,
    CustomInjective,
    WasmKeeper<InjectiveMsg, InjectiveQueryWrapper>,
>;

pub trait FakeMarketCreator {
    fn create_market(&mut self, base_currency: String, quote_currency: String) -> String;

    //fn create_order(&mut self, sender: Addr, market_id: String);

    fn set_market_midprice(&mut self, market_id: String, midprice: FPDecimal);
}

impl FakeMarketCreator for InjectiveApp {
    fn create_market(&mut self, base_currency: String, quote_currency: String) -> String {
        let storage = self.storage_mut();
        let market_id = format!("{}_{}", base_currency, quote_currency);

        let mut hasher = Sha256::new();
        hasher.update(market_id.as_bytes());
        //let market_id = format!("0x{:02X?}", hasher.finalize().to_vec());
        let market_id = format!("0x{}", hex::encode(hasher.finalize()));

        storage.set(
            format!("markets_{}_{}", base_currency, quote_currency).as_bytes(),
            &market_id.as_bytes(),
        );

        return market_id;
    }

    //fn create_order(&mut self, sender: Addr, market_id: String) {}

    fn set_market_midprice(&mut self, market_id: String, midprice: FPDecimal) {
        let storage = self.storage_mut();
        storage.set(
            format!("midprice_{}", market_id).as_bytes(),
            midprice.to_string().as_bytes(),
        );
    }
}


pub struct CustomInjective {
}

impl CustomInjective {
    pub fn new() -> Self {
        return CustomInjective {

        };
    }
}

impl Module for CustomInjective {
    type ExecT = InjectiveMsg;
    type QueryT = InjectiveQueryWrapper;
    type SudoT = Empty;

    fn execute<ExecC, QueryC>(
        &self,
        api: &dyn cosmwasm_std::Api,
        storage: &mut dyn cosmwasm_std::Storage,
        router: &dyn cw_multi_test::CosmosRouter<ExecC = ExecC, QueryC = QueryC>,
        block: &cosmwasm_std::BlockInfo,
        sender: Addr,
        msg: Self::ExecT,
    ) -> AnyResult<cw_multi_test::AppResponse>
    where
        ExecC: std::fmt::Debug
            + Clone
            + PartialEq
            + schemars::JsonSchema
            + serde::de::DeserializeOwned
            + 'static,
        QueryC: cosmwasm_std::CustomQuery + serde::de::DeserializeOwned + 'static,
    {
        let msg_sender = sender;
        match msg {
            InjectiveMsg::CreateDenom { sender, subdenom } => {
                // does the subdenom exist?
                match storage.get(format!("tokenfactory/{}/{}", sender, subdenom).as_bytes()) {
                    None => {
                        storage.set(
                            format!("tokenfactory/{}/{}", sender, subdenom).as_bytes(),
                            sender.as_bytes(),
                        );
                        return Ok(AppResponse {
                            events: vec![],
                            data: None,
                        });
                    }
                    Some(_) => {
                        //panic!("denom already exists");
                        return Err(InjectiveCustomError::DenomAlreadyExists {}.into());
                    }
                }
            }
            InjectiveMsg::Mint {
                sender: _,
                amount,
                mint_to,
            } => {
                // check if sender is admin of denom
                match storage.get(amount.denom.as_bytes()) {
                    None => return Err(InjectiveCustomError::UnknownDenom {  }.into()),
                    Some(denom_admin) => {
                        assert_eq!(denom_admin, msg_sender.as_bytes());
                    }
                }

                return router.sudo(
                    api,
                    storage,
                    block,
                    cw_multi_test::SudoMsg::Bank(cw_multi_test::BankSudo::Mint {
                        to_address: mint_to,
                        amount: vec![amount],
                    }),
                );
            }
            InjectiveMsg::Burn { sender, amount } => {
                // burn not available in BankSudo, so do it by hand
                let mut bank_storage = prefixed(storage, b"bank");
                let balances_map: Map<&Addr, NativeBalance> = Map::new("balances");

                let new_balance = match balances_map.load(&bank_storage, &sender) {
                    Err(_) => {
                        panic!();
                    }
                    Ok(curr_balance) => (curr_balance - amount)?,
                };

                balances_map.save(&mut bank_storage, &sender, &new_balance)?;

                return Ok(AppResponse {
                    events: vec![],
                    data: None,
                });
            }
            _ => return Err(InjectiveCustomError::NotImplemented {}.into()),
        }
    }

    fn query(
        &self,
        _api: &dyn cosmwasm_std::Api,
        storage: &dyn cosmwasm_std::Storage,
        _querier: &dyn cosmwasm_std::Querier,
        _block: &cosmwasm_std::BlockInfo,
        request: Self::QueryT,
    ) -> AnyResult<cosmwasm_std::Binary> {
        match request {
            InjectiveQueryWrapper {
                route: InjectiveRoute::Exchange,
                query_data: InjectiveQuery::SpotMarketMidPriceAndTob { market_id },
            } => {
                let midprice =
                    match storage.get(format!("midprice_{}", market_id.as_str()).as_bytes()) {
                        Some(midprice) => midprice,
                        None => return Err(InjectiveCustomError::UnknownMarket {}.into()),
                    };
                let midprice = String::from_utf8(midprice)?;
                let midprice = FPDecimal::from_str(&midprice)?;

                let res = MarketMidPriceAndTOBResponse {
                    mid_price: Some(midprice),
                    best_buy_price: None,
                    best_sell_price: None,
                };

                return Ok(to_json_binary(&res)?);
            }
            _ => return Err(InjectiveCustomError::NotImplemented {}.into()),
        }
    }

    fn sudo<ExecC, QueryC>(
        &self,
        _api: &dyn cosmwasm_std::Api,
        _storage: &mut dyn cosmwasm_std::Storage,
        _router: &dyn cw_multi_test::CosmosRouter<ExecC = ExecC, QueryC = QueryC>,
        _block: &cosmwasm_std::BlockInfo,
        _msg: Self::SudoT,
    ) -> AnyResult<cw_multi_test::AppResponse>
    where
        ExecC: std::fmt::Debug
            + Clone
            + PartialEq
            + schemars::JsonSchema
            + serde::de::DeserializeOwned
            + 'static,
        QueryC: cosmwasm_std::CustomQuery + serde::de::DeserializeOwned + 'static,
    {
        return Err(InjectiveCustomError::NotImplemented {  }.into());
    }
}



pub fn create_app() -> App<
    BankKeeper,
    MockApi,
    MemoryStorage,
    CustomInjective,
    WasmKeeper<InjectiveMsg, InjectiveQueryWrapper>,
> {
    let builder: AppBuilder<
        BankKeeper,
        MockApi,
        MemoryStorage,
        FailingModule<InjectiveMsg, InjectiveQueryWrapper, Empty>,
        WasmKeeper<InjectiveMsg, InjectiveQueryWrapper>,
        StakeKeeper,
        DistributionKeeper,
        FailingModule<IbcMsg, IbcQuery, Empty>,
        FailingModule<GovMsg, Empty, Empty>,
    > = AppBuilder::new_custom();
    let builder = builder.with_custom(CustomInjective::new());

    return builder.build(|_, _, _| {});
}
