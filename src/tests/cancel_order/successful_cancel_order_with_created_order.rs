use crate::tests::mock::multitest::ElysApp;

use super::*;
use get_order_id_from_events::get_order_id_from_events;

#[test]
fn successful_cancel_order_with_created_order() {
    let wallets = vec![("user", coins(150, "eth"))];
    let mut app = ElysApp::new_with_wallets(wallets);

    let instantiate_msg = InstantiateMockMsg {
        epoch_cycle_interval: 1,
        orders: vec![],
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

    let resp = app
        .execute_contract(
            Addr::unchecked("user"),
            addr.clone(),
            &ExecuteMsg::CreateOrder {
                order_type: OrderType::LimitSell,
                order_price: coin(255, "eth"),
                order_amm_routes: vec![],
            },
            &coins(45, "eth"),
        )
        .unwrap();
    let id = get_order_id_from_events(&resp.events).unwrap();

    app.execute_contract(
        Addr::unchecked("user"),
        addr.clone(),
        &ExecuteMsg::CancelOrder { order_id: id },
        &[],
    )
    .unwrap();

    assert_eq!(
        app.wrap()
            .query_balance("user", "eth")
            .unwrap()
            .amount
            .u128(),
        150
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
