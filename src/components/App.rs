use dioxus::prelude::*;

use crate::components::Game;

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Route {
    #[route("/")]
    Game,
}

pub fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}
