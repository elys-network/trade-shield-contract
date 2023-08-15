use cosmwasm_std::{coin, Coin};

use crate::bindings::{query::ElysQuery, query_resp::GetAllPricesResp};

use super::multitest::*;

fn check_prices(app: &mut ElysApp, prices: &Vec<Coin>) {
    let prices = prices.to_owned();
    let request = ElysQuery::GetAllPrices {}.into();
    let actual_prices: GetAllPricesResp = app.wrap().query(&request).unwrap();
    assert_eq!(prices, actual_prices.prices);
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
