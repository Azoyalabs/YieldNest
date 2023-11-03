use cosmwasm_std::{to_json_binary, Binary, Deps, Env, StdResult};
use erased_serde::Serialize;
use injective_cosmwasm::{InjectiveQueryWrapper, InjectiveQuerier};

use crate::{
    msg::{GetAdminResponse, QueryMsg, SampleQueryResponse},
    state::ADMIN,
};

pub fn route_query(deps: Deps<InjectiveQueryWrapper>, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    let res: Box<dyn Serialize> = match msg {
        QueryMsg::GetAdmin {} => get_admin(deps),
        QueryMsg::SampleQuery {} => sample_query(deps),
        QueryMsg::GetCurrentAuctionBasket {  } => get_auction_basket(deps),
    };

    return Ok(to_json_binary(&res)?);
}

fn sample_query(_deps: Deps<InjectiveQueryWrapper>) -> Box<dyn Serialize> {
    return Box::new(SampleQueryResponse { value: true });
}

fn get_admin(deps: Deps<InjectiveQueryWrapper>) -> Box<dyn Serialize> {
    return Box::new(GetAdminResponse {
        admin: ADMIN.load(deps.storage).ok(),
    });
}


fn get_auction_status(deps: Deps<InjectiveQueryWrapper>) {
    let querier = InjectiveQuerier::new(&deps.querier);
    
}

fn get_auction_basket(deps: Deps<InjectiveQueryWrapper>) -> Box<dyn Serialize> {
    let querier = InjectiveQuerier::new(&deps.querier);

    return Box::new(querier.query_current_auction_basket().unwrap());
}

