use std::{thread, time};
use crate::users::Users;
use crate::key::Key;
use crate::card_checker::*;

pub async fn server_mode() {
    let users: Users = Users::new(vec![]);

    loop {
        for key in &users.keys {
            check_for_due_cards(&key).await;
        }
        thread::sleep(time::Duration::from_secs(60 * /* Time in minutes: */ 15));
    }
}