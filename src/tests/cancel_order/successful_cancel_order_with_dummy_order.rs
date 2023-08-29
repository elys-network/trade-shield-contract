use crate::tests::mock::multitest::ElysApp;

use super::*;

#[test]
fn successful_cancel_order_with_dummy_order() {
    let wallets = vec![("user", coins(150, "eth")), ("owner", coins(1200, "btc"))];
    let mut app = ElysApp::new_with_wallets(wallets);
    let dummy_order = Order::new_dummy();

    let instantiate_msg = InstantiateMockMsg {
        epoch_cycle_interval: 2,
        orders: vec![dummy_order.clone()],
    };

    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));

    let addr = app
        .instantiate_contract(
            code_id,
            Addr::unchecked("owner"),
            &instantiate_msg,
            &coins(1200, "btc"),
            "Contract",
            None,
        )
        .unwrap();

    app.execute_contract(
        Addr::unchecked("user"),
        addr.clone(),
        &ExecuteMsg::CancelOrder {
            order_id: dummy_order.order_id,
        },
        &[],
    )
    .unwrap();

    assert_eq!(
        app.wrap()
            .query_balance("user", "btc")
            .unwrap()
            .amount
            .u128(),
        1000
    );
    assert_eq!(
        app.wrap()
            .query_balance(&addr, "btc")
            .unwrap()
            .amount
            .u128(),
        200
    );
}
