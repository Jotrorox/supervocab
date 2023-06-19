use std::{thread, time};

mod card_checker;
mod card_getter;
mod constants;
mod key;
mod markup_updater;
mod simple_card;
mod tag_updater;
mod util;

use card_checker::*;
use key::Key;
use markup_updater::*;
use tag_updater::*;
use util::*;

#[tokio::main]
async fn main() {
    let key: Key = Key::new();

    loop {
        check_for_due_cards(&key).await;
        thread::sleep(time::Duration::from_secs(60 * /* Minutes: */ 15))
    }
}
