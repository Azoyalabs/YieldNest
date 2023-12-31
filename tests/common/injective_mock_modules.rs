use std::collections::HashMap;

use cosmwasm_std::{
    testing::MockApi, Addr, Empty, GovMsg, IbcMsg, IbcQuery, MemoryStorage, Uint128, WasmMsg,
    WasmQuery,
};
use cw_multi_test::{
    App, AppBuilder, BankKeeper, DistributionKeeper, Executor, FailingModule, Module, StakeKeeper,
    Wasm, WasmKeeper,
};
use injective_cosmwasm::{Hash, InjectiveMsg, InjectiveQuery, InjectiveQueryWrapper};
use lend_protocol::msg::InstantiateMsg;

use anyhow::{Context, Result as AnyResult};

pub struct TokenFactory {
    /// tracking balances for custom denoms? or can call bank?
    pub balances: HashMap<(String, String), Uint128>,
}

impl TokenFactory {
    pub fn new() -> Self {
        return TokenFactory {
            balances: HashMap::new(),
        };
    }

    pub fn mint(&self) {}
}

pub struct CustomInjective {
    pub token_factory: TokenFactory,
}

impl CustomInjective {
    pub fn new() -> Self {
        return CustomInjective {
            token_factory: TokenFactory::new(),
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
        match msg {
            InjectiveMsg::ActivateContract {
                sender,
                contract_address,
            } => {
                panic!("oh no") //Ok(())
            }
            //InjectiveMsg::Mint { sender, amount, mint_to } => self.token_factory.mint(),
            _ => panic!("yep"),
        }
    }

    fn query(
        &self,
        api: &dyn cosmwasm_std::Api,
        storage: &dyn cosmwasm_std::Storage,
        querier: &dyn cosmwasm_std::Querier,
        block: &cosmwasm_std::BlockInfo,
        request: Self::QueryT,
    ) -> AnyResult<cosmwasm_std::Binary> {
        panic!("in query");
    }

    fn sudo<ExecC, QueryC>(
        &self,
        api: &dyn cosmwasm_std::Api,
        storage: &mut dyn cosmwasm_std::Storage,
        router: &dyn cw_multi_test::CosmosRouter<ExecC = ExecC, QueryC = QueryC>,
        block: &cosmwasm_std::BlockInfo,
        msg: Self::SudoT,
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
        panic!("in sudo");
    }
}

/*
impl Module for WasmKeeper<InjectiveMsg, InjectiveQueryWrapper> {
}
*/

pub fn create_app() -> App<
    BankKeeper,
    MockApi,
    MemoryStorage,
    CustomInjective,
    WasmKeeper<InjectiveMsg, InjectiveQueryWrapper>,
> {
    //-> App<BankKeeper, MockApi, MemoryStorage, CustomInjective> {
    //let builder = AppBuilder::new_custom().with_custom(CustomInjective {});

    //let builder = AppBuilder::default().with_custom(CustomInjective {});

    let wasm_keeper: WasmKeeper<InjectiveMsg, InjectiveQueryWrapper> = WasmKeeper::new();
    /*
    let builder = AppBuilder::default()
        .with_wasm::<WasmKeeper<InjectiveMsg, InjectiveQueryWrapper>>(wasm_keeper);
    */

    /*
    let builder = AppBuilder::default()
        .with_wasm::<Box<dyn Wasm<WasmMsg, WasmQuery>>, WasmKeeper<InjectiveMsg, InjectiveQueryWrapper>>(wasm_keeper);
    */
    let builder = AppBuilder::new_custom();
    let builder = builder.with_custom(CustomInjective::new());
    //let builder = builder.with_wasm(WasmKeeper::<InjectiveMsg, InjectiveQueryWrapper>::new());

    return builder.build(|_, _, _| {});
}
