use cosmwasm_std::{OverflowError, StdError};

use crate::tests::mock::multitest::ElysApp;

use super::*;

#[test]
fn not_enough_fund() {
    let wallets = vec![("user", coins(40, "eth"))];
    let mut app = ElysApp::new_with_wallets(wallets);

    let instantiate_msg = InstantiateMockMsg {
        epoch_cycle_interval: 1,
        orders: vec![],
    };
    let create_order_msg = ExecuteMsg::CreateOrder {
        order_type: OrderType::TakeProfit,
        order_price: coin(255, "btc"),
        order_amm_routes: vec![],
    };

    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));

    let addr = app
        .instantiate_contract(
            code_id,
            Addr::unchecked("owner"),
            &instantiate_msg,
            &[],
            "Contract",
            None,
        )
        .unwrap();

    let err = app
        .execute_contract(
            Addr::unchecked("user"),
            addr.clone(),
            &create_order_msg,
            &coins(45, "eth"),
        )
        .unwrap_err();

    let error_msg: StdError = StdError::Overflow {
        source: OverflowError::new(cosmwasm_std::OverflowOperation::Sub, 40, 45),
    };

    assert_eq!(error_msg, err.downcast().unwrap());

    assert_eq!(
        app.wrap()
            .query_balance("user", "eth")
            .unwrap()
            .amount
            .u128(),
        40
    );

    assert_eq!(
        app.wrap()
            .query_balance(&addr, "eth")
            .unwrap()
            .amount
            .u128(),
        0
    );
}
