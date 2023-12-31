mod common;

#[cfg(test)]
mod tests {
    use cosmwasm_std::{coin, Addr};
    use cw_multi_test::{App, AppBuilder, Executor, WasmKeeper};
    use yield_nest::msg::InstantiateMsg;

    use crate::common::injective_mock_modules::create_app;
    //use robot_code_gen::Robot;

    //use crate::common::cosmwasm_contract_template_robot::CosmwasmContractTemplateRobot;
    //use crate::common::test_utils::get_contract;

    const TEST_ADMIN: &'static str = "ADMIN";
    const CONTRACT_LABEL: &'static str = "CONTRACT_LABEL";

    #[test]
    fn try_access_bank() {
        let mut app = create_app();

        let user_address = "user";
        let denom = "test_dollar";

        app.sudo(cw_multi_test::SudoMsg::Bank(
            cw_multi_test::BankSudo::Mint {
                to_address: user_address.into(),
                amount: vec![coin(1000, denom)],
            },
        ));
    }
}
