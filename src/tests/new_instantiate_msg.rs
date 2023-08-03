use super::*;

impl InstantiateMsg {
    pub fn new(list_order_info: Vec<(&str, Coin, Coin, OrderType)>) -> InstantiateMsg {
        let mut order_id: u128 = 0;
        let mut list_of_order: Vec<Order> = vec![];

        if list_order_info.is_empty() {
            return InstantiateMsg {
                orders: list_of_order,
            };
        }

        for (owner_address, order_price, order_amount, order_type) in list_order_info {
            let owner_address = Addr::unchecked(owner_address);
            let order = Order {
                order_type,
                order_id,
                order_price,
                order_amount,
                owner_address,
            };
            order_id += 1;
            list_of_order.push(order);
        }

        InstantiateMsg {
            orders: list_of_order,
        }
    }
}
