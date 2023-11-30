use super::*;

pub fn get_order_id_from_events(events: &Vec<Event>) -> Option<u64> {
    let key = "order_id";

    for event in events {
        if let Some(attr) = event.attributes.iter().find(|attr| attr.key == key) {
            if let Ok(id) = attr.value.parse::<u64>() {
                return Some(id);
            }
        }
    }
    None
}

pub fn get_attr_from_events(events: &Vec<Event>, key: &str) -> Option<String> {
    for event in events {
        if let Some(attr) = event.attributes.iter().find(|attr| attr.key == key) {
            return Some(attr.value.clone());
        }
    }
    None
}
