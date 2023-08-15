use crate::tests::mock::multitest::ElysApp;

use super::*;

#[test]
fn coin_number() {
    let mut app = ElysApp::new();

    let instantiate_msg = InstantiateMsg { orders: vec![] };

    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));
    let addr = app
        .instantiate_contract(
            code_id,
            Addr::unchecked("owner"),
            &instantiate_msg,
            &[],
            "Contract",
            None,
        )
        .unwrap();
    let err = app
        .execute_contract(
            Addr::unchecked("user"),
            addr,
            &ExecuteMsg::CreateOrder {
                order_type: OrderType::StopLoss,
                order_price: coin(100, "eth"),
            },
            &[],
        )
        .unwrap_err();

    assert_eq!(ContractError::CoinNumber, err.downcast().unwrap());
}
