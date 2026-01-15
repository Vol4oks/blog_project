use dioxus::prelude::*;

use super::Header;

use crate::{
    infrastructure::{self, SaveData},
    Route,
};

#[component]
pub fn Register() -> Element {
    let mut username = use_signal(|| String::new());
    let mut email = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    let mut error = use_signal(|| String::new());
    let mut is_loading = use_signal(|| false);

    let nav = navigator();

    let on_submit = move |event: FormEvent| {
        event.stop_propagation();
        event.prevent_default();
        error.set(String::new());

        if username.read().is_empty() {
            error.set("Имя пользователя обязательно".to_string());
        }

        if email.read().is_empty() {
            error.set("Почта обязательна".to_string());
        }

        if password.read().is_empty() {
            error.set("пароль обязателен".to_string());
        }

        is_loading.set(true);

        let username_clone = username.read().clone();
        let email_clone = email.read().clone();
        let password_clone = password.read().clone();
        let mut error_clone = error.clone();
        let nav_clone = nav.clone();

        spawn(async move {
            match infrastructure::register_user(&username_clone, &email_clone, &password_clone)
                .await
            {
                Ok(auth_responce) => {
                    if let Err(e) = infrastructure::save_token(&SaveData::new(
                        auth_responce.token,
                        username_clone,
                        auth_responce.uuid,
                    )) {
                        error_clone.set(e);
                        is_loading.set(false);
                        return;
                    }
                    nav_clone.push(Route::Index);
                }
                Err(e) => {
                    error_clone.set(format!("Ошибка регистрации: {}", e));
                    is_loading.set(false);
                }
            }
        });
    };

    rsx!(
        Header {}
        div{
            class: "form-container",
            div {
                h2 { "Регистрация" }
            }
            form {
                onsubmit: on_submit,

                div {
                    label { "Имя пользователя"}
                    input {
                        r#type: "text",
                        value: "{username}",
                        placeholder: "Введите имя пользователя",
                        oninput: move |e| username.set(e.value())
                    }

                }

                div {
                    label { "Почта"}
                    input {
                        r#type: "email",
                        value: "{email}",
                        placeholder: "Введите почту",
                        oninput: move |e| email.set(e.value())
                    }
                }

                div {
                    label { "Пароль"}
                    input {
                        r#type: "password",
                        value: "{password}",
                        placeholder: "Введите Пароль",
                        oninput: move |e| password.set(e.value())
                    }
                }
                if !error.read().is_empty() {
                    p {
                        class: "error-message",
                        "{error}"
                    }
                }

                div{
                    button {
                        r#type: "submit",
                        disabled: *is_loading.read(),

                        if *is_loading.read(){
                            "Регистрация..."
                        } else {
                            "Зарегистрироваться"
                        }
                    }
                }
            }
        }
    )
}

#[component]
pub fn Login() -> Element {
    let mut username = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    let mut error = use_signal(|| String::new());
    let mut is_loading = use_signal(|| false);

    let nav = navigator();

    let on_submit = move |event: FormEvent| {
        event.stop_propagation();
        event.prevent_default();
        error.set(String::new());

        if username.read().is_empty() {
            error.set("Имя пользователя обязательно".to_string());
        }

        if password.read().is_empty() {
            error.set("пароль обязателен".to_string());
        }

        is_loading.set(true);

        let username_clone = username.read().clone();
        let password_clone = password.read().clone();
        let mut error_clone = error.clone();
        let nav_clone = nav.clone();

        spawn(async move {
            match infrastructure::login_user(&username_clone, &password_clone).await {
                Ok(auth_responce) => {
                    if let Err(e) = infrastructure::save_token(&SaveData::new(
                        auth_responce.token,
                        username_clone,
                        auth_responce.uuid,
                    )) {
                        error_clone.set(e);
                        is_loading.set(false);
                        return;
                    }
                    nav_clone.push(Route::Index);
                }
                Err(e) => {
                    error_clone.set(format!("Ошибка входа: {}", e));
                    is_loading.set(false);
                }
            }
        });
    };

    rsx!(
        Header {}
        div {
            class: "form-container",
            div{
                h2 { "Вход" }
            }
            form {
                onsubmit: on_submit,

                div {
                    label { "Имя пользователя"}
                    input {
                        r#type: "text",
                        value: "{username}",
                        placeholder: "Введите имя пользователя",
                        oninput: move |e| username.set(e.value())
                    }
                }

                div {
                    label { "Пароль"}
                    input {
                        r#type: "password",
                        value: "{password}",
                        placeholder: "Введите Пароль",
                        oninput: move |e| password.set(e.value())
                    }
                }
                if !error.read().is_empty() {
                    p {
                        class: "error-message",
                        "{error}"
                    }
                }

                div {
                    button {
                        r#type: "submit",
                        disabled: *is_loading.read(),

                        if *is_loading.read(){
                            "Вход..."
                        } else {
                            "Войти"
                        }
                    }
                }
            }
        }
    )
}
