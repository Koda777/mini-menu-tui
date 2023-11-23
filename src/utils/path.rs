use crate::constants::data_cursor::DATA_CURSOR;
use crate::constants::data_menu::DATA_MENU;
use std::{env, fs};

fn get_dir(path: String) -> Vec<String> {
    let mut current_files: Vec<String> = Vec::new();

    match fs::read_dir(".") {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Some(file_name) = entry.file_name().to_str() {
                        if !file_name.starts_with(".") {
                            current_files.push(file_name.to_string());
                        }
                    }
                }
            }
        }
        Err(err) => {
            eprintln!("Error reading directory: {}", err);
        }
    }
    current_files
}

pub fn get_current_dir() -> Vec<String>{
    match env::current_dir() {
        Ok(path) => {
            get_dir(path.to_str().unwrap().to_string())

        },
        Err(e) => {
            println!("Error getting current directory: {}", e);
            vec![]
        },
    }
}

pub fn get_dir_from_position() -> String {
    let position = DATA_CURSOR.get_position() as usize;
    let vec = DATA_MENU.get_value();
    let dir = vec[position].clone();
    dir
}