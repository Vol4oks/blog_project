use std::{
    fs,
    io::{self, Write},
    path::PathBuf,
};

static PATH_TOKEN: &str = ".blog_token";

pub fn get_token_path() -> PathBuf {
    PathBuf::from(PATH_TOKEN)
}

pub fn save_token(token: &str) -> io::Result<()> {
    let token_path = get_token_path();
    let mut file = fs::File::create(&token_path)?;

    write!(file, "{}", token)?;
    println!("Токен сохранен в файл: {:?}", token_path);
    Ok(())
}

pub fn read_token() -> io::Result<String> {
    let token_path = get_token_path();
    fs::read_to_string(&token_path)
}

#[allow(dead_code)]
pub fn delete_token() -> io::Result<()> {
    let token_path = get_token_path();
    if token_path.exists() {
        fs::remove_file(&token_path)?;
        println!("Токен удален");
    }
    Ok(())
}

pub fn has_token() -> bool {
    get_token_path().exists()
}
