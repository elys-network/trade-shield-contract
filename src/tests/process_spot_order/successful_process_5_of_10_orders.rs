use crate::{tests::read_processed_order_id::read_processed_order_id, types::SwapAmountInRoute};

use super::*;
use cosmwasm_std::Coin;

#[test]
fn successful_process_5_of_10_orders() {
    let wallets: Vec<(&str, Vec<Coin>)> = vec![
        ("owner", vec![coin(11, "btc"), coin(9, "eth")]),
        ("user", vec![]),
    ];
    let mut app = ElysApp::new_with_wallets(wallets);

    let prices_at_t0 = vec![coin(20000, "btc"), coin(1, "usdc"), coin(2000, "eth")];
    let prices_at_t1 = vec![coin(30000, "btc"), coin(1, "usdc"), coin(1700, "eth")];

    let code = ContractWrapper::new(execute, instantiate, query).with_reply(reply);
    let code_id = app.store_code(Box::new(code));

    let orders = create_dummy_orders();

    let instantiate_msg = InstantiateMockMsg {
        process_order_executor: "owner".to_string(),
        orders,
    };
    let execute_msg = ExecuteMsg::ProcessSpotOrders {};

    let addr = app
        .instantiate_contract(
            code_id,
            Addr::unchecked("owner"),
            &instantiate_msg,
            &vec![coin(11, "btc"), coin(9, "eth")],
            "Contract",
            None,
        )
        .unwrap();

    app.init_modules(|router, _, store| router.custom.set_prices(store, &prices_at_t0))
        .unwrap();

    let resp = app
        .execute_contract(Addr::unchecked("owner"), addr.clone(), &execute_msg, &[])
        .unwrap();

    assert_eq!(
        app.wrap()
            .query_balance(&addr, "btc")
            .unwrap()
            .amount
            .u128(),
        11
    );
    assert_eq!(
        app.wrap()
            .query_balance(&addr, "eth")
            .unwrap()
            .amount
            .u128(),
        9
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
            .query_balance("user", "btc")
            .unwrap()
            .amount
            .u128(),
        0
    );
    assert_eq!(
        app.wrap()
            .query_balance("user", "eth")
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

    let order_ids: Vec<u64> = read_processed_order_id(resp);

    assert!(order_ids.is_empty());

    app.init_modules(|router, _, store| router.custom.set_prices(store, &prices_at_t1))
        .unwrap();

    let resp = app
        .execute_contract(Addr::unchecked("owner"), addr.clone(), &execute_msg, &[])
        .unwrap();

    assert_eq!(
        app.wrap()
            .query_balance(&addr, "btc")
            .unwrap()
            .amount
            .u128(),
        5
    );
    assert_eq!(
        app.wrap()
            .query_balance(&addr, "eth")
            .unwrap()
            .amount
            .u128(),
        3
    );
    assert_eq!(
        app.wrap()
            .query_balance(&addr, "usdc")
            .unwrap()
            .amount
            .u128(),
        190200
    );

    assert_eq!(
        app.wrap()
            .query_balance("user", "btc")
            .unwrap()
            .amount
            .u128(),
        0
    );
    assert_eq!(
        app.wrap()
            .query_balance("user", "eth")
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

    let order_ids: Vec<u64> = read_processed_order_id(resp);

    assert!(order_ids.is_empty());

    let resp = app
        .execute_contract(Addr::unchecked("owner"), addr.clone(), &execute_msg, &[])
        .unwrap();

    let order_ids: Vec<u64> = read_processed_order_id(resp);

    assert!(order_ids.contains(&instantiate_msg.orders[7].order_id));
    assert!(order_ids.contains(&instantiate_msg.orders[3].order_id));
    assert!(order_ids.contains(&instantiate_msg.orders[0].order_id));
    assert!(order_ids.contains(&instantiate_msg.orders[6].order_id));
    assert!(order_ids.contains(&instantiate_msg.orders[8].order_id));

    assert_eq!(
        app.wrap()
            .query_balance(&addr, "btc")
            .unwrap()
            .amount
            .u128(),
        5
    );
    assert_eq!(
        app.wrap()
            .query_balance(&addr, "eth")
            .unwrap()
            .amount
            .u128(),
        3
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
            .query_balance("user", "btc")
            .unwrap()
            .amount
            .u128(),
        0
    );
    assert_eq!(
        app.wrap()
            .query_balance("user", "eth")
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
        190200
    );
}

fn create_dummy_orders() -> Vec<SpotOrder> {
    vec![
        SpotOrder {
            order_type: SpotOrderType::StopLoss,
            order_id: 0,
            order_target_denom: "usdc".to_string(),
            order_amount: coin(1, "eth"),
            owner_address: Addr::unchecked("user"),
            order_amm_routes: vec![SwapAmountInRoute::new(1, "usdc")],
            order_price: SpotOrderPrice {
                base_denom: "eth".to_string(),
                quote_denom: "usdc".to_string(),
                rate: Decimal::from_atomics(Uint128::new(1700), 0).unwrap(),
            },
        },
        SpotOrder {
            order_type: SpotOrderType::StopLoss,
            order_id: 1,
            order_amount: coin(2, "btc"),
            owner_address: Addr::unchecked("user"),
            order_amm_routes: vec![SwapAmountInRoute::new(1, "usdc")],
            order_price: SpotOrderPrice {
                base_denom: "btc".to_string(),
                quote_denom: "usdc".to_string(),
                rate: Decimal::from_atomics(Uint128::new(12000), 0).unwrap(),
            },
            order_target_denom: "usdc".to_string(),
        },
        SpotOrder {
            order_type: SpotOrderType::StopLoss,
            order_id: 2,
            order_amount: coin(3, "btc"),
            owner_address: Addr::unchecked("user"),
            order_amm_routes: vec![SwapAmountInRoute::new(1, "usdc")],
            order_price: SpotOrderPrice {
                base_denom: "btc".to_string(),
                quote_denom: "usdc".to_string(),
                rate: Decimal::from_atomics(Uint128::new(10000), 0).unwrap(),
            },
            order_target_denom: "usdc".to_string(),
        },
        SpotOrder {
            order_type: SpotOrderType::StopLoss,
            order_id: 3,
            order_amount: coin(5, "eth"),
            owner_address: Addr::unchecked("user"),
            order_amm_routes: vec![SwapAmountInRoute::new(1, "usdc")],
            order_price: SpotOrderPrice {
                base_denom: "eth".to_string(),
                quote_denom: "usdc".to_string(),
                rate: Decimal::from_atomics(Uint128::new(1800), 0).unwrap(),
            },
            order_target_denom: "usdc".to_string(),
        },
        SpotOrder {
            order_type: SpotOrderType::StopLoss,
            order_id: 4,
            order_amount: coin(1, "eth"),
            owner_address: Addr::unchecked("user"),
            order_amm_routes: vec![SwapAmountInRoute::new(1, "usdc")],
            order_price: SpotOrderPrice {
                base_denom: "eth".to_string(),
                quote_denom: "usdc".to_string(),
                rate: Decimal::from_atomics(Uint128::new(1200), 0).unwrap(),
            },
            order_target_denom: "usdc".to_string(),
        },
        SpotOrder {
            order_type: SpotOrderType::LimitSell,
            order_id: 5,
            order_amount: coin(1, "eth"),
            owner_address: Addr::unchecked("user"),
            order_amm_routes: vec![SwapAmountInRoute::new(1, "usdc")],
            order_price: SpotOrderPrice {
                base_denom: "eth".to_string(),
                quote_denom: "usdc".to_string(),
                rate: Decimal::from_atomics(Uint128::new(2500), 0).unwrap(),
            },
            order_target_denom: "usdc".to_string(),
        },
        SpotOrder {
            order_type: SpotOrderType::LimitSell,
            order_id: 6,
            order_amount: coin(3, "btc"),
            owner_address: Addr::unchecked("user"),
            order_amm_routes: vec![SwapAmountInRoute::new(1, "usdc")],
            order_price: SpotOrderPrice {
                base_denom: "btc".to_string(),
                quote_denom: "usdc".to_string(),
                rate: Decimal::from_atomics(Uint128::new(21000), 0).unwrap(),
            },
            order_target_denom: "usdc".to_string(),
        },
        SpotOrder {
            order_type: SpotOrderType::LimitSell,
            order_id: 7,
            order_amount: coin(2, "btc"),
            owner_address: Addr::unchecked("user"),
            order_amm_routes: vec![SwapAmountInRoute::new(1, "usdc")],
            order_price: SpotOrderPrice {
                base_denom: "btc".to_string(),
                quote_denom: "usdc".to_string(),
                rate: Decimal::from_atomics(Uint128::new(25000), 0).unwrap(),
            },
            order_target_denom: "usdc".to_string(),
        },
        SpotOrder {
            order_type: SpotOrderType::LimitSell,
            order_id: 8,
            order_amount: coin(1, "btc"),
            owner_address: Addr::unchecked("user"),
            order_amm_routes: vec![SwapAmountInRoute::new(1, "usdc")],
            order_price: SpotOrderPrice {
                base_denom: "btc".to_string(),
                quote_denom: "usdc".to_string(),
                rate: Decimal::from_atomics(Uint128::new(30000), 0).unwrap(),
            },
            order_target_denom: "usdc".to_string(),
        },
        SpotOrder {
            order_type: SpotOrderType::LimitSell,
            order_id: 9,
            order_amount: coin(1, "eth"),
            owner_address: Addr::unchecked("user"),
            order_amm_routes: vec![SwapAmountInRoute::new(1, "usdc")],
            order_price: SpotOrderPrice {
                base_denom: "eth".to_string(),
                quote_denom: "usdc".to_string(),
                rate: Decimal::from_atomics(Uint128::new(2100), 0).unwrap(),
            },
            order_target_denom: "usdc".to_string(),
        },
    ]
}
