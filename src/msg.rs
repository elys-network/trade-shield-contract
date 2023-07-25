use crate::state::{Order, OrderType};
use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    pub orders : Vec<Order>
}

#[cw_serde]
pub struct GetOrderResp {
    pub order: Order,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetOrderResp)]
    GetOrder { order_id: String },
}

#[cw_serde]
pub enum ExecuteMsg {
    CreateOrder {
        order_type: OrderType,
        stop_price: u128,
        selling_denom: String,
    },
    CancelOrder {
        order_id: String,
    },
    ExecuteOrder {},
}
