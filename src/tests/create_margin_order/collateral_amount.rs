use std::str::FromStr;

use super::*;

#[test]
fn collateral_amount() {
    let wallets = vec![("user", coins(45, "usdc"))];

    // Initialize the ElysApp instance.
    let mut app = ElysApp::new_with_wallets(wallets);

    // Create a mock message to instantiate the contract with no initial orders.
    let instantiate_msg = InstantiateMockMsg {
        process_order_executor: "owner".to_string(),
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

    // User "user" attempts to create an order with the wrong amount of collateral asset.

    let err = app
        .execute_contract(
            Addr::unchecked("user"),
            addr,
            &ExecuteMsg::CreateMarginOrder {
                position: MarginPosition::Short,
                collateral: coin(35, "usdc"),
                leverage: Decimal::from_atomics(Uint128::new(500), 2).unwrap(),
                borrow_asset: "uatom".to_string(),
                take_profit_price: Decimal::from_atomics(Uint128::new(500), 2).unwrap(),
                order_type: OrderType::StopLoss,
                trigger_price: OrderPrice {
                    base_denom: "uatom".to_string(),
                    quote_denom: "uusdc".to_string(),
                    rate: Decimal::from_str("1.25").unwrap(),
                },
            },
            &[coin(45, "usdc")],
        )
        .unwrap_err();

    // Verify that the error is of type "CollateralAmount."
    assert_eq!(ContractError::CollateralAmount, err.downcast().unwrap());
}
