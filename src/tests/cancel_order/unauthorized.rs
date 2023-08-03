use super::*;

#[test]
fn unauthorized() {
    let mut app = App::default();

    let instantiate_msg = InstantiateMsg {
        orders: vec![Order::new_dummy()],
    };

    let id = instantiate_msg.orders[0].order_id.clone().to_owned();

    let addr = new_contract_addr(&mut app, &instantiate_msg, &vec![]);

    let err = app
        .execute_contract(
            Addr::unchecked("not_user"),
            addr,
            &ExecuteMsg::CancelOrder {
                order_id: id.clone(),
            },
            &[],
        )
        .unwrap_err();

    assert_eq!(
        ContractError::Unauthorized {
            sender: Addr::unchecked("not_user")
        },
        err.downcast().unwrap()
    );
}
