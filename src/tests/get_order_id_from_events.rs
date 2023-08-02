use super::*;

pub fn get_order_id_from_events(events: &Vec<Event>) -> Option<u128> {
    let key = "order_id";

    for event in events {
        if let Some(attr) = event.attributes.iter().find(|attr| attr.key == key) {
            if let Ok(id) = attr.value.parse::<u128>() {
                return Some(id);
            }
        }
    }
    None
}
