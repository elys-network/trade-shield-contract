use super::*;

pub fn new_app(list_of_user: &Vec<(String, Vec<Coin>)>) -> App {
    if list_of_user.is_empty() {
        return App::default();
    }

    App::new(|router, _, storage| {
        for (user, token_vec) in list_of_user {
            router
                .bank
                .init_balance(storage, &Addr::unchecked(user), token_vec.to_vec())
                .unwrap();
        }
    })
}
