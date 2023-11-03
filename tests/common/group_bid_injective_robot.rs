use robot_code_gen::Robot;
use cosmwasm_std::{Addr, Coin};
use cw_multi_test::{App, Executor};
use lend_protocol::msg::*;

pub trait GroupBidInjectiveRobot {
	fn add_capital(&mut self, app: &mut App, contract: &Addr, caller: &Addr,  funds: Vec<Coin>);
	fn bid(&mut self, app: &mut App, contract: &Addr, caller: &Addr,  funds: Vec<Coin>);
	fn withdraw(&mut self, app: &mut App, contract: &Addr, caller: &Addr,  funds: Vec<Coin>);
	fn get_admin(&self, app: &App, contract: &Addr, ) -> GetAdminResponse;
	fn sample_query(&self, app: &App, contract: &Addr, ) -> SampleQueryResponse;
	fn get_current_auction_basket(&self, app: &App, contract: &Addr, ) -> GetCurrentAuctionBasketResponse;
}

impl GroupBidInjectiveRobot for Robot {
	fn add_capital(&mut self, app: &mut App, contract: &Addr, caller: &Addr,  funds: Vec<Coin>){
		let msg = ExecuteMsg::AddCapital {};
		app.execute_contract(caller.to_owned(), contract.to_owned(), &msg, &funds).unwrap();
	}
	fn bid(&mut self, app: &mut App, contract: &Addr, caller: &Addr,  funds: Vec<Coin>){
		let msg = ExecuteMsg::Bid {};
		app.execute_contract(caller.to_owned(), contract.to_owned(), &msg, &funds).unwrap();
	}
	fn withdraw(&mut self, app: &mut App, contract: &Addr, caller: &Addr,  funds: Vec<Coin>){
		let msg = ExecuteMsg::Withdraw {};
		app.execute_contract(caller.to_owned(), contract.to_owned(), &msg, &funds).unwrap();
	}
	fn get_admin(&self, app: &App, contract: &Addr, ) -> GetAdminResponse{
		let msg = QueryMsg::GetAdmin {};
		return app.wrap().query_wasm_smart(contract.to_owned(), &msg).unwrap();
	}
	fn sample_query(&self, app: &App, contract: &Addr, ) -> SampleQueryResponse{
		let msg = QueryMsg::SampleQuery {};
		return app.wrap().query_wasm_smart(contract.to_owned(), &msg).unwrap();
	}
	fn get_current_auction_basket(&self, app: &App, contract: &Addr, ) -> GetCurrentAuctionBasketResponse{
		let msg = QueryMsg::GetCurrentAuctionBasket {};
		return app.wrap().query_wasm_smart(contract.to_owned(), &msg).unwrap();
	}
}