use cosmwasm_std::{coin, coins, Addr, Coin, Decimal, Int128, Uint128};
use cw_multi_test::Executor;

use crate::{
    bindings::{msg::ElysMsg, query::ElysQuery, query_resp::QuerySwapEstimationResponse},
    types::{PageRequest, SwapAmountInRoute},
};

use super::multitest::*;

fn check_prices(app: &mut ElysApp, prices: &Vec<Coin>) {
    let dummy_req = PageRequest::new(20);

    let prices = prices.to_owned();
    let request = ElysQuery::PriceAll {
        pagination: dummy_req,
    }
    .into();
    let actual_prices: Vec<Coin> = app.wrap().query(&request).unwrap();
    assert_eq!(prices, actual_prices);
}

#[test]
fn query_price() {
    let mut prices: Vec<Coin> = vec![coin(20000, "btc"), coin(1, "usdc")];
    let mut app = ElysApp::new();

    app.init_modules(|router, _, storage| router.custom.set_prices(storage, &prices))
        .unwrap();

    check_prices(&mut app, &prices);

    let new_price = coin(1700, "eth");
    app.init_modules(|router, _, storage| router.custom.new_price(storage, &new_price))
        .unwrap();
    prices.push(new_price);

    check_prices(&mut app, &prices);

    let new_price: Coin = coin(1200, "eth");
    app.init_modules(|router, _, storage| router.custom.new_price(storage, &new_price))
        .unwrap();
    prices[2].amount = new_price.amount;
    check_prices(&mut app, &prices);
}

#[test]
fn swap() {
    let prices: Vec<Coin> = vec![coin(20000, "btc"), coin(1, "usdc")];
    let wallets: Vec<(&str, Vec<Coin>)> = vec![("user", coins(5, "btc"))];
    let mut app = ElysApp::new_with_wallets(wallets);
    app.init_modules(|router, _, storage| router.custom.set_prices(storage, &prices))
        .unwrap();

    let msg = ElysMsg::MsgSwapExactAmountIn {
        sender: "user".to_string(),
        routes: vec![SwapAmountInRoute::new(1, "usdc")],
        token_in: coin(5, "btc"),
        token_out_min_amount: Int128::zero(),
        meta_data: None,
    };

    assert_eq!(
        app.wrap()
            .query_balance("user", "btc")
            .unwrap()
            .amount
            .u128(),
        5
    );
    assert_eq!(
        app.wrap()
            .query_balance("user", "usdc")
            .unwrap()
            .amount
            .u128(),
        0
    );

    app.execute(Addr::unchecked("user"), msg.into()).unwrap();

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
            .query_balance("user", "usdc")
            .unwrap()
            .amount
            .u128(),
        5 * 20000
    );
}

#[test]
pub fn swap_estimation() {
    let prices: Vec<Coin> = vec![coin(20000, "btc"), coin(1, "usdc")];
    let mut app = ElysApp::new();

    app.init_modules(|router, _, storage| router.custom.set_prices(storage, &prices))
        .unwrap();

    let swap: QuerySwapEstimationResponse = app
        .wrap()
        .query(&cosmwasm_std::QueryRequest::Custom(
            ElysQuery::QuerySwapEstimation {
                routes: vec![SwapAmountInRoute::new(1, "usdc")],
                token_in: coin(5, "btc"),
            },
        ))
        .unwrap();

    assert_eq!(
        swap.spot_price,
        Decimal::from_atomics(Uint128::new(20000), 0).unwrap()
    );

    assert_eq!(swap.token_out, coin(100000, "usdc"));
}
