use std::str::FromStr;

use super::*;

#[test]
fn coin_number() {
    // Initialize the ElysApp instance.
    let mut app = ElysApp::new();

    // Create a mock message to instantiate the contract with no initial orders.
    let instantiate_msg = InstantiateMockMsg {
        spot_orders: vec![],
        margin_orders: vec![],
    };
    // Create a contract wrapper and store its code.
    let code = ContractWrapper::new(execute, instantiate, query);
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

    // User "user" attempts to create an order without specifying the amount.

    let err = app
        .execute_contract(
            Addr::unchecked("user"),
            addr,
            &ExecuteMsg::CreateMarginOrder {
                position: MarginPosition::Short,
                collateral: coin(600, "uusdc"),
                leverage: Decimal::from_atomics(Uint128::new(500), 2).unwrap(),
                borrow_asset: "uatom".to_string(),
                take_profit_price: Decimal::from_atomics(Uint128::new(500), 2).unwrap(),
                order_type: OrderType::LimitSell,
                trigger_price: Some(OrderPrice {
                    base_denom: "uatom".to_string(),
                    quote_denom: "uusdc".to_string(),
                    rate: Decimal::from_str("1.5").unwrap(),
                }),
            },
            &[],
        )
        .unwrap_err();

    // Verify that the error is of type "CoinNumber."
    assert_eq!(ContractError::CoinNumber, err.downcast().unwrap());
}
