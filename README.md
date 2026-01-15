# blog_project
Перед запуском переименуйте файл `.env.pub` в  `.env` и заполните 
```
DATABASE_URL= путь до базы данных 
JWT_SECRET= ключ для токенов
```

Для корректной работы стоит оставить порт 8081 для сервера. Он прописан как дефолтный для `blog-cli` и `blog-wasm`.

Команда для запуска:
```bash
cargo run --bin blog-server
```

#
### blog-wasm
Для запуска нужно установить [dioxus](https://github.com/DioxusLabs/dioxus?tab=readme-ov-file), и запустить командой:

```bash
dx serve --package blog-wasm
```

#
### blog-cli
Команда для запуска:
```bash
cargo run --bin blog-cli -- --help
```
