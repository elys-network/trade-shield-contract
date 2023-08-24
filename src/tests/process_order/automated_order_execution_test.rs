use crate::tests::mock::multitest::ElysApp;

use super::*;
use cosmwasm_std::Coin;
use mock::execute::execute as mock_execute;

#[test]
fn automated_order_execution_test() {
    let wallets: Vec<(&str, Vec<Coin>)> = vec![("owner", coins(5, "usdc"))];
    let mut app = ElysApp::new_with_wallets(wallets);

    let code = ContractWrapper::new(mock_execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));

    let instantiate_msg = InstantiateMockMsg {
        epoch_cycle_interval: 1,
        orders: vec![],
    };

    let addr = app
        .instantiate_contract(
            code_id,
            Addr::unchecked("owner"),
            &instantiate_msg,
            &coins(5, "usdc"),
            "Contract",
            None,
        )
        .unwrap();

    assert_eq!(
        app.wrap()
            .query_balance(&addr, "usdc")
            .unwrap()
            .amount
            .u128(),
        5
    );
    assert_eq!(
        app.wrap()
            .query_balance("owner", "usdc")
            .unwrap()
            .amount
            .u128(),
        0
    );

    app.next_block();

    assert_eq!(
        app.wrap()
            .query_balance(&addr, "usdc")
            .unwrap()
            .amount
            .u128(),
        4
    );
    assert_eq!(
        app.wrap()
            .query_balance("owner", "usdc")
            .unwrap()
            .amount
            .u128(),
        1
    );
}
