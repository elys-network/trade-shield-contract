use crate::tests::mock::multitest::ElysApp;

use super::*;
use cosmwasm_std::Coin;
use cw_multi_test::AppResponse;

#[test]
fn successful_process_5_of_10_orders() {
    let wallets: Vec<(&str, Vec<Coin>)> = vec![
        ("owner", vec![coin(11, "btc"), coin(9, "eth")]),
        ("user", vec![]),
    ];
    let mut app = ElysApp::new_with_wallets(wallets);

    let prices_at_t0 = vec![coin(20000, "btc"), coin(1, "usdc"), coin(2000, "eth")];
    let prices_at_t1 = vec![coin(30000, "btc"), coin(1, "usdc"), coin(1700, "eth")];

    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));

    let orders = create_dummy_orders();

    let instantiate_msg = InstantiateMsg { orders };
    let execute_msg = ExecuteMsg::ProcessOrder {};

    let addr = app
        .instantiate_contract(
            code_id,
            Addr::unchecked("owner"),
            &instantiate_msg,
            &vec![coin(11, "btc"), coin(9, "eth")],
            "Contract",
            None,
        )
        .unwrap();

    app.init_modules(|router, _, store| router.custom.set_prices(store, &prices_at_t0))
        .unwrap();

    let resp = app
        .execute_contract(addr.clone(), addr.clone(), &execute_msg, &[])
        .unwrap();

    assert_eq!(
        app.wrap()
            .query_balance(&addr, "btc")
            .unwrap()
            .amount
            .u128(),
        11
    );
    assert_eq!(
        app.wrap()
            .query_balance(&addr, "eth")
            .unwrap()
            .amount
            .u128(),
        9
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
            .query_balance("user", "btc")
            .unwrap()
            .amount
            .u128(),
        0
    );
    assert_eq!(
        app.wrap()
            .query_balance("user", "eth")
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

    let order_ids: Vec<u128> = read_processed_order_id(resp);

    assert!(order_ids.is_empty());

    app.init_modules(|router, _, store| router.custom.set_prices(store, &prices_at_t1))
        .unwrap();

    let resp = app
        .execute_contract(addr.clone(), addr.clone(), &execute_msg, &[])
        .unwrap();

    assert_eq!(
        app.wrap()
            .query_balance(&addr, "btc")
            .unwrap()
            .amount
            .u128(),
        5
    );
    assert_eq!(
        app.wrap()
            .query_balance(&addr, "eth")
            .unwrap()
            .amount
            .u128(),
        3
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
            .query_balance("user", "btc")
            .unwrap()
            .amount
            .u128(),
        0
    );
    assert_eq!(
        app.wrap()
            .query_balance("user", "eth")
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
        190200
    );

    let order_ids: Vec<u128> = read_processed_order_id(resp);

    assert!(order_ids.contains(&instantiate_msg.orders[0].order_id));
    assert!(order_ids.contains(&instantiate_msg.orders[3].order_id));
    assert!(order_ids.contains(&instantiate_msg.orders[6].order_id));
    assert!(order_ids.contains(&instantiate_msg.orders[7].order_id));
    assert!(order_ids.contains(&instantiate_msg.orders[8].order_id));
}

fn read_processed_order_id(resp: AppResponse) -> Vec<u128> {
    let mut order_ids: Vec<u128> = vec![];
    for event in resp.events {
        if let Some(attr) = event.attributes.iter().find(|attr| attr.key == "order_id") {
            order_ids.push(attr.value.parse::<u128>().unwrap());
        }
    }
    order_ids
}

fn create_dummy_orders() -> Vec<Order> {
    vec![
        Order {
            order_type: OrderType::StopLoss,
            order_id: 0,
            order_price: coin(1950, "usdc"),
            order_amount: coin(1, "eth"),
            owner_address: Addr::unchecked("user"),
        },
        Order {
            order_type: OrderType::StopLoss,
            order_id: 1,
            order_price: coin(12000, "usdc"),
            order_amount: coin(2, "btc"),
            owner_address: Addr::unchecked("user"),
        },
        Order {
            order_type: OrderType::StopLoss,
            order_id: 2,
            order_price: coin(10000, "usdc"),
            order_amount: coin(3, "btc"),
            owner_address: Addr::unchecked("user"),
        },
        Order {
            order_type: OrderType::StopLoss,
            order_id: 3,
            order_price: coin(1500, "usdc"),
            order_amount: coin(5, "eth"),
            owner_address: Addr::unchecked("user"),
        },
        Order {
            order_type: OrderType::StopLoss,
            order_id: 4,
            order_price: coin(1800, "usdc"),
            order_amount: coin(1, "eth"),
            owner_address: Addr::unchecked("user"),
        },
        Order {
            order_type: OrderType::TakeProfit,
            order_id: 5,
            order_price: coin(2500, "usdc"),
            order_amount: coin(1, "eth"),
            owner_address: Addr::unchecked("user"),
        },
        Order {
            order_type: OrderType::TakeProfit,
            order_id: 6,
            order_price: coin(21000, "usdc"),
            order_amount: coin(3, "btc"),
            owner_address: Addr::unchecked("user"),
        },
        Order {
            order_type: OrderType::TakeProfit,
            order_id: 7,
            order_price: coin(25000, "usdc"),
            order_amount: coin(2, "btc"),
            owner_address: Addr::unchecked("user"),
        },
        Order {
            order_type: OrderType::TakeProfit,
            order_id: 8,
            order_price: coin(30000, "usdc"),
            order_amount: coin(1, "btc"),
            owner_address: Addr::unchecked("user"),
        },
        Order {
            order_type: OrderType::TakeProfit,
            order_id: 9,
            order_price: coin(2100, "usdc"),
            order_amount: coin(1, "eth"),
            owner_address: Addr::unchecked("user"),
        },
    ]
}
