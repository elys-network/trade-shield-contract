use crate::tests::{get_order_id_from_events::get_order_id_from_events, mock::multitest::ElysApp};

use super::*;

#[test]
fn successfuly_create_a_limit_buy_order() {
    let wallet = vec![("user", coins(100, "usdc"))];
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
                order_type: OrderType::LimitBuy,
                order_price: coin(2, "atom"),
                order_amm_routes: vec![],
            },
            &coins(100, "usdc"),
        )
        .unwrap();

    assert_eq!(
        app.wrap()
            .query_balance("user", "usdc")
            .unwrap()
            .amount
            .u128(),
        0
    );

    assert_eq!(
        app.wrap()
            .query_balance(&addr, "usdc")
            .unwrap()
            .amount
            .u128(),
        100
    );

    assert!(get_order_id_from_events(&resp.events).is_some());
}
