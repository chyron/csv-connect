use tauri::api::path::local_data_dir;
use std::path::Path;
use std::fs;
use std::str;

use crate::AppSettings;
use crate::AppState;

#[tauri::command]
pub fn get_settings(state: tauri::State<'_, AppState>) -> Result<AppSettings, String> {
    let settings = state.settings.lock().unwrap();
    Ok(*settings)
}

#[tauri::command]
pub fn set_settings(state: tauri::State<'_, AppState>) -> Result<AppSettings, String> {
    let mut settings = state.settings.lock().unwrap();
    settings.set_delimiter();
    Ok(*settings)
}


#[tauri::command]
pub fn write_data(key: &str, value: serde_json::Value) {
    let storage_dir = Path::new(&local_data_dir().unwrap()).join("csv-connect");

    if let Err(e) = fs::create_dir_all(&storage_dir) {
        eprintln!("Failed to create dirs: {:?}", e);
    }
    let value = bincode::serialize(&serde_json::to_vec(&value).unwrap()).unwrap();

    if let Err(e) = fs::write(storage_dir.join(key), value) {
        eprintln!("Failed to write data {:?}", e);
    }
}

#[tauri::command]
pub fn read_data(key: &str) -> Result<serde_json::Value, String> {
    let storage_dir = Path::new(&local_data_dir().unwrap()).join("csv-connect");
    let data: String;
    match fs::read(storage_dir.join(key)) {
        Ok(result) => match bincode::deserialize(&result) {
            Ok(deserialized_bincode) => data = deserialized_bincode,
            Err(_) => data = str::from_utf8(&result).unwrap().to_string(),
        },
        Err(e) => {
            data = e.to_string();
        }
    }

    let serde_value: Result<serde_json::Value, serde_json::Error> = serde_json::from_str(&data);
    let data = match serde_value {
        Ok(result) => result,
        Err(_) => {
            serde_json::Value::Null
        }
    };

    Ok(data)
}
