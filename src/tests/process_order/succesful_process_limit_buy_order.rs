use crate::{tests::mock::multitest::ElysApp, types::SwapAmountInRoute};

use super::*;
use cosmwasm_std::{coins, Coin};

#[test]
fn successful_process_limit_buy_order() {
    let wallets: Vec<(&str, Vec<Coin>)> = vec![("owner", coins(100, "usdc")), ("user", vec![])];
    let mut app = ElysApp::new_with_wallets(wallets);

    let prices_at_t0 = vec![coin(40, "ubtc"), coin(1, "usdc")];
    let prices_at_t1 = vec![coin(38, "ubtc"), coin(1, "usdc")];

    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));

    let dummy_order = Order::new(
        OrderType::StopLoss,
        OrderPrice {
            base_denom: "usdc".to_string(),
            quote_denom: "ubtc".to_string(),
            rate: Uint128::new(38),
        },
        coin(100, "usdc"),
        Addr::unchecked("user"),
        "ubtc".to_string(),
        vec![SwapAmountInRoute::new(1, "ubtc")],
        &vec![],
    );

    let instantiate_msg = InstantiateMockMsg {
        process_order_executor: "owner".to_string(),
        orders: vec![dummy_order],
    };
    let execute_msg = ExecuteMsg::ProcessOrders {};

    let addr = app
        .instantiate_contract(
            code_id,
            Addr::unchecked("owner"),
            &instantiate_msg,
            &coins(100, "usdc"),
            "Contract",
            None,
        )
        .unwrap();

    app.init_modules(|router, _, store| router.custom.set_prices(store, &prices_at_t0))
        .unwrap();

    let resp = app
        .execute_contract(addr.clone(), addr.clone(), &execute_msg, &[])
        .unwrap();

    assert_eq!(
        app.wrap()
            .query_balance(&addr, "usdc")
            .unwrap()
            .amount
            .u128(),
        100
    );
    assert_eq!(
        app.wrap()
            .query_balance(&addr, "ubtc")
            .unwrap()
            .amount
            .u128(),
        0
    );
    assert_eq!(
        app.wrap()
            .query_balance("user", "ubtc")
            .unwrap()
            .amount
            .u128(),
        0
    );
    assert_eq!(
        app.wrap()
            .query_balance("user", "usdc")
            .unwrap()
            .amount
            .u128(),
        0
    );

    let order_id: Option<u128> = resp.events.iter().find_map(|e| {
        e.attributes
            .iter()
            .find(|attr| {
                attr.key == "order_id"
                    && attr.value == instantiate_msg.orders[0].order_id.to_string()
            })
            .and_then(|attr| attr.value.parse::<u128>().ok())
    });

    assert!(order_id.is_none());

    app.init_modules(|router, _, store| router.custom.set_prices(store, &prices_at_t1))
        .unwrap();

    let resp = app
        .execute_contract(addr.clone(), addr.clone(), &execute_msg, &[])
        .unwrap();

    assert_eq!(
        app.wrap()
            .query_balance(&addr, "ubtc")
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
        0
    );
    assert_eq!(
        app.wrap()
            .query_balance("user", "ubtc")
            .unwrap()
            .amount
            .u128(),
        0
    );
    assert_eq!(
        app.wrap()
            .query_balance("user", "usdc")
            .unwrap()
            .amount
            .u128(),
        3800
    );

    let order_id: Option<u128> = resp.events.iter().find_map(|e| {
        e.attributes
            .iter()
            .find(|attr| {
                attr.key == "order_id"
                    && attr.value == instantiate_msg.orders[0].order_id.to_string()
            })
            .and_then(|attr| attr.value.parse::<u128>().ok())
    });

    assert!(order_id.is_some());
}
