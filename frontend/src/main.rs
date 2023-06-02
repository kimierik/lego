use leptos::*;

mod components;
mod utils;

use components::App;

pub fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    mount_to_body(|cx| view! { cx, <App/>  })
}
