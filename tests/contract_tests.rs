mod common;

#[cfg(test)]
mod tests {
    use cosmwasm_std::Addr;
    use cw_multi_test::{App, AppBuilder, Executor, WasmKeeper};
    use lend_protocol::msg::InstantiateMsg;

    use crate::common::injective_mock_modules::create_app;
    //use robot_code_gen::Robot;

    //use crate::common::cosmwasm_contract_template_robot::CosmwasmContractTemplateRobot;
    //use crate::common::test_utils::get_contract;

    const TEST_ADMIN: &'static str = "ADMIN";
    const CONTRACT_LABEL: &'static str = "CONTRACT_LABEL";

    #[test]
    fn successful_deployment() {
        let mut router = App::default();
        let mut router = create_app();
        /*
        let contract_code_id = router.store_code(get_contract());

        let instantiate_msg = InstantiateMsg {};

        let admin = Addr::unchecked(TEST_ADMIN);
        let instantiate_res = router.instantiate_contract(
            contract_code_id,
            admin,
            &instantiate_msg,
            &[],
            String::from(CONTRACT_LABEL),
            Some(TEST_ADMIN.to_owned()),
        );

        match instantiate_res {
            Ok(_contract_address) => (),
            Err(err) => panic!("Failed to instantiate contract: {}", err.to_string()),
        }
        */
    }

    #[test]
    fn compiling_with_wasm_keeper_should_work() {
        // this verifies only compilation errors
        // while our WasmKeeper does not implement Module
        let app_builder = AppBuilder::default();
        let _ = app_builder
            .with_wasm(WasmKeeper::default())
            .build(|_, _, _| {});
    }
}
