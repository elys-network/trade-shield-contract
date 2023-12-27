use super::*;
use cosmwasm_std::{Coin, Int128};

#[test]
fn successful_process_limit_buy_order() {
    let wallets: Vec<(&str, Vec<Coin>)> = vec![];
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
    let dummy_order = MarginOrder::new_close(
        "user",
        1,
        &MarginOrderType::LimitClose,
        &coin(120, "usdc"),
        "ubtc",
        &Decimal::from_str("1.1").unwrap(),
        1,
        &Some(OrderPrice {
            base_denom: "ubtc".to_string(),
            quote_denom: "usdc".to_string(),
            rate: Decimal::from_atomics(Uint128::new(38), 0).unwrap(), // Rate at which ubtc will be bought (38 USDC per ubtc).
        }),
        &Decimal::from_str("1.1").unwrap(),
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
            &[],
            "Contract",
            None,
        )
        .unwrap();

    // Set the initial ubtc and USDC prices.
    app.init_modules(|router, _, store| router.custom.set_prices(store, &prices_at_t0))
        .unwrap();

    // Execute the order processing.
    app.wasm_sudo(addr.clone(), &sudo_msg).unwrap();

    app.init_modules(|router, _, storage| {
        router.custom.set_mtp(
            storage,
            &vec![Mtp {
                address: "user".to_string(),
                liabilities: Int128::zero(),
                take_profit_liabilities: Int128::zero(),
                mtp_health: Decimal::one(),
                position: 2,
                id: 1,
                amm_pool_id: 1,
                consolidate_leverage: Decimal::zero(),
                sum_collateral: Int128::zero(),
                take_profit_price: Decimal::from_str("1.2").unwrap(),
                borrow_interest_paid_collateral: Int128::zero(),
                borrow_interest_paid_custody: Int128::zero(),
                borrow_interest_unpaid_collateral: Int128::zero(),
                collateral_asset: "btc".to_string(),
                collateral: Int128::zero(),
                custody: Int128::zero(),
                custody_asset: "btc".to_string(),
                funding_fee_paid_collateral: Int128::zero(),
                funding_fee_paid_custody: Int128::zero(),
                funding_fee_received_collateral: Int128::zero(),
                funding_fee_received_custody: Int128::zero(),
                leverage: Decimal::one(),
                liabilities_asset: "usdc".to_string(),
                open_price: Decimal::one(),
                take_profit_borrow_rate: Decimal::one(),
                take_profit_custody: Int128::zero(),
                trading_asset: "usdc".to_string(),
            }],
        )
    })
    .unwrap();

    let last_module =
        app.init_modules(|router, _, store| router.custom.get_last_module(store).unwrap());

    assert_eq!(last_module, None);

    // Update the ubtc and USDC prices to match the order rate.
    app.init_modules(|router, _, store| router.custom.set_prices(store, &prices_at_t1))
        .unwrap();

    // Execute the order processing.
    app.wasm_sudo(addr.clone(), &sudo_msg).unwrap();

    let last_module = app
        .init_modules(|router, _, store| router.custom.get_last_module(store).unwrap())
        .unwrap();

    assert_eq!(last_module, "MarginClose");
}
