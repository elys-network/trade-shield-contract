use super::*;
use cosmwasm_std::{coins, Coin};

#[test]
fn process_spot_order_processing() {
    // Initialize wallets for "owner" and "user."
    let wallets: Vec<(&str, Vec<Coin>)> = vec![("owner", coins(120, "usdc")), ("user", vec![])];
    let mut app = ElysApp::new_with_wallets(wallets);

    // Create a contract wrapper and store its code.
    let code = ContractWrapper::new(execute, instantiate, query)
        .with_reply(reply)
        .with_sudo(sudo);
    let code_id = app.store_code(Box::new(code));

    // Create a "limit buy" order (dummy order) with a specific rate and balance.
    let dummy_order = SpotOrder {
        order_type: SpotOrderType::LimitBuy,
        order_id: 0,
        order_price: OrderPrice {
            base_denom: "ubtc".to_string(),
            quote_denom: "usdc".to_string(),
            rate: Decimal::from_atomics(Uint128::new(38), 0).unwrap(), // Rate at which ubtc will be bought (38 USDC per ubtc).
        },
        order_amount: coin(120, "usdc"), // 120 USDC to be used for buying,
        owner_address: Addr::unchecked("user"),
        order_target_denom: "ubtc".to_string(),
        status: Status::Processed,
    };

    // Create a mock message to instantiate the contract with the dummy order.
    let instantiate_msg = InstantiateMockMsg {
        spot_orders: vec![dummy_order],
        margin_orders: vec![],
    };

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

    // Verify the swap occurred.

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

    // Try to cancel an order that's processing
    let err = app
        .execute_contract(
            Addr::unchecked("user"),
            addr,
            &ExecuteMsg::CancelSpotOrder { order_id: 0 },
            &vec![],
        )
        .unwrap_err();

    assert_eq!(
        ContractError::CancelStatusError {
            order_id: 0,
            status: Status::Processed
        },
        err.downcast().unwrap()
    );
}
