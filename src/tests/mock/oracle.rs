use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::testing::{MockApi, MockQuerier, MockStorage};
use cosmwasm_std::{
    to_binary, Coin, Empty, OwnedDeps, Querier, QuerierResult, QueryRequest, SystemError,
    SystemResult,
};
use std::marker::PhantomData;

// Define custom query enum
#[cw_serde]
#[derive(QueryResponses)]
pub enum OracleElys {
    #[returns(GetAllPricesResp)]
    GetAllPrices {},
    // Define other query variants if needed
}

// Define the response struct for GetAllPrices query
#[cw_serde]
pub struct GetAllPricesResp {
    pub prices: Vec<Coin>,
}

// Define the mock querier struct
pub struct OracleMock {
    prices: Vec<Coin>,
}

pub struct OracleMockWithQuerier {
    oracle_mock: OracleMock,
    base: MockQuerier,
}

impl OracleMockWithQuerier {
    fn new() -> Self {
        OracleMockWithQuerier {
            oracle_mock: OracleMock { prices: vec![] },
            base: MockQuerier::default(),
        }
    }

    pub fn update_price(&mut self, prices: &[Coin]) {
        self.oracle_mock.prices = prices.to_vec();
    }

    pub fn handle_query(&self, request: &QueryRequest<OracleElys>) -> QuerierResult {
        match &request {
            QueryRequest::Custom(custom) => match custom {
                OracleElys::GetAllPrices {} => SystemResult::Ok(
                    to_binary(&GetAllPricesResp {
                        prices: self.oracle_mock.prices.clone(),
                    })
                    .into(),
                ),
            },

            _ => self.base.handle_query(&QueryRequest::Custom(Empty {})),
        }
    }
}

impl Querier for OracleMockWithQuerier {
    fn raw_query(&self, bin_request: &[u8]) -> QuerierResult {
        let request: QueryRequest<OracleElys> = match cosmwasm_std::from_slice(bin_request) {
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
pub fn mock_dependencies() -> OwnedDeps<MockStorage, MockApi, OracleMockWithQuerier, Empty> {
    let querier: OracleMockWithQuerier = OracleMockWithQuerier::new();

    OwnedDeps {
        storage: MockStorage::default(),
        api: MockApi::default(),
        querier,
        custom_query_type: PhantomData,
    }
}
