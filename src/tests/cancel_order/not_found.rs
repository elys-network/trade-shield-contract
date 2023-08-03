use super::*;

#[test]
fn not_found() {
    let mut app = App::default();

    let instantiate_msg = InstantiateMsg::new(vec![]);
    let id: u128 = 0;

    let addr: Addr = new_contract_addr(&mut app, &instantiate_msg, &vec![]);

    let err = app
        .execute_contract(
            Addr::unchecked("user"),
            addr,
            &ExecuteMsg::CancelOrder { order_id: id },
            &[],
        )
        .unwrap_err();

    assert_eq!(
        ContractError::OrderNotFound { order_id: id },
        err.downcast().unwrap()
    );
}
