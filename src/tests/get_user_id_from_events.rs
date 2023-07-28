use super::*;

pub fn get_user_id_from_events(events: &Vec<Event>, key: &str) -> Option<u128> {
    for event in events {
        if let Some(attr) = event.attributes.iter().find(|attr| attr.key == key) {
            if let Ok(id) = attr.value.parse::<u128>() {
                return Some(id);
            }
        }
    }
    None
}