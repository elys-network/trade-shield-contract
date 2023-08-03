use super::*;

pub fn new_contract_addr(
    app: &mut App,
    msg: &InstantiateMsg,
    list_of_user: &Vec<(String, Vec<Coin>)>,
) -> Addr {
    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));

    let empty: Vec<Coin> = vec![];

    let fund: &Vec<Coin> = match list_of_user
        .iter()
        .find(|(user, _)| user == "owner")
        .map(|(_, tokens)| tokens)
    {
        Some(tokens) => tokens,
        None => &empty,
    };

    app.instantiate_contract(
        code_id,
        Addr::unchecked("owner"),
        msg,
        fund,
        "contract",
        None,
    )
    .unwrap()
}
