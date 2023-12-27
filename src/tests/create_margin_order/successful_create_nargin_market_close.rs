use std::str::FromStr;

use cosmwasm_std::Int128;

use super::*;

#[test]
fn successful_create_margin_market_open_order() {
    // Initialize the ElysApp instance with the specified wallet.
    let mut app = ElysApp::new();

    // Create a mock message to instantiate the contract with no initial orders.
    let instantiate_msg = InstantiateMockMsg {
        spot_orders: vec![],
        margin_orders: vec![],
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

    app.init_modules(|r, _, s| {
        r.custom.set_mtp(
            s,
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
    // User "user" creates a "MakerBuy" margin order for BTC
    app.execute_contract(
        Addr::unchecked("user"),
        addr.clone(),
        &ExecuteMsg::CreateMarginOrder {
            position_id: Some(1),
            position: None,
            leverage: None,
            trading_asset: None,
            take_profit_price: None,
            order_type: MarginOrderType::MarketClose,
            trigger_price: None,
        },
        &[],
    )
    .unwrap();

    let last_module = app
        .init_modules(|router, _, store| router.custom.get_last_module(store).unwrap())
        .unwrap();

    assert_eq!(last_module, "MarginClose");
}
