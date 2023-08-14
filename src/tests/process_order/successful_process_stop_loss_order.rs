use crate::tests::mock::mock_dependencies;

use super::*;
use cosmwasm_std::{
    coins,
    testing::{mock_env, mock_info, MOCK_CONTRACT_ADDR},
    Coin,
};

#[test]
fn successful_process_stop_loss_order() {
    let wallets = vec![("owner", coins(2, "btc")), ("user", vec![])];
    let dummy_order = Order::new(
        OrderType::StopLoss,
        coin(20000, "usdc"),
        coin(2, "btc"),
        Addr::unchecked("user"),
        &vec![],
    );
    let instantiate_msg = InstantiateMsg {
        orders: vec![dummy_order.clone()],
    };
    let prices: Vec<Coin> = vec![coin(20000, "btc"), coin(1, "usdc")];
    let execute_msg = ExecuteMsg::ProcessOrder {};
    let mut deps = mock_dependencies(wallets);
    let env = mock_env();
    let info = mock_info(MOCK_CONTRACT_ADDR, &[]);
    deps.querier
        .init_modules(|router, _, storage| router.custom.set_prices(storage, &prices))
        .unwrap();

    //need implementation of the Elys Querry in production to be used by the endpoint
}
