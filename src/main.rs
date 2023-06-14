use std::{thread, time};

mod card_getter;
mod util;
mod simple_card;
mod constants;
mod key;
mod tag_updater;
mod markup_updater;
mod card_checker;

use key::Key;
use tag_updater::*;
use markup_updater::*;
use card_checker::*;
use util::*;

#[tokio::main]
async fn main() {
    let key = Key::new();

    loop {
        check_for_due_cards(&key).await;
        thread::sleep(time::Duration::from_secs(60 * /* Minutes: */ 15))
    }
}
