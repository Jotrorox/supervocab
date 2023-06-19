mod card_checker;
mod card_getter;
mod constants;
mod key;
mod markup_updater;
mod simple_card;
mod tag_updater;
mod util;
mod users;
mod normal_mode;
mod server_mode;
mod mode_selector;

use markup_updater::*;
use tag_updater::*;
use util::*;
use normal_mode::*;
use server_mode::*;
use mode_selector::*;

#[tokio::main]
async fn main() {
    if get_if_server_mode() {
        server_mode().await;
    } else {
        normal_mode().await;
    }
}
