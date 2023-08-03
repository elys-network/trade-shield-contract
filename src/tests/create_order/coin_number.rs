use super::*;

#[test]
fn coin_number() {
    let mut app = App::default();

    let instantiate_msg = InstantiateMsg::new(vec![]);

    let addr: Addr = new_contract_addr(&mut app, &instantiate_msg, &vec![]);

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
