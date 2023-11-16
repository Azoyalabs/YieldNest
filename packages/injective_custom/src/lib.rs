mod mock_module;

mod errors;

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use cosmwasm_std::{coin, Addr, CosmosMsg};
    use cw_multi_test::Executor;
    use injective_cosmwasm::{InjectiveMsg, InjectiveQueryWrapper, InjectiveQuery, MarketId, MarketMidPriceAndTOBResponse};
    use injective_math::FPDecimal;
    use injective_std::types::injective::exchange::v1beta1::QuerySpotMidPriceAndTobResponse;

    use crate::mock_module::{create_app, FakeMarketCreator};

    #[test]
    fn access_internal_state() {
        let mut app = create_app();

        let user_address = "user";
        let denom = "test_dollar";

        app.sudo(cw_multi_test::SudoMsg::Bank(
            cw_multi_test::BankSudo::Mint {
                to_address: user_address.into(),
                amount: vec![coin(1000, denom)],
            },
        ));

        //println!("{:?}", app.storage());
    }

    #[test]
    fn mint_through_custom() {
        let mut app = create_app();

        let user_address = "user";
        let user2_address = "user2";
        let denom = "test_dollar_tokenfactory";

        // create denom
        app.execute(
            Addr::unchecked(user_address),
            CosmosMsg::Custom(InjectiveMsg::CreateDenom {
                sender: user_address.into(),
                subdenom: denom.into(),
            }),
        );

        // and mint
        app.execute(
            Addr::unchecked(user_address),
            CosmosMsg::Custom(InjectiveMsg::Mint {
                sender: Addr::unchecked(user_address),
                amount: coin(1000, format!("tokenfactory/{}/{}", user_address, denom)),
                mint_to: user_address.into(),
            }),
        );

        println!("{:?}", app.storage());

        let balance = app.wrap().query_all_balances(user_address).unwrap();

        println!("{:?}", balance);

        // burn it
        app.execute(
            Addr::unchecked(user_address),
            CosmosMsg::Custom(InjectiveMsg::Burn {
                sender: Addr::unchecked(user_address),
                amount: coin(500, format!("tokenfactory/{}/{}", user_address, denom)),
            }),
        );

        let new_balance = app.wrap().query_all_balances(user_address).unwrap();

        println!("{:?}", new_balance);

        // try mint while non admin 
        /*
        app.execute(
            Addr::unchecked(user2_address),
            CosmosMsg::Custom(InjectiveMsg::Mint {
                sender: Addr::unchecked(user2_address),
                amount: coin(1000, format!("tokenfactory/{}/{}", user_address, denom)),
                mint_to: user_address.into(),
            }),
        );
        */
    }

    #[test]
    fn create_market() {
        let mut app = create_app();

        let base_currency = "inj";
        let quote_currency = "usdt";
        let market_id = app.create_market(base_currency.into(), quote_currency.into());

        // set midprice
        app.set_market_midprice(market_id.clone(), FPDecimal::from_str("1.5").unwrap());

        // query midprice 
        let res: MarketMidPriceAndTOBResponse = app.wrap().query(
            &cosmwasm_std::QueryRequest::Custom(InjectiveQueryWrapper {
                route: injective_cosmwasm::InjectiveRoute::Exchange,
                query_data: InjectiveQuery::SpotMarketMidPriceAndTob { market_id: MarketId::unchecked(market_id) }
            })
        ).unwrap();

        println!("{:?}", res.mid_price.unwrap().to_string());

    }
}
