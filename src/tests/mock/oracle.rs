use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::testing::{MockApi, MockQuerier, MockStorage};
use cosmwasm_std::{Coin, Empty, OwnedDeps, Querier, QuerierResult, QueryRequest, SystemError};
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

impl OracleMock {
    pub fn new() -> OracleMock {
        OracleMock { prices: vec![] }
    }

    pub fn change_prices(&mut self, prices: Vec<Coin>) {
        self.prices = prices;
    }
}

impl Querier for OracleMock {
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

        match request {
            QueryRequest::Custom(custom_query) => {
                // Process the custom query and return a mock response
                let response = match custom_query {
                    OracleElys::GetAllPrices {} => {
                        // Simulate a response based on the parameter
                        GetAllPricesResp {
                            prices: self.prices.clone(),
                        }
                    } // Handle other query types if needed
                };

                // Return the mock response as a QuerierResult::Ok
                QuerierResult::Ok(cosmwasm_std::to_binary(&response).into())
            }
            // Handle other query types if needed
            _ => QuerierResult::Err(SystemError::InvalidRequest {
                error: "Unsupported query request".to_string(),
                request: bin_request.into(),
            }),
        }
    }
}

// Define a wrapper struct that combines both OracleMock and MockQuerier traits
pub struct OracleMockWithQuerier {
    pub oracle_mock: OracleMock,
    pub querier: MockQuerier,
}

// Implement the Querier trait for the wrapper struct
impl Querier for OracleMockWithQuerier {
    fn raw_query(&self, bin_request: &[u8]) -> QuerierResult {
        self.oracle_mock.raw_query(bin_request)
    }
}

// You can use the OracleMockWithQuerier in your tests using mock_dependencies
pub fn mock_dependencies() -> OwnedDeps<MockStorage, MockApi, OracleMockWithQuerier, Empty> {
    let oracle_mock = OracleMock::new();
    let querier = MockQuerier::new(&[]);

    OwnedDeps {
        storage: MockStorage::default(),
        api: MockApi::default(),
        querier: OracleMockWithQuerier {
            oracle_mock,
            querier,
        },
        custom_query_type: PhantomData,
    }
}
