use dioxus::prelude::*;

use super::Header;
use crate::{dto::Post, infrastructure, Route};
#[component]
pub fn Index() -> Element {
    let posts = use_resource(move || infrastructure::get_list_posts(100, 0));
    let posts_list: Element = match &*posts.read() {
        Some(Ok(posts)) => {
            let posts = posts.post.clone();
            rsx! { PostList { posts } }
        }
        Some(Err(e)) => rsx!("Ошибка: `{e}`"),
        None => rsx!("Загрузка ..."),
    };

    rsx!(
        Header {}
        div {
            {posts_list}
         }
    )
}

#[component]
pub fn PostList(posts: Vec<Post>) -> Element {
    let post_components = posts
        .into_iter()
        .map(|post_data| {
            let create_at = post_data.get_created_at();
            let update_at = post_data.get_update_at();
            rsx!(
                div {
                    class: "post-card",
                    h3 {
                        {post_data.title}
                    }
                    p {
                        {create_at}
                    }
                    if let Some(t) = update_at {
                        p { {t} }
                    }
                    p {
                        {post_data.content}
                    }

                    if let Ok(name) = infrastructure::get_token() {
                        if name.uuid == post_data.author_id {
                                button {
                                    class: "btn",
                                    onclick: move |_| {
                                        navigator().push(Route::PostUpdate {id: post_data.id});
                                    },
                                    "Редактировать"
                                }
                                DeletePost {id: post_data.id}
                        }
                    }
                }
            )
        })
        .collect::<Vec<_>>();

    rsx!(

        if post_components.is_empty() {
            div {
                p {
                    "Постов нет"
                }
             }

        }

        div {
            class: "post-list",
            for post in post_components {
                div { {post} }
            }
        }
    )
}

#[component]
pub fn PostUpdate(id: i64) -> Element {
    let post = use_resource(move || infrastructure::get_post(id));
    let post_data = match &*post.read() {
        Some(Ok(post_data)) => post_data.clone(),
        Some(Err(e)) => {
            return rsx!(
                Header {}
                p { "Ошибка: `{e}`" }
            );
        }
        None => {
            return rsx!(
                Header {}
                p { "Загрузка...." }
            );
        }
    };

    let mut title_post = use_signal(|| post_data.title.clone());
    let mut content_post = use_signal(|| post_data.content.clone());

    let mut error = use_signal(|| String::new());
    let mut is_loading = use_signal(|| false);

    let nav = navigator();
    let user = match infrastructure::get_token() {
        Ok(n) => n,
        Err(e) => {
            return rsx!("Ошибка: `{e}`");
        }
    };

    let on_submit = move |event: FormEvent| {
        event.stop_propagation();
        event.prevent_default();

        if title_post.read().len() == 0 {
            error.set("Название обязательно".to_string());
            return;
        }

        if content_post.read().len() == 0 {
            error.set("Содержание обязательно".to_string());
            return;
        }

        is_loading.set(true);

        let title_post_clone = title_post.read().clone();
        let content_post_clone = content_post.read().clone();
        let mut error_clone = error.clone();
        let nav_clone = nav.clone();
        let token_clone = user.get_brear();
        let post_id = post_data.id;

        spawn(async move {
            match infrastructure::update_post(
                post_id,
                &title_post_clone,
                &content_post_clone,
                &token_clone,
            )
            .await
            {
                Ok(_) => {
                    nav_clone.push(Route::Index);
                }
                Err(e) => {
                    error_clone.set(format!("Ошибка обновления поста: {}", e));
                    is_loading.set(false);
                }
            }
        });
    };

    rsx!(
        Header {  }

        div {
            class: "form-container",
            div {
                h2 { "Обновить пост" }
            }
            form {
                onsubmit: on_submit,
                div {
                    label {"Название"}
                    input {
                        r#type: "text",
                        value: "{title_post}",
                        placeholder: "Введите название",
                        oninput: move |e| title_post.set(e.value())
                    }
                }

                div {
                    label {"Содержимое"}
                    textarea {
                        value: "{content_post}",
                        placeholder: "Введите текст",
                        rows: "15",
                        oninput: move |e| content_post.set(e.value())
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
                        class: "btn",
                        r#type: "submit",
                        disabled: *is_loading.read(),

                        if *is_loading.read(){
                            "Обновление..."
                        } else {
                            "Обновить"
                        }
                    }
                }
            }
        }
    )
}

#[component]
pub fn CreatePost() -> Element {
    let mut title_post = use_signal(String::new);
    let mut content_post = use_signal(String::new);
    let mut error = use_signal(|| String::new());
    let mut is_loading = use_signal(|| false);

    let nav = navigator();
    let name = match infrastructure::get_token() {
        Ok(n) => n,
        Err(e) => {
            return rsx!("Ошибка: `{e}`");
        }
    };

    let on_submit = move |event: FormEvent| {
        event.stop_propagation();
        event.prevent_default();

        if title_post.read().len() == 0 {
            error.set("Название обязательно".to_string());
            return;
        }

        if content_post.read().len() == 0 {
            error.set("Содержание обязательно".to_string());
            return;
        }

        is_loading.set(true);

        let title_post_clone = title_post.read().clone();
        let content_post_clone = content_post.read().clone();
        let mut error_clone = error.clone();
        let nav_clone = nav.clone();
        let token_clone = name.get_brear();

        spawn(async move {
            match infrastructure::create_post(&title_post_clone, &content_post_clone, &token_clone)
                .await
            {
                Ok(_) => {
                    nav_clone.push(Route::Index);
                }
                Err(e) => {
                    error_clone.set(format!("Ошибка создании поста: {}", e));
                    is_loading.set(false);
                }
            }
        });
    };

    rsx!(
        Header {  }
        div {
            class: "form-container",
            div {
                h2 { "Создать пост" }
            }
            form {
                onsubmit: on_submit,
                div {
                    label {"Название"}
                    input {
                        r#type: "text",
                        value: "{title_post}",
                        placeholder: "Введите название",
                        oninput: move |e| title_post.set(e.value())
                    }
                }

                div {
                    label {"Содержимое"}
                    textarea {
                        value: "{content_post}",
                        placeholder: "Введите текст",
                        rows: "15",
                        oninput: move |e| content_post.set(e.value())
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
                        class: "btn",
                        r#type: "submit",
                        disabled: *is_loading.read(),

                        if *is_loading.read(){
                            "Создание..."
                        } else {
                            "Создать"
                        }
                    }
                }
            }
        }
    )
}

#[component]
pub fn DeletePost(id: i64) -> Element {
    let mut error = use_signal(|| String::new());
    let mut is_loading = use_signal(|| false);

    let user = match infrastructure::get_token() {
        Ok(n) => n,
        Err(e) => {
            return rsx!("Ошибка: `{e}`");
        }
    };

    let on_click = move |_| {
        error.set(String::new());
        is_loading.set(true);

        let id_clone = id;
        let mut error_clone = error.clone();
        let token = user.get_brear();

        spawn(async move {
            match infrastructure::delete_post(id_clone, &token).await {
                Ok(_) => {
                    if let Some(window) = web_sys::window() {
                        let _ = window.location().reload();
                    }
                }
                Err(e) => {
                    error_clone.set(format!("Ошибка удалении: {}", e));
                    is_loading.set(false);
                }
            }
        });
    };

    rsx!(
        button {
            class: "btn",
            onclick: on_click,
            disabled: *is_loading.read(),
            "Удалить"
        }
    )
}
