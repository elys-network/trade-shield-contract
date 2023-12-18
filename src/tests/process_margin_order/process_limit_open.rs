use super::*;
use cosmwasm_std::{coins, Coin};

#[test]
fn successful_process_limit_buy_order() {
    // Initialize wallets for "owner"
    let wallets: Vec<(&str, Vec<Coin>)> = vec![("owner", coins(120, "usdc"))];
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

    // Create a "limit open" order (dummy order) with a specific rate and balance.
    let dummy_order = MarginOrder::new_open(
        "user",
        &MarginPosition::Short,
        &MarginOrderType::LimitOpen,
        &coin(120, "usdc"),
        "ubtc",
        &Decimal::from_str("1.1").unwrap(),
        &Decimal::from_str("1.1").unwrap(),
        &Some(OrderPrice {
            base_denom: "ubtc".to_string(),
            quote_denom: "usdc".to_string(),
            rate: Decimal::from_atomics(Uint128::new(38), 0).unwrap(), // Rate at which ubtc will be bought (38 USDC per ubtc).
        }),
        &vec![],
    )
    .unwrap();

    // Create a mock message to instantiate the contract with the dummy order.
    let instantiate_msg = InstantiateMockMsg {
        spot_orders: vec![],
        margin_orders: vec![dummy_order],
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

    let last_module =
        app.init_modules(|router, _, store| router.custom.get_last_module(store).unwrap());

    assert_eq!(last_module, None);

    // Update the ubtc and USDC prices to match the order rate.
    app.init_modules(|router, _, store| router.custom.set_prices(store, &prices_at_t1))
        .unwrap();

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
    let last_module = app
        .init_modules(|router, _, store| router.custom.get_last_module(store).unwrap())
        .unwrap();

    assert_eq!(last_module, "MarginOpen");
}
