use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct InstantiateMsg {
    process_order_executor: String,
}
