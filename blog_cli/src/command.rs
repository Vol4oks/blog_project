use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "Blog Cli", version, about, long_about = None)]
pub struct Cli {
    /// Включить gRPC режим
    #[arg(long, global = true)]
    pub grpc: bool,

    /// Адрес сервера
    #[arg(long, global = true, required = false, default_value = "127.0.0.1")]
    pub server: String,

    /// Порт сервера
    #[arg(long, global = true, required = false, default_value = "8081")]
    pub port: String,

    /// Команда для выполнения
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Регистрация нового пользователя
    Register(RegisterArgs),

    /// Вход в систему
    Login(LoginArgs),

    /// Создание нового ресурса
    Create(CreateArgs),

    /// Получение ресурса по ID
    Get(GetArgs),

    /// Обновление ресурса
    Update(UpdateArgs),

    /// Удаление ресурса
    Delete(DeleteArgs),

    /// Список ресурсов
    List(ListArgs),
}

#[derive(Args, Debug)]
pub struct RegisterArgs {
    /// Имя пользователя
    #[arg(long)]
    pub username: String,

    /// Email адрес
    #[arg(long)]
    pub email: String,

    /// Пароль
    #[arg(long)]
    pub password: String,
}

#[derive(Args, Debug)]
pub struct LoginArgs {
    /// Имя пользователя
    #[arg(long)]
    pub username: String,

    /// Пароль
    #[arg(long)]
    pub password: String,
}

#[derive(Args, Debug)]
pub struct CreateArgs {
    /// Заголовок
    #[arg(long)]
    pub title: String,

    /// Содержимое
    #[arg(long)]
    pub content: String,
}

#[derive(Args, Debug)]
pub struct GetArgs {
    /// ID ресурса
    #[arg(long)]
    pub id: i64,
}

#[derive(Args, Debug)]
pub struct UpdateArgs {
    /// ID ресурса для обновления
    #[arg(long)]
    pub id: i64,

    /// Новый заголовок
    #[arg(long)]
    pub title: Option<String>,

    /// Новое содержимое
    #[arg(long)]
    pub content: Option<String>,
}

#[derive(Args, Debug)]
pub struct DeleteArgs {
    /// ID ресурса для удаления
    #[arg(long)]
    pub id: i64,
}

#[derive(Args, Debug)]
pub struct ListArgs {
    /// Максимальное количество элементов
    #[arg(long, default_value_t = 10)]
    pub limit: i32,

    /// Смещение (сколько элементов пропустить)
    #[arg(long, default_value_t = 0)]
    pub offset: i32,
}
