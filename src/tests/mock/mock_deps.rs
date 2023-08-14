use std::marker::PhantomData;

use cosmwasm_std::{
    testing::{MockApi, MockStorage},
    Coin, OwnedDeps,
};

use super::multitest::OracleApp;
use crate::tests::elys_oracle::query::ElysQuery;

pub fn mock_dependencies(
    wallets: Vec<(&str, Vec<Coin>)>,
) -> OwnedDeps<MockStorage, MockApi, OracleApp, ElysQuery> {
    let custom_querier: OracleApp = OracleApp::new_with_wallets(wallets);

    OwnedDeps {
        storage: MockStorage::default(),
        api: MockApi::default(),
        querier: custom_querier,
        custom_query_type: PhantomData,
    }
}
