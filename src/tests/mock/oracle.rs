use cosmwasm_std::testing::{MockApi, MockQuerier, MockStorage};
use cosmwasm_std::{
    to_binary, BankQuery, Coin, Empty, OwnedDeps, Querier, QuerierResult, QueryRequest,
    SystemError, SystemResult,
};
use std::marker::PhantomData;

use crate::tests::elys_oracle::query::ElysQuery;
use crate::tests::elys_oracle::query_resp::GetAllPricesResp;

// Define the mock querier struct
pub struct OracleMock {
    prices: Vec<Coin>,
}

pub struct OracleMockWithQuerier {
    oracle_mock: OracleMock,
    base: MockQuerier,
}

impl OracleMockWithQuerier {
    fn new(balances: &[(&str, &[Coin])]) -> Self {
        OracleMockWithQuerier {
            oracle_mock: OracleMock { prices: vec![] },
            base: MockQuerier::new(balances),
        }
    }

    pub fn update_price(&mut self, prices: &[Coin]) {
        self.oracle_mock.prices = prices.to_vec();
    }

    pub fn handle_query(&self, request: &QueryRequest<ElysQuery>) -> QuerierResult {
        match &request {
            QueryRequest::Custom(custom) => match custom {
                ElysQuery::GetAllPrices {} => SystemResult::Ok(
                    to_binary(&GetAllPricesResp {
                        prices: self.oracle_mock.prices.clone(),
                    })
                    .into(),
                ),
            },

            _ => self.base.handle_query(&QueryRequest::Custom(Empty {})),
        }
    }
    pub fn check_wallet(&self, address: &str, denom: &str) -> QuerierResult {
        let (address, denom) = (address.to_string(), denom.to_string());
        let request = QueryRequest::Bank(BankQuery::Balance { address, denom });
        self.base.handle_query(&request)
    }
}

impl Querier for OracleMockWithQuerier {
    fn raw_query(&self, bin_request: &[u8]) -> QuerierResult {
        let request: QueryRequest<ElysQuery> = match cosmwasm_std::from_slice(bin_request) {
            Ok(v) => v,
            Err(e) => {
                return QuerierResult::Err(SystemError::InvalidRequest {
                    error: format!("Parsing query request: {}", e),
                    request: bin_request.into(),
                })
            }
        };
        self.handle_query(&request)
    }
}

// You can use the OracleMockWithQuerier in your tests using mock_dependencies
pub fn mock_dependencies(
    balances: &Vec<(&str, Vec<Coin>)>,
) -> OwnedDeps<MockStorage, MockApi, OracleMockWithQuerier, Empty> {
    let balances: &[(&str, &[Coin])] = &balances
        .iter()
        .map(|(addr, coins)| (*addr, coins.as_slice()))
        .collect::<Vec<(&str, &[Coin])>>();

    let querier: OracleMockWithQuerier = OracleMockWithQuerier::new(balances);

    OwnedDeps {
        storage: MockStorage::default(),
        api: MockApi::default(),
        querier,
        custom_query_type: PhantomData,
    }
}
