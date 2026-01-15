use dioxus::{logger::tracing, prelude::*};

use crate::{infrastructure, Route};

#[component]
pub fn Header() -> Element {
    let nav = navigator();
    let mut username = "NoName".to_string();
    let auth_head: Element = {
        if let Ok(name) = infrastructure::get_token() {
            username = name.username;
            rsx!(Auth {})
        } else {
            rsx!(NoAuth {})
        }
    };

    rsx!(
        div{
            h1 {
                {username}
            }
        }
        div {
            class: "header",
            button {
                class: "auth-btn",
                onclick: move |_| { nav.push(Route::Index);},
                "Главная"
            }
            {auth_head}
        }

    )
}

#[component]
pub fn NoAuth() -> Element {
    let nav = navigator();

    rsx!(
        button {
            class: "auth-btn",
            onclick: move |_| { nav.push(Route::Login);},
            "Вход"
        }
        button {
            class: "auth-btn",
            onclick: move |_| { nav.push(Route::Register);},
            "Регистраци"
        }
    )
}

#[component]
pub fn Auth() -> Element {
    let nav = navigator();
    rsx!(
        button {
            class: "auth-btn",
            onclick: move |_| { nav.push(Route::CreatePost);},
            "Создать пост"
        }
        ExitButton {  }
    )
}

#[component]
pub fn ExitButton() -> Element {
    let nav = navigator();
    let current_route = use_route::<Route>();

    rsx!(button {
        class: "logout-btn",
        onclick: move |_| {
            if let Err(e) = infrastructure::delete_token(){
                tracing::error!("Ошибка при удалении токена: {e}");
            };
            if current_route == Route::Index{
                if let Some(window) = web_sys::window() {
                    let _ = window.location().reload();
                }
            }else {
                nav.push(Route::Index);
            }
        },
        "Выход"
    })
}

#[component]
pub fn NotFound(route: Vec<String>) -> Element {
    rsx! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre { color: "red", "log:\nattempted to navigate to: {route:?}" }
    }
}
