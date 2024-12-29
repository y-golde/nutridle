#![allow(non_snake_case)]

use components::App;
use dioxus::prelude::*;
use dioxus_logger::tracing;

mod components;
mod server;
mod structs;

fn main() {
    #[cfg(not(target_arch = "wasm32"))] // Hack to only run this on the server - idk
    server::init();

    println!("Server initialized");
    // Init logger
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    tracing::info!("starting app");
    launch(App);
}
