use cosmwasm_std::{to_json_binary, Coin};

use super::*;

#[test]
fn successfully_cancel_orders_ids() {
    let wallet: Vec<(&str, Vec<Coin>)> = vec![(
        "owner",
        vec![coin(16, "btc"), coin(5, "eth"), coin(20, "usdt")],
    )];

    let spot_orders = vec![
        SpotOrder {
            order_type: SpotOrderType::LimitBuy,
            order_id: 0,
            order_price: OrderPrice {
                base_denom: "".to_string(),
                quote_denom: "".to_string(),
                rate: Decimal::zero(),
            },
            order_amount: coin(10, "btc"),
            owner_address: Addr::unchecked("user"),
            order_target_denom: "".to_string(),
            status: Status::NotProcessed,
        },
        SpotOrder {
            order_type: SpotOrderType::LimitSell,
            order_id: 1,
            order_price: OrderPrice {
                base_denom: "".to_string(),
                quote_denom: "".to_string(),
                rate: Decimal::zero(),
            },
            order_amount: coin(5, "eth"),
            owner_address: Addr::unchecked("user"),
            order_target_denom: "".to_string(),
            status: Status::NotProcessed,
        },
        SpotOrder {
            order_type: SpotOrderType::StopLoss,
            order_id: 2,
            order_price: OrderPrice {
                base_denom: "".to_string(),
                quote_denom: "".to_string(),
                rate: Decimal::zero(),
            },
            order_amount: coin(20, "usdt"),
            owner_address: Addr::unchecked("user1"),
            order_target_denom: "".to_string(),
            status: Status::NotProcessed,
        },
        SpotOrder {
            order_type: SpotOrderType::StopLoss,
            order_id: 3,
            order_price: OrderPrice {
                base_denom: "".to_string(),
                quote_denom: "".to_string(),
                rate: Decimal::zero(),
            },
            order_amount: coin(6, "btc"),
            owner_address: Addr::unchecked("user"),
            order_target_denom: "".to_string(),
            status: Status::NotProcessed,
        },
    ];

    let mut app = ElysApp::new_with_wallets(wallet);

    // Create a mock message to instantiate the contract with an empty list of orders.
    let instantiate_msg = InstantiateMockMsg {
        spot_orders,
        margin_orders: vec![],
    };

    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));

    let addr = app
        .instantiate_contract(
            code_id,
            Addr::unchecked("owner"),
            &instantiate_msg,
            &vec![coin(16, "btc"), coin(5, "eth"), coin(20, "usdt")],
            "Contract",
            None,
        )
        .unwrap();

    assert_eq!(
        app.wrap().query_balance(&addr, "btc").unwrap(),
        coin(16, "btc")
    );

    assert_eq!(
        app.wrap().query_balance(&addr, "eth").unwrap(),
        coin(5, "eth")
    );
    assert_eq!(
        app.wrap().query_balance(&addr, "usdt").unwrap(),
        coin(20, "usdt")
    );

    assert_eq!(
        app.wrap().query_balance("user", "btc").unwrap(),
        coin(0, "btc")
    );

    assert_eq!(
        app.wrap().query_balance("user", "eth").unwrap(),
        coin(0, "eth")
    );
    assert_eq!(
        app.wrap().query_balance("user", "usdt").unwrap(),
        coin(0, "usdt")
    );

    let resp = app
        .execute_contract(
            Addr::unchecked("user"),
            addr.clone(),
            &&ExecuteMsg::CancelSpotOrders {
                order_ids: Some(vec![1, 3]),
                owner_address: "user".to_string(),
                order_type: None,
            },
            &[],
        )
        .unwrap();

    assert_eq!(resp.data.unwrap(), to_json_binary(&vec![1, 3]).unwrap());

    assert_eq!(
        app.wrap().query_balance("user", "btc").unwrap(),
        coin(6, "btc")
    );

    assert_eq!(
        app.wrap().query_balance("user", "eth").unwrap(),
        coin(5, "eth")
    );
    assert_eq!(
        app.wrap().query_balance("user", "usdt").unwrap(),
        coin(0, "usdt")
    );

    assert_eq!(
        app.wrap().query_balance(&addr, "btc").unwrap(),
        coin(10, "btc")
    );

    assert_eq!(
        app.wrap().query_balance(&addr, "eth").unwrap(),
        coin(0, "eth")
    );
    assert_eq!(
        app.wrap().query_balance(&addr, "usdt").unwrap(),
        coin(20, "usdt")
    );
}
