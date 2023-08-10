use super::*;
use crate::{execute, instantiate};
use cosmwasm_std::{
    from_binary,
    testing::{mock_env, mock_info, MOCK_CONTRACT_ADDR},
    BalanceResponse, Coin,
};
use mock::oracle::*;

// in this exemple we assume :
//  USDC have a value of 1
//  Bitcoin have a value of 20 000

impl OracleMockWithQuerier {
    fn unwrap_wallet(&self, address: &str, denom: &str) -> BalanceResponse {
        let raw = self.check_wallet(address, denom).unwrap().unwrap();
        from_binary(&raw).unwrap()
    }
}

#[test]
fn successful_process_stop_loss_order() {
    let env = mock_env();
    let balances: Vec<(&str, Vec<Coin>)> =
        vec![(MOCK_CONTRACT_ADDR, coins(2, "btc")), ("user", vec![])];
    let mut deps = mock_dependencies(&balances);

    let info = mock_info("owner", &[]);

    let prices_at_t0 = vec![coin(30000, "btc"), coin(1, "usdc")];
    let prices_at_t1 = vec![coin(20000, "btc"), coin(1, "usdc")];

    let dummy_order = Order::new(
        OrderType::StopLoss,
        coin(20000, "usdc"),
        coin(2, "btc"),
        Addr::unchecked("user"),
        &vec![],
    );

    let instantiate_msg = InstantiateMsg {
        orders: vec![dummy_order],
    };
    let execute_msg = ExecuteMsg::ProcessOrder {};

    instantiate(
        deps.as_mut(),
        env.clone(),
        info.clone(),
        instantiate_msg.clone(),
    )
    .unwrap();

    deps.querier.update_price(&prices_at_t0);

    let resp = execute(
        deps.as_mut(),
        env.clone(),
        info.clone(),
        execute_msg.clone(),
    )
    .unwrap();

    let refunded_id: Option<u128> = resp.events.iter().find_map(|e| {
        e.attributes
            .iter()
            .find(|attr| {
                attr.key == "refunded_order_id"
                    && attr.value == instantiate_msg.orders[0].order_id.to_string()
            })
            .and_then(|attr| attr.value.parse::<u128>().ok())
    });

    assert!(refunded_id.is_none());

    let contract_balance = deps.querier.unwrap_wallet(MOCK_CONTRACT_ADDR, "btc");

    let user_balance = deps.querier.unwrap_wallet("user", "usdc");

    assert_eq!(contract_balance.amount, coin(2, "btc"));
    assert_eq!(contract_balance.amount, coin(0, "usdc"));
    assert_eq!(user_balance.amount, coin(0, "usdc"));
    assert_eq!(user_balance.amount, coin(0, "btc"));

    deps.querier.update_price(&prices_at_t1);

    let resp = execute(deps.as_mut(), env.clone(), info.clone(), execute_msg).unwrap();

    let contract_balance = deps.querier.unwrap_wallet(MOCK_CONTRACT_ADDR, "btc");

    let user_balance = deps.querier.unwrap_wallet("user", "usdc");

    assert_eq!(contract_balance.amount, coin(0, "btc"));
    assert_eq!(contract_balance.amount, coin(0, "usdc"));
    assert_eq!(user_balance.amount, coin(40000, "usdc"));
    assert_eq!(user_balance.amount, coin(0, "btc"));

    let refunded_id: Option<u128> = resp.events.iter().find_map(|e| {
        e.attributes
            .iter()
            .find(|attr| {
                attr.key == "refunded_order_id"
                    && attr.value == instantiate_msg.orders[0].order_id.to_string()
            })
            .and_then(|attr| attr.value.parse::<u128>().ok())
    });

    assert!(refunded_id.is_some());
}
