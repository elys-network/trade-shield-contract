use super::*;
use cw_multi_test::BankSudo;

#[test]
fn unauthorize() {
    // Initialize the ElysApp.
    let mut app = ElysApp::new();

    // Create a mock message to instantiate the contract with no initial orders.
    let instantiate_msg = InstantiateMockMsg {
        spot_orders: vec![],
        margin_orders: vec![MarginOrder::new_open(
            "user",
            &MarginPosition::Long,
            &MarginOrderType::LimitOpen,
            &coin(255, "usdc"),
            "btc",
            &Decimal::from_str("1.2").unwrap(),
            &Decimal::from_str("1.2").unwrap(),
            &Some(OrderPrice {
                base_denom: "btc".to_string(),
                quote_denom: "usdc".to_string(),
                rate: Decimal::from_str("20000.0").unwrap(),
            }),
            &vec![],
        )
        .unwrap()],
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

    // Mint the token from the order to simulate that the tokens are already locked.

    app.sudo(
        BankSudo::Mint {
            to_address: addr.to_string(),
            amount: coins(255, "usdc"),
        }
        .into(),
    )
    .unwrap();

    let err = app
        .execute_contract(
            Addr::unchecked("not-user"),
            addr.clone(),
            &ExecuteMsg::CancelMarginOrder { order_id: 0 },
            &[],
        )
        .unwrap_err();

    assert_eq!(
        app.wrap()
            .query_balance(&addr, "usdc")
            .unwrap()
            .amount
            .u128(),
        255
    );

    assert_eq!(
        app.wrap()
            .query_balance("not-user", "usdc")
            .unwrap()
            .amount
            .u128(),
        0
    );

    assert_eq!(
        ContractError::Unauthorized {
            sender: Addr::unchecked("not-user")
        },
        err.downcast().unwrap()
    );
}
