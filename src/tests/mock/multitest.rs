use crate::{
    bindings::{
        msg::ElysMsg, msg_resp::MsgSwapExactAmountInResp, query::ElysQuery,
        query_resp::QuerySwapEstimationResponse,
    },
    types::AssetInfo,
};
use anyhow::{bail, Error, Result as AnyResult};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    coin, coins, from_binary,
    testing::{MockApi, MockStorage},
    to_binary, Addr, BankMsg, BlockInfo, Coin, Decimal, Empty, Querier, StdError, StdResult,
    Storage, Uint128,
};
use cw_multi_test::{App, AppResponse, BankKeeper, BankSudo, BasicAppBuilder, Module, WasmKeeper};
use cw_storage_plus::Item;
use std::cmp::max;
use std::ops::{Deref, DerefMut};

pub const PRICES: Item<Vec<Coin>> = Item::new("prices");
pub const ASSET_INFO: Item<Vec<AssetInfo>> = Item::new("asset_info");
pub const BLOCK_TIME: u64 = 5;

pub struct ElysModule {}

impl ElysModule {
    fn get_all_price(&self, store: &dyn Storage) -> StdResult<Vec<Coin>> {
        Ok(PRICES.load(store)?)
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
    pub fn set_asset_infos(
        &self,
        store: &mut dyn Storage,
        infos: &Vec<AssetInfo>,
    ) -> StdResult<()> {
        ASSET_INFO.save(store, infos)
    }
}

impl Module for ElysModule {
    type ExecT = ElysMsg;
    type QueryT = ElysQuery;
    type SudoT = Empty;

    fn query(
        &self,
        _api: &dyn cosmwasm_std::Api,
        storage: &dyn cosmwasm_std::Storage,
        _querier: &dyn cosmwasm_std::Querier,
        _block: &cosmwasm_std::BlockInfo,
        request: Self::QueryT,
    ) -> AnyResult<cosmwasm_std::Binary> {
        match request {
            ElysQuery::PriceAll { pagination } => Ok(to_binary(&self.get_all_price(storage)?)?),
            ElysQuery::QuerySwapEstimation { routes, token_in } => {
                let prices = &self.get_all_price(storage)?;
                let price_in = prices
                    .iter()
                    .find(|price| price.denom == token_in.denom)
                    .unwrap();
                let price_out = prices
                    .iter()
                    .find(|price| price.denom == routes[0].token_out_denom())
                    .unwrap();
                let spot_price = Decimal::from_ratio(price_in.amount, price_out.amount);
                let token_out_amount =
                    (Decimal::from_atomics(token_in.amount, spot_price.decimal_places())?
                        * spot_price)
                        .atomics()
                        .u128();

                Ok(to_binary(&QuerySwapEstimationResponse {
                    spot_price,
                    token_out: coin(token_out_amount, routes[0].token_out_denom()),
                })?)
            }

            ElysQuery::AssetInfo { denom } => {
                let infos = ASSET_INFO.load(storage)?;
                let may_have_info = infos.iter().find(|asset| asset.denom == denom);

                match may_have_info {
                    Some(info) => Ok(to_binary(info)?),
                    None => Err(Error::new(StdError::NotFound {
                        kind: "asset denom".to_string(),
                    })),
                }
            }
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
        match msg {
            ElysMsg::MsgSwapExactAmountIn {
                sender,
                routes,
                token_in,
                token_out_min_amount,
                meta_data,
            } => {
                let route = routes[0].clone();
                let prices = self.get_all_price(storage)?;
                let price_in = prices.iter().find(|p| p.denom == token_in.denom).unwrap();
                let price_out = prices
                    .iter()
                    .find(|p| p.denom == route.token_out_denom())
                    .unwrap();

                let mint_amount = coins(
                    (token_in.amount * price_in.amount / price_out.amount).u128(),
                    route.token_out_denom(),
                );
                let data = to_binary(&MsgSwapExactAmountInResp::new(
                    mint_amount[0].amount.u128() as i64,
                    meta_data,
                ))?;

                let mint = BankSudo::Mint {
                    to_address: sender.clone(),
                    amount: mint_amount.clone(),
                };

                let burn = BankMsg::Burn {
                    amount: vec![token_in],
                };
                router
                    .execute(
                        api,
                        storage,
                        block,
                        Addr::unchecked(sender.clone()),
                        burn.into(),
                    )
                    .unwrap();
                router.sudo(api, storage, block, mint.into()).unwrap();

                Ok(AppResponse {
                    events: vec![],
                    data: Some(data),
                })
            }
        }
    }

    fn sudo<ExecC, QueryC>(
        &self,
        _api: &dyn cosmwasm_std::Api,
        _storage: &mut dyn cosmwasm_std::Storage,
        _router: &dyn cw_multi_test::CosmosRouter<ExecC = ExecC, QueryC = QueryC>,
        _block: &cosmwasm_std::BlockInfo,
        _msg: Self::SudoT,
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
        bail!("execute is not implemented for ElysMsg")
    }
}

pub type ElysAppWrapped =
    App<BankKeeper, MockApi, MockStorage, ElysModule, WasmKeeper<ElysMsg, ElysQuery>>;

pub struct ElysApp(ElysAppWrapped);

impl Deref for ElysApp {
    type Target = ElysAppWrapped;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ElysApp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Querier for ElysApp {
    fn raw_query(&self, bin_request: &[u8]) -> cosmwasm_std::QuerierResult {
        self.0.raw_query(bin_request)
    }
}

impl Default for ElysApp {
    fn default() -> Self {
        Self::new()
    }
}

impl ElysApp {
    pub fn new_with_wallets(wallets: Vec<(&str, Vec<Coin>)>) -> Self {
        Self(
            BasicAppBuilder::<ElysMsg, ElysQuery>::new_custom()
                .with_custom(ElysModule {})
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
            BasicAppBuilder::<ElysMsg, ElysQuery>::new_custom()
                .with_custom(ElysModule {})
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
