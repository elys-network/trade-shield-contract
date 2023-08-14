use crate::tests::elys_oracle::{query::ElysQuery, query_resp::GetAllPricesResp};
use anyhow::{bail, Result as AnyResult};
use cosmwasm_std::{
    coin,
    testing::{MockApi, MockStorage},
    to_binary, Addr, BlockInfo, Coin, Empty, Querier, StdResult, Storage,
};
use cw_multi_test::{App, BankKeeper, BasicAppBuilder, Module, WasmKeeper};
use cw_storage_plus::Item;
use std::cmp::max;
use std::ops::{Deref, DerefMut};

pub const PRICES: Item<Vec<Coin>> = Item::new("prices");
pub const BLOCK_TIME: u64 = 5;

pub struct OracleModule {}

impl OracleModule {
    fn get_all_price(&self, store: &dyn Storage) -> StdResult<GetAllPricesResp> {
        Ok(GetAllPricesResp {
            prices: PRICES.load(store)?,
        })
    }

    pub fn set_prices(&self, store: &mut dyn Storage, prices: &Vec<Coin>) -> StdResult<()> {
        PRICES.save(store, prices)
    }

    pub fn new_price(&self, store: &mut dyn Storage, new_price: &Coin) -> StdResult<()> {
        let mut prices = PRICES.load(store)?;
        for price in prices.iter_mut() {
            if price.denom == new_price.denom {
                price.amount = new_price.amount;
                return PRICES.save(store, &prices);
            }
        }
        prices.push(new_price.to_owned());
        PRICES.save(store, &prices)
    }
}

impl Module for OracleModule {
    type ExecT = Empty;
    type QueryT = ElysQuery;
    type SudoT = Empty;

    fn query(
        &self,
        api: &dyn cosmwasm_std::Api,
        storage: &dyn cosmwasm_std::Storage,
        querier: &dyn cosmwasm_std::Querier,
        block: &cosmwasm_std::BlockInfo,
        request: Self::QueryT,
    ) -> AnyResult<cosmwasm_std::Binary> {
        match request {
            ElysQuery::GetAllPrices {} => Ok(to_binary(&self.get_all_price(storage)?)?),
        }
    }

    fn execute<ExecC, QueryC>(
        &self,
        api: &dyn cosmwasm_std::Api,
        storage: &mut dyn cosmwasm_std::Storage,
        router: &dyn cw_multi_test::CosmosRouter<ExecC = ExecC, QueryC = QueryC>,
        block: &cosmwasm_std::BlockInfo,
        sender: cosmwasm_std::Addr,
        msg: Self::ExecT,
    ) -> AnyResult<cw_multi_test::AppResponse>
    where
        ExecC: std::fmt::Debug
            + Clone
            + PartialEq
            + schemars::JsonSchema
            + serde::de::DeserializeOwned
            + 'static,
        QueryC: cosmwasm_std::CustomQuery + serde::de::DeserializeOwned + 'static,
    {
        bail!("execute is not implemented for OracleModule")
    }

    fn sudo<ExecC, QueryC>(
        &self,
        api: &dyn cosmwasm_std::Api,
        storage: &mut dyn cosmwasm_std::Storage,
        router: &dyn cw_multi_test::CosmosRouter<ExecC = ExecC, QueryC = QueryC>,
        block: &cosmwasm_std::BlockInfo,
        msg: Self::SudoT,
    ) -> AnyResult<cw_multi_test::AppResponse>
    where
        ExecC: std::fmt::Debug
            + Clone
            + PartialEq
            + schemars::JsonSchema
            + serde::de::DeserializeOwned
            + 'static,
        QueryC: cosmwasm_std::CustomQuery + serde::de::DeserializeOwned + 'static,
    {
        bail!("execute is not implemented for OracleModule")
    }
}

pub type OracleAppWrapped =
    App<BankKeeper, MockApi, MockStorage, OracleModule, WasmKeeper<Empty, ElysQuery>>;

pub struct OracleApp(OracleAppWrapped);

impl Deref for OracleApp {
    type Target = OracleAppWrapped;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for OracleApp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Querier for OracleApp {
    fn raw_query(&self, bin_request: &[u8]) -> cosmwasm_std::QuerierResult {
        self.0.raw_query(bin_request)
    }
}

impl Default for OracleApp {
    fn default() -> Self {
        Self::new()
    }
}

impl OracleApp {
    pub fn new_with_wallets(wallets: Vec<(&str, Vec<Coin>)>) -> Self {
        Self(
            BasicAppBuilder::<Empty, ElysQuery>::new_custom()
                .with_custom(OracleModule {})
                .build(|roouter, _, storage| {
                    for (wallet_owner, wallet_contenent) in wallets {
                        roouter
                            .bank
                            .init_balance(storage, &Addr::unchecked(wallet_owner), wallet_contenent)
                            .unwrap();
                    }
                }),
        )
    }

    pub fn new() -> Self {
        Self(
            BasicAppBuilder::<Empty, ElysQuery>::new_custom()
                .with_custom(OracleModule {})
                .build(|_roouter, _, _storage| {}),
        )
    }
    pub fn block_info(&self) -> BlockInfo {
        self.0.block_info()
    }
    pub fn advance_blocks(&mut self, blocks: u64) {
        self.update_block(|block| {
            block.time = block.time.plus_seconds(BLOCK_TIME * blocks);
            block.height += blocks;
        });
    }

    /// This advances BlockInfo by given number of seconds.
    /// It does not do any callbacks, but keeps the ratio of seconds/block
    pub fn advance_seconds(&mut self, seconds: u64) {
        self.update_block(|block| {
            block.time = block.time.plus_seconds(seconds);
            block.height += max(1, seconds / BLOCK_TIME);
        });
    }

    /// Simple iterator when you don't care too much about the details and just want to
    /// simulate forward motion.
    pub fn next_block(&mut self) {
        self.advance_blocks(1)
    }
}
