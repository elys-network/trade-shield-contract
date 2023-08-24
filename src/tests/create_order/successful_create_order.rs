use crate::tests::{get_order_id_from_events::get_order_id_from_events, mock::multitest::ElysApp};

use super::*;

#[test]
fn successful_create_order() {
    let wallet = vec![("user", coins(150, "eth"))];
    let mut app = ElysApp::new_with_wallets(wallet);
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
                order_type: OrderType::TakeProfit,
                order_price: coin(255, "btc"),
            },
            &coins(45, "eth"),
        )
        .unwrap();

    assert_eq!(
        app.wrap()
            .query_balance("user", "eth")
            .unwrap()
            .amount
            .u128(),
        105
    );

    assert_eq!(
        app.wrap()
            .query_balance(&addr, "eth")
            .unwrap()
            .amount
            .u128(),
        45
    );

    assert!(get_order_id_from_events(&resp.events).is_some());
}
