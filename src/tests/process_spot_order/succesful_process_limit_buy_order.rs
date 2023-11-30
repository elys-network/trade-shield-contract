use super::*;
use cosmwasm_std::{coins, BlockInfo, Coin, Timestamp};

// This test case verifies the successful processing of a "limit buy" order in the contract.
// The scenario involves a "limit buy" order created by a user to buy ubtc at a specific price.
// - Initially, the ubtc price is 70 USDC, and the order rate is set at 38 USDC per ubtc.
// - The order is created with 120 USDC to be used for buying ubtc at the specified rate.
// - After processing the order, the ubtc price decreases to 40 USDC, matching the order rate.
// - The order is executed, and the user receives 3 ubtc.
#[test]
fn successful_process_limit_buy_order() {
    // Initialize wallets for "owner" and "user."
    let wallets: Vec<(&str, Vec<Coin>)> = vec![("owner", coins(120, "usdc")), ("user", vec![])];
    let mut app = ElysApp::new_with_wallets(wallets);

    // Define ubtc and USDC prices at two different time intervals (t0 and t1).
    let prices_at_t0 = vec![
        Price::new("ubtc", Decimal::from_atomics(Uint128::new(70), 0).unwrap()),
        Price::new("usdc", Decimal::from_atomics(Uint128::new(1), 0).unwrap()),
    ];
    let prices_at_t1 = vec![
        Price::new("ubtc", Decimal::from_atomics(Uint128::new(40), 0).unwrap()),
        Price::new("usdc", Decimal::from_atomics(Uint128::new(1), 0).unwrap()),
    ];

    // Create a contract wrapper and store its code.
    let code = ContractWrapper::new(execute, instantiate, query)
        .with_reply(reply)
        .with_sudo(sudo);
    let code_id = app.store_code(Box::new(code));

    // Create a "limit buy" order (dummy order) with a specific rate and balance.
    let dummy_order = SpotOrder::new(
        SpotOrderType::LimitBuy,
        Some(OrderPrice {
            base_denom: "ubtc".to_string(),
            quote_denom: "usdc".to_string(),
            rate: Decimal::from_atomics(Uint128::new(38), 0).unwrap(), // Rate at which ubtc will be bought (38 USDC per ubtc).
        }),
        coin(120, "usdc"), // 120 USDC to be used for buying.
        Addr::unchecked("user"),
        "ubtc".to_string(),
        &vec![],
        &BlockInfo {
            height: 50,
            time: Timestamp::from_seconds(600),
            chain_id: "elys-app".to_string(),
        },
    );

    // Create a mock message to instantiate the contract with the dummy order.
    let instantiate_msg = InstantiateMockMsg {
        spot_orders: vec![dummy_order],
        margin_orders: vec![],
    };

    // Create an sudo message to process orders.
    let sudo_msg = SudoMsg::ClockEndBlock {};

    // Instantiate the contract with "owner" as the deployer and deposit 120 USDC.
    let addr = app
        .instantiate_contract(
            code_id,
            Addr::unchecked("owner"),
            &instantiate_msg,
            &coins(120, "usdc"),
            "Contract",
            None,
        )
        .unwrap();

    // Set the initial ubtc and USDC prices.
    app.init_modules(|router, _, store| router.custom.set_prices(store, &prices_at_t0))
        .unwrap();

    // Execute the order processing.
    // Execute the order processing.
    app.wasm_sudo(addr.clone(), &sudo_msg).unwrap();

    // Verify the resulting balances after order processing.
    assert_eq!(
        app.wrap()
            .query_balance(&addr, "usdc")
            .unwrap()
            .amount
            .u128(),
        120
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

    // Update the ubtc and USDC prices to match the order rate.
    app.init_modules(|router, _, store| router.custom.set_prices(store, &prices_at_t1))
        .unwrap();

    // Execute the order processing.
    // Execute the order processing.
    app.wasm_sudo(addr.clone(), &sudo_msg).unwrap();

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
        3
    );
    assert_eq!(
        app.wrap()
            .query_balance("user", "usdc")
            .unwrap()
            .amount
            .u128(),
        0
    );
}
