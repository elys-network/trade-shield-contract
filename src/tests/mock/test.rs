use super::oracle::*;
use cosmwasm_std::{coin, from_binary, Coin};

#[test]
fn test_get_all_prices_query() {
    let mut deps = mock_dependencies();
    let prices: Vec<Coin> = vec![coin(1000000, "uusd"), coin(8000000, "ukrw")];
    deps.querier.update_price(&prices);

    let bin = deps
        .querier
        .handle_query(&cosmwasm_std::QueryRequest::Custom(
            OracleElys::GetAllPrices {},
        ))
        .unwrap()
        .unwrap();
    let get_all_prices_response: GetAllPricesResp = from_binary(&bin).unwrap();
    assert_eq!(get_all_prices_response.prices[0].denom, prices[0].denom);
    assert_eq!(get_all_prices_response.prices[0].amount, prices[0].amount);
    assert_eq!(get_all_prices_response.prices[1].denom, prices[1].denom);
    assert_eq!(get_all_prices_response.prices[1].amount, prices[1].amount);
}
