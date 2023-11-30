use super::*;
use std::str::FromStr;

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

    app.execute_contract(
        Addr::unchecked("user"),
        addr,
        &ExecuteMsg::CreateMarginOrder {
            position: Some(MarginPosition::Short),
            leverage: Some(Decimal::from_atomics(Uint128::new(500), 2).unwrap()),
            borrow_asset: Some("uatom".to_string()),
            take_profit_price: Some(Decimal::from_atomics(Uint128::new(500), 2).unwrap()),
            order_type: MarginOrderType::LimitOpen,
            trigger_price: Some(OrderPrice {
                base_denom: "uatom".to_string(),
                quote_denom: "uusdc".to_string(),
                rate: Decimal::from_str("1.5").unwrap(),
            }),
            position_id: None,
        },
        &[],
    )
    .unwrap_err();
}
