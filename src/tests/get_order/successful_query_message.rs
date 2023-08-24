use crate::tests::mock::multitest::ElysApp;

use super::*;
use query_resp::GetOrderResp;

#[test]
fn successful_query_message() {
    let mut app = ElysApp::new();

    let instantiate_msg = InstantiateMockMsg {
        epoch_cycle_interval: 2,
        orders: vec![Order::new_dummy()],
    };
    let id = instantiate_msg.orders[0].order_id.clone().to_owned();

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

    let resp: GetOrderResp = app
        .wrap()
        .query_wasm_smart(&addr, &QueryMsg::GetOrder { order_id: id })
        .unwrap();

    assert_eq!(
        resp,
        GetOrderResp {
            order: Order::new_dummy(),
        }
    );
}
