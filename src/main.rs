mod app;
mod components;
mod drive;
mod models;
mod state;
mod storage;
mod utils;

use app::App;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
