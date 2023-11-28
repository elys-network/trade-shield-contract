use crate::tests::read_processed_order_id::read_processed_order_id;

use super::*;
use cosmwasm_std::Coin;

#[test]
fn successful_process_5_of_10_orders() {
    let wallets: Vec<(&str, Vec<Coin>)> = vec![
        ("owner", vec![coin(11, "btc"), coin(9, "eth")]),
        ("user", vec![]),
    ];
    let mut app = ElysApp::new_with_wallets(wallets);

    let prices_at_t0 = vec![
        Price::new(
            "btc",
            Decimal::from_atomics(Uint128::new(20000), 0).unwrap(),
        ),
        Price::new("usdc", Decimal::from_atomics(Uint128::new(1), 0).unwrap()),
        Price::new("eth", Decimal::from_atomics(Uint128::new(2000), 0).unwrap()),
    ];
    let prices_at_t1 = vec![
        Price::new(
            "btc",
            Decimal::from_atomics(Uint128::new(30000), 0).unwrap(),
        ),
        Price::new("usdc", Decimal::from_atomics(Uint128::new(1), 0).unwrap()),
        Price::new("eth", Decimal::from_atomics(Uint128::new(1700), 0).unwrap()),
    ];

    let code = ContractWrapper::new(execute, instantiate, query)
        .with_reply(reply)
        .with_sudo(sudo);
    let code_id = app.store_code(Box::new(code));

    let spot_orders = create_dummy_orders();

    let instantiate_msg = InstantiateMockMsg {
        spot_orders,
        margin_orders: vec![],
    };

    let sudo_msg = SudoMsg::ClockEndBlock {};

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

    // Execute the order processing.
    let resp = app.wasm_sudo(addr.clone(), &sudo_msg).unwrap();

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

    // Execute the order processing.
    let resp = app.wasm_sudo(addr.clone(), &sudo_msg).unwrap();

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

    let order_ids: Vec<u64> = read_processed_order_id(resp);

    assert!(order_ids.is_empty());

    // Execute the order processing.
    app.wasm_sudo(addr.clone(), &sudo_msg).unwrap();

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
            order_type: OrderType::StopLoss,
            order_id: 0,
            order_target_denom: "usdc".to_string(),
            order_amount: coin(1, "eth"),
            owner_address: Addr::unchecked("user"),
            order_price: OrderPrice {
                base_denom: "eth".to_string(),
                quote_denom: "usdc".to_string(),
                rate: Decimal::from_atomics(Uint128::new(1700), 0).unwrap(),
            },
            status: Status::NotProcessed,
        },
        SpotOrder {
            order_type: OrderType::StopLoss,
            order_id: 1,
            order_amount: coin(2, "btc"),
            owner_address: Addr::unchecked("user"),
            order_price: OrderPrice {
                base_denom: "btc".to_string(),
                quote_denom: "usdc".to_string(),
                rate: Decimal::from_atomics(Uint128::new(12000), 0).unwrap(),
            },
            order_target_denom: "usdc".to_string(),
            status: Status::NotProcessed,
        },
        SpotOrder {
            order_type: OrderType::StopLoss,
            order_id: 2,
            order_amount: coin(3, "btc"),
            owner_address: Addr::unchecked("user"),
            order_price: OrderPrice {
                base_denom: "btc".to_string(),
                quote_denom: "usdc".to_string(),
                rate: Decimal::from_atomics(Uint128::new(10000), 0).unwrap(),
            },
            order_target_denom: "usdc".to_string(),
            status: Status::NotProcessed,
        },
        SpotOrder {
            order_type: OrderType::StopLoss,
            order_id: 3,
            order_amount: coin(5, "eth"),
            owner_address: Addr::unchecked("user"),
            order_price: OrderPrice {
                base_denom: "eth".to_string(),
                quote_denom: "usdc".to_string(),
                rate: Decimal::from_atomics(Uint128::new(1800), 0).unwrap(),
            },
            order_target_denom: "usdc".to_string(),
            status: Status::NotProcessed,
        },
        SpotOrder {
            order_type: OrderType::StopLoss,
            order_id: 4,
            order_amount: coin(1, "eth"),
            owner_address: Addr::unchecked("user"),
            order_price: OrderPrice {
                base_denom: "eth".to_string(),
                quote_denom: "usdc".to_string(),
                rate: Decimal::from_atomics(Uint128::new(1200), 0).unwrap(),
            },
            order_target_denom: "usdc".to_string(),
            status: Status::NotProcessed,
        },
        SpotOrder {
            order_type: OrderType::LimitSell,
            order_id: 5,
            order_amount: coin(1, "eth"),
            owner_address: Addr::unchecked("user"),
            order_price: OrderPrice {
                base_denom: "eth".to_string(),
                quote_denom: "usdc".to_string(),
                rate: Decimal::from_atomics(Uint128::new(2500), 0).unwrap(),
            },
            order_target_denom: "usdc".to_string(),
            status: Status::NotProcessed,
        },
        SpotOrder {
            order_type: OrderType::LimitSell,
            order_id: 6,
            order_amount: coin(3, "btc"),
            owner_address: Addr::unchecked("user"),
            order_price: OrderPrice {
                base_denom: "btc".to_string(),
                quote_denom: "usdc".to_string(),
                rate: Decimal::from_atomics(Uint128::new(21000), 0).unwrap(),
            },
            order_target_denom: "usdc".to_string(),
            status: Status::NotProcessed,
        },
        SpotOrder {
            order_type: OrderType::LimitSell,
            order_id: 7,
            order_amount: coin(2, "btc"),
            owner_address: Addr::unchecked("user"),
            order_price: OrderPrice {
                base_denom: "btc".to_string(),
                quote_denom: "usdc".to_string(),
                rate: Decimal::from_atomics(Uint128::new(25000), 0).unwrap(),
            },
            order_target_denom: "usdc".to_string(),
            status: Status::NotProcessed,
        },
        SpotOrder {
            order_type: OrderType::LimitSell,
            order_id: 8,
            order_amount: coin(1, "btc"),
            owner_address: Addr::unchecked("user"),
            order_price: OrderPrice {
                base_denom: "btc".to_string(),
                quote_denom: "usdc".to_string(),
                rate: Decimal::from_atomics(Uint128::new(30000), 0).unwrap(),
            },
            order_target_denom: "usdc".to_string(),
            status: Status::NotProcessed,
        },
        SpotOrder {
            order_type: OrderType::LimitSell,
            order_id: 9,
            order_amount: coin(1, "eth"),
            owner_address: Addr::unchecked("user"),
            order_price: OrderPrice {
                base_denom: "eth".to_string(),
                quote_denom: "usdc".to_string(),
                rate: Decimal::from_atomics(Uint128::new(2100), 0).unwrap(),
            },
            order_target_denom: "usdc".to_string(),
            status: Status::NotProcessed,
        },
    ]
}
