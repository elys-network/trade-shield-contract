use cw_multi_test::AppResponse;
use serde_json::Value;

pub fn read_processed_order_id(resp: AppResponse) -> Vec<u64> {
    for event in resp.events {
        if let Some(attr) = event
            .attributes
            .iter()
            .find(|attr| attr.key == "spot_order_processed")
        {
            let value: Value = serde_json::from_str(&attr.value).unwrap();
            return serde_json::from_value(value).unwrap();
        }
    }
    return vec![];
}
