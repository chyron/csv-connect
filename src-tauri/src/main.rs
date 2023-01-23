#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::api::dialog::blocking::FileDialogBuilder;
use std::sync::Mutex;

#[derive(Debug)]
pub struct AppState {
    first: Mutex<Vec<Vec<String>>>,
    second: Mutex<Vec<Vec<String>>>,
    connect: Mutex<Vec<Vec<String>>>,
}

#[tauri::command]
async fn read_file(file_number: u8, delimiter: char, state: tauri::State<'_, AppState>) -> Result<Vec<Vec<String>>, ()> {
    let file_path = FileDialogBuilder::new().pick_file();

    if let Some(file_path) = file_path {
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .delimiter(delimiter as u8)
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

    Ok(if file_number == 0 { state.first.lock().unwrap().to_vec() } else { state.second.lock().unwrap().to_vec() })
}

#[tauri::command]
async fn connect(first_index: usize, second_index: usize, state: tauri::State<'_, AppState>) -> Result<Vec<Vec<String>>, ()> {
    let first = state.first.lock().unwrap();
    let second = state.second.lock().unwrap();
    let mut result: Vec<Vec<String>> = Vec::new();
    'outer: for row_in_first in first.iter() {
        let first_id = &row_in_first[first_index];
        for row_in_second in second.iter() {
            let second_id = &row_in_second[second_index];
            if first_id == second_id {
                let mut connected_row = row_in_first.to_vec();
                connected_row.append(&mut row_in_second.to_vec());
                result.push(connected_row);
                continue 'outer
            }
        }
        let mut first_row = row_in_first.to_vec();
        first_row.resize(first[0].len() + second[0].len(), String::from(""));
        result.push(first_row);
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
async fn save(delimiter: char, state: tauri::State<'_, AppState>) -> Result<(), ()> {
    let file_path = FileDialogBuilder::new().save_file();

    let connect = state.connect.lock().unwrap();

    if let Some(file_path) = file_path {
        let mut wtr = csv::WriterBuilder::new()
            .delimiter(delimiter as u8)
            .from_path(file_path).unwrap();

        for row in connect.iter() {
            wtr.write_record(row).unwrap();
        }

        wtr.flush().unwrap();
    }

    Ok(())
}

fn main() {
  tauri::Builder::default()
    .manage(AppState {
        first: Default::default(),
        second: Default::default(),
        connect: Default::default(),
    })
    .invoke_handler(tauri::generate_handler![read_file, connect, remove_column, save])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
