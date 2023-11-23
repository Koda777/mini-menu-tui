extern crate ncurses;
use std::env;
use ncurses::*;
use crate::parcing::position;
use crate::utils::{remove, path};
use crate::constants::data_cursor::DATA_CURSOR;
use crate::constants::data_menu::DATA_MENU;
use crate::utils::remove::{remove_cursor, remove_menu};
use crate::utils::sort::sort_files;

fn is_valid_horizontal(new_y: i32) -> bool {
    if (new_y >= 0) && (new_y < DATA_MENU.get_value().len() as i32) {
        return true;
    }
    return false;
}

fn is_dir(file: &str) -> bool {
    let metadata = std::fs::metadata(file).unwrap();
    return metadata.is_dir();
}
pub fn get_input() -> i32 {
    let ch = getch();

    match ch {
        107 => {
            let y = DATA_CURSOR.get_position();
            if is_valid_horizontal(y  - 1) {
                remove::remove_cursor();
                DATA_CURSOR.set_position(y - 1);
            }
        }
        106 => {
            let y = DATA_CURSOR.get_position();
            if is_valid_horizontal(y  + 1) {
                remove::remove_cursor();
                DATA_CURSOR.set_position(y + 1);
            }
        }
        104 => {
            remove_menu();
            remove_cursor();
            env::set_current_dir("..").unwrap();
            let mut vec: Vec<String> = path::get_current_dir();
            sort_files(&mut vec);
            let box_coordinates = position::center_by_list(&vec);
            DATA_CURSOR.set_position(0);
            DATA_MENU.set_position(box_coordinates.clone());
            DATA_MENU.set_value(vec);
        }
        108 => {
            if is_dir(&DATA_MENU.get_value()[DATA_CURSOR.get_position() as usize]) {
                remove_menu();
                remove_cursor();
                let dir = path::get_dir_from_position();
                let new_path = env::current_dir().unwrap().join(dir);
                env::set_current_dir(new_path).unwrap();
                let mut vec: Vec<String> = path::get_current_dir();
                sort_files(&mut vec);
                let box_coordinates = position::center_by_list(&vec);
                DATA_CURSOR.set_position(0);
                DATA_MENU.set_position(box_coordinates.clone());
                DATA_MENU.set_value(vec.clone());
            }
        }

        113 => {
            endwin();
            std::process::exit(0);
        }
        _ => {
        }
    }
    return ch;
}