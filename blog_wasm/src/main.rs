mod components;
mod dto;
mod infrastructure;

use components::*;
use dioxus::prelude::*;

const API_PATH: &str = "http://127.0.0.1:8081";
const NAME_STORAGE_TOKEN: &str = "auth_token";

const MAIN_CSS: Asset = asset!("/assets/main.css");

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Index,

    #[route("/post/:id")]
    PostUpdate { id: i64 },
    #[route("/login")]
    Login,
    #[route("/register")]
    Register,
    #[route("/create")]
    CreatePost,
    #[route("/:..route")]
    NotFound { route: Vec<String> },
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        div {
            class: "app",
            main {
                Router::<Route> {},
            }
        }

    }
}
