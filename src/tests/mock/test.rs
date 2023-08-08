use super::oracle::*;
use cosmwasm_std::{
    coin,
    testing::{mock_dependencies, mock_env, mock_info, MockQuerier},
    Coin,
};

#[test]
fn test_get_all_prices_query() {
    let mut deps = mock_dependencies();
    let prices: Vec<Coin> = vec![coin(1000000, "uusd"), coin(8000000, "ukrw")];
    // deps.querier.oracle_mock.change_prices(prices.clone());

    let env = mock_env();
    let info = mock_info("sender", &[]);
    let get_all_prices_response = vec![];

    assert_eq!(get_all_prices_response.prices[0].denom, prices[0].denom);
    assert_eq!(get_all_prices_response.prices[0].amount, prices[0].amount);
    assert_eq!(get_all_prices_response.prices[1].denom, prices[1].denom);
    assert_eq!(get_all_prices_response.prices[1].amount, prices[1].amount);
}
