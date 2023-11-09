use crate::msg::query_resp::GetSpotOrdersResp;

use super::*;

#[test]
fn get_spot_orders() {
    let orders: Vec<SpotOrder> = create_orders();
    let mut app = ElysApp::new();

    let instantiate_msg = InstantiateMockMsg {
        process_order_executor: "owner".to_string(),
        orders: orders.clone(),
    };

    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));

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

    let mut page_req = PageRequest::new(2);

    let resp: GetSpotOrdersResp = app
        .wrap()
        .query_wasm_smart(
            &addr,
            &QueryMsg::GetSpotOrders {
                pagination: page_req.clone(),
                order_owner: None,
                order_type: None,
            },
        )
        .unwrap();

    let (first_half, second_half) = orders.split_at(2);
}

fn create_orders() -> Vec<SpotOrder> {
    vec![
        SpotOrder {
            order_type: SpotOrderType::LimitBuy,
            order_id: 0,
            order_price: SpotOrderPrice {
                base_denom: "btc".to_owned(),
                quote_denom: "usdc".to_owned(),
                rate: Decimal::from_atomics(Uint128::new(25), 1).unwrap(),
            },
            order_amount: coin(255, "btc"),
            owner_address: Addr::unchecked("userA"),
            order_target_denom: "btc".to_owned(),
            order_amm_routes: vec![],
        },
        SpotOrder {
            order_type: SpotOrderType::LimitSell,
            order_id: 1,
            order_price: SpotOrderPrice {
                base_denom: "eth".to_owned(),
                quote_denom: "usdt".to_owned(),
                rate: Decimal::from_atomics(Uint128::new(10), 1).unwrap(),
            },
            order_amount: coin(100, "eth"),
            owner_address: Addr::unchecked("userB"),
            order_target_denom: "eth".to_owned(),
            order_amm_routes: vec![],
        },
        SpotOrder {
            order_type: SpotOrderType::StopLoss,
            order_id: 2,
            order_price: SpotOrderPrice {
                base_denom: "xrp".to_owned(),
                quote_denom: "usdt".to_owned(),
                rate: Decimal::from_atomics(Uint128::new(5), 1).unwrap(),
            },
            order_amount: coin(500, "xrp"),
            owner_address: Addr::unchecked("userC"),
            order_target_denom: "xrp".to_owned(),
            order_amm_routes: vec![],
        },
        SpotOrder {
            order_type: SpotOrderType::StopLoss,
            order_id: 3,
            order_price: SpotOrderPrice {
                base_denom: "ltc".to_owned(),
                quote_denom: "usdc".to_owned(),
                rate: Decimal::from_atomics(Uint128::new(15), 1).unwrap(),
            },
            order_amount: coin(75, "ltc"),
            owner_address: Addr::unchecked("userD"),
            order_target_denom: "ltc".to_owned(),
            order_amm_routes: vec![],
        },
        SpotOrder {
            order_type: SpotOrderType::LimitBuy,
            order_id: 4,
            order_price: SpotOrderPrice {
                base_denom: "ada".to_owned(),
                quote_denom: "usdt".to_owned(),
                rate: Decimal::from_atomics(Uint128::new(3), 1).unwrap(),
            },
            order_amount: coin(200, "ada"),
            owner_address: Addr::unchecked("userE"),
            order_target_denom: "ada".to_owned(),
            order_amm_routes: vec![],
        },
    ]
}
