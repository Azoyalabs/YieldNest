use cosmwasm_std::{to_json_binary, Addr, Binary, Deps, Env, StdResult};
use erased_serde::Serialize;
use injective_cosmwasm::InjectiveQueryWrapper;

use crate::{
    msg::{GetAdminResponse, GetUserMintPositionsResponse, QueryMsg},
    state::{ADMIN, MINT_POSITIONS, USER_MINT_POSITIONS},
};

pub fn route_query(
    deps: Deps<InjectiveQueryWrapper>,
    _env: Env,
    msg: QueryMsg,
) -> StdResult<Binary> {
    let res: Box<dyn Serialize> = match msg {
        QueryMsg::GetAdmin {} => get_admin(deps),
        QueryMsg::GetUserMintPositions { user_address } => get_user_mint_positions(deps, user_address),
    };

    return Ok(to_json_binary(&res)?);
}


fn get_admin(deps: Deps<InjectiveQueryWrapper>) -> Box<dyn Serialize> {
    return Box::new(GetAdminResponse {
        admin: ADMIN.load(deps.storage).ok(),
    });
}

/*
fn get_auction_basket(deps: Deps<InjectiveQueryWrapper>) -> Box<dyn Serialize> {
    let querier = InjectiveQuerier::new(&deps.querier);

    return Box::new(querier.query_current_auction_basket().unwrap());
}
*/

fn get_user_mint_positions(
    deps: Deps<InjectiveQueryWrapper>,
    user_address: Addr,
) -> Box<dyn Serialize> {
    let id_positions = match USER_MINT_POSITIONS.load(deps.storage, user_address) {
        Ok(pos) => pos,
        Err(_) => vec![],
    };

    let positions = id_positions
        .iter()
        .map(|id_pos| MINT_POSITIONS.load(deps.storage, *id_pos).unwrap().1)
        .collect();

    return Box::new(GetUserMintPositionsResponse {
        mint_positions: positions,
    });
}
