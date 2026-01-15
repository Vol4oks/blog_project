use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::NAME_STORAGE_TOKEN;

#[derive(Debug, Serialize, Deserialize)]
pub struct SaveData {
    pub token: String,
    pub username: String,
    pub uuid: Uuid,
}

impl SaveData {
    pub fn get_brear(&self) -> String {
        format!("Bearer {}", self.token)
    }
}

impl SaveData {
    pub fn new(token: String, username: String, uuid: Uuid) -> Self {
        Self {
            token,
            username,
            uuid,
        }
    }
}

pub fn save_token(data: &SaveData) -> Result<(), String> {
    let window = web_sys::window().ok_or("No window found".to_string())?;
    let storage: web_sys::Storage = window
        .local_storage()
        .map_err(|e| format!("Error connect to local storage: {:?}", e))?
        .ok_or("No local storage found".to_string())?;

    let json_string =
        serde_json::to_string(data).map_err(|e| format!("Error to serialize: {}", e))?;

    storage
        .set_item(NAME_STORAGE_TOKEN, &json_string)
        .map_err(|e| format!("Error save token: {:?}", e))?;
    Ok(())
}

pub fn get_token() -> Result<SaveData, String> {
    let window = web_sys::window().ok_or("No window found".to_string())?;
    let storage: web_sys::Storage = window
        .local_storage()
        .map_err(|e| format!("Error connect to local storage: {:?}", e))?
        .ok_or("No local storage found".to_string())?;

    let value = storage
        .get(NAME_STORAGE_TOKEN)
        .map_err(|e| format!("Error read from local storage: {:?}", e))?
        .ok_or("Token not found".to_string())?;

    let data = serde_json::from_str(&value).map_err(|e| format!("Error to seserialize: {}", e))?;

    Ok(data)
}

pub fn delete_token() -> Result<(), String> {
    let window = web_sys::window().ok_or("No window found".to_string())?;
    let storage: web_sys::Storage = window
        .local_storage()
        .map_err(|e| format!("Error connect to local storage: {:?}", e))?
        .ok_or("No local storage found".to_string())?;

    storage
        .delete(NAME_STORAGE_TOKEN)
        .map_err(|e| format!("Error save token: {:?}", e))?;
    Ok(())
}
