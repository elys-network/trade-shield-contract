use crate::tests::get_order_id_from_events::get_order_id_from_events;

use super::*;

#[test]
fn successful_create_long_order() {
    // Create a wallet for the "user" with an initial balance of 10 BTC.
    let wallet = vec![("user", coins(10, "btc"))];

    // Initialize the ElysApp instance with the specified wallet.
    let mut app = ElysApp::new_with_wallets(wallet);

    // Create a mock message to instantiate the contract with no initial orders.
    let instantiate_msg = InstantiateMockMsg {
        process_order_executor: "owner".to_string(),
        orders: vec![],
    };

    // Create a contract wrapper and store its code.
    let code = ContractWrapper::new(execute, instantiate, query).with_reply(reply);
    let code_id = app.store_code(Box::new(code));

    // Instantiate the contract with "owner" as the deployer.
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

    // User "user" creates a "long position" margin order for BTC, specifying the maximum price in BTC.
    let resp = app
        .execute_contract(
            Addr::unchecked("user"),
            addr.clone(),
            &ExecuteMsg::CreateMarginOrder {
                position: MarginPosition::Long,
                collateral: coin(10, "btc"),
                leverage: Decimal::from_atomics(Uint128::new(215), 2).unwrap(),
                borrow_asset: "btc".to_string(),
                take_profit_price: Decimal::from_atomics(Uint128::new(200), 2).unwrap(),
            },
            &coins(10, "btc"), // User's BTC balance.
        )
        .unwrap();

    // Verify that the "user" no longer has any BTC after creating the order.
    assert_eq!(
        app.wrap()
            .query_balance("user", "btc")
            .unwrap()
            .amount
            .u128(),
        0
    );

    // Verify that the contract address sends the BTC to the margin module.
    assert_eq!(
        app.wrap()
            .query_balance(&addr, "btc")
            .unwrap()
            .amount
            .u128(),
        0
    );

    // Verify that an order ID is emitted in the contract's events.
    assert!(get_order_id_from_events(&resp.events).is_some());
}