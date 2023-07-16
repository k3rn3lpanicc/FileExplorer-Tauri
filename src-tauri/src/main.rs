// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use drives::get_logical_drives;
use drives::{search, recursive_search_entrypoint};
use std::fs;
use tauri::Manager;
use tauri::PhysicalSize;
use std::process::Command;
mod drives;

/*
    * Get a list of logical drives
    * @return: A vector of drive letters
*/
#[tauri::command]
fn get_drives() -> Vec<String> {
    get_logical_drives()
}

#[tauri::command]
fn get_files(input: &str) -> Vec<String> {
  let mut result = Vec::new();
  let paths = fs::read_dir(input).unwrap();
  for path in paths {
    if path.as_ref().unwrap().path().is_file() {
      let path = path.unwrap().path();
      let path = path.to_str().unwrap();
      let file_folder = path.to_string();
      if file_folder.ends_with("\\") {
          continue;
      }
      result.push(file_folder.to_string());
    }
  }
  result
}

#[tauri::command]
fn get_folders(input: &str) -> Result<Vec<String>, String>{
    let mut result = Vec::new();
    let paths = fs::read_dir(input);
    if paths.is_err() {
      return Err("Folder not found".to_string());
    }
    let paths = paths.unwrap();
    for path in paths {
      if path.as_ref().unwrap().path().is_dir() {
        let path = path.unwrap().path();
        let path = path.to_str().unwrap();
        let file_folder = path.to_string();
        if file_folder.ends_with("\\") {
            continue;
        }
        result.push(file_folder.to_string());
      }
    }
    Ok(result)
}

#[tauri::command]
fn open_explorer(path: &str) {
  Command::new("explorer.exe")
        .arg(path)
        .output().unwrap();
}

#[tauri::command]
fn _get_file_size(path: &str) -> u64 {
  let metadata = fs::metadata(path).unwrap();
  let file_size = metadata.len();
  file_size
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();
            main_window.set_title("Explorer").unwrap();
            main_window
                .set_size(tauri::Size::Physical(PhysicalSize::new(1400, 850)))
                .unwrap();
            main_window
                .set_position(tauri::Position::Physical(tauri::PhysicalPosition {
                    x: 260,
                    y: 100,
                }))
                .unwrap();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_drives,
            get_files,
            get_folders,
            open_explorer,
            search,
            recursive_search_entrypoint
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
