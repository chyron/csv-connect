#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod settings;

use tauri::api::{dialog::blocking::FileDialogBuilder, path::local_data_dir};
use std::sync::Mutex;
use std::path::Path;
use std::fs;
use std::str;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Copy, Default)]
pub struct AppSettings {
    delimiter: u8
}

impl AppSettings {
    fn set_delimiter(&mut self) {
        self.delimiter = b',';
    }
}

#[derive(Debug)]
pub struct AppState {
    first: Mutex<Vec<Vec<String>>>,
    second: Mutex<Vec<Vec<String>>>,
    connect: Mutex<Vec<Vec<String>>>,
    settings: Mutex<AppSettings>,
}

#[tauri::command]
async fn read_file(file_number: u8, state: tauri::State<'_, AppState>) -> Result<Vec<Vec<String>>, ()> {
    let file_path = FileDialogBuilder::new().pick_file();
    let delimiter = state.settings.lock().unwrap().delimiter;

    if let Some(file_path) = file_path {
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .delimiter(delimiter)
            .from_path(file_path).unwrap();

        let mut rows = Vec::new();

        for result in rdr.records() {
            let record = result.unwrap();
            let mut fields = Vec::new();
            for field in record.iter() {
                fields.push(String::from(field));
            }

            rows.push(fields);

        }

        if file_number == 0 {
            let mut first = state.first.lock().unwrap();
            *first = rows;
        } else if file_number == 1 {
            let mut second = state.second.lock().unwrap();
            *second = rows;
        }
    }

    Ok(state.first.lock().unwrap().to_vec())
}

#[tauri::command]
async fn connect(first_index: usize, second_index: usize, state: tauri::State<'_, AppState>) -> Result<Vec<Vec<String>>, ()> {
    let first = state.first.lock().unwrap();
    let second = state.second.lock().unwrap();
    let mut result: Vec<Vec<String>> = Vec::new();
    for row in first.iter() {
        let first_id = &row[first_index];
        for r in second.iter() {
            let second_id = &r[second_index];
            if first_id == second_id {
                let mut new = r.to_vec();
                let mut new2 = row.to_vec();
                new2.append(&mut new);
                result.push(new2);
            }
        }
    }

    let mut connect = state.connect.lock().unwrap();
    *connect = result;

    Ok(connect.to_vec())
}

#[tauri::command]
async fn remove_column(column_index: usize, state: tauri::State<'_, AppState>) -> Result<Vec<Vec<String>>, ()> {
    let mut result: Vec<Vec<String>> = Vec::new();
    let mut connect = state.connect.lock().unwrap();

    for row in connect.iter() {
        let mut new = row.to_vec();
        new.remove(column_index);
        result.push(new);
    }

    *connect = result;

    Ok(connect.to_vec())
}

#[tauri::command]
async fn save(state: tauri::State<'_, AppState>) -> Result<(), ()> {
    let file_path = FileDialogBuilder::new().save_file();

    let connect = state.connect.lock().unwrap();
    let delimiter = state.settings.lock().unwrap().delimiter;

    if let Some(file_path) = file_path {
        let mut wtr = csv::WriterBuilder::new()
            .delimiter(delimiter)
            .from_path(file_path).unwrap();

        for row in connect.iter() {
            wtr.write_record(row).unwrap();
        }

        wtr.flush().unwrap();
    }

    Ok(())
}

fn main() {
    let storage_dir = Path::new(&local_data_dir().unwrap()).join("csv-connect");
    let data: String;
    match fs::read(storage_dir.join("settings")) {
        Ok(result) => match bincode::deserialize(&result) {
            Ok(deserialized_bincode) => data = deserialized_bincode,
            Err(_) => data = str::from_utf8(&result).unwrap().to_string(),
        },
        Err(e) => {
            data = e.to_string();
        }
    }

    let serde_value: Result<AppSettings, serde_json::Error> = serde_json::from_str(&data);
    let settings = match serde_value {
        Ok(result) => result,
        Err(_) => {
            AppSettings {
                delimiter: b';'
            }
        }
    };

  tauri::Builder::default()
    .manage(AppState {
        first: Default::default(),
        second: Default::default(),
        connect: Default::default(),
        settings: Mutex::new(settings),
    })
    .invoke_handler(tauri::generate_handler![read_file, connect, remove_column, save, settings::read_data, settings::write_data, settings::get_settings, settings::set_settings])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
