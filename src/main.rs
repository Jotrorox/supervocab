use std::{thread, time};

mod card_checker;
mod card_getter;
mod constants;
mod key;
mod markup_updater;
mod simple_card;
mod tag_updater;
mod users;
mod util;

use markup_updater::*;
use tag_updater::*;
use util::*;

#[tokio::main]
async fn main() {
    let users: Users = Users::new(vec![Key::new()]);

    loop {
        for key in &users.keys {
            check_for_due_cards(&key).await;
        }
        thread::sleep(time::Duration::from_secs(60 * /* Time in minutes: */ 15));
    }
}
