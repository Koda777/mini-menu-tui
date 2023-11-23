extern crate ncurses;
use std::fs::metadata;

use ncurses::*;
pub fn display_folder(str: &str, coordinates: (i32, i32)) -> () {
    mvprintw(coordinates.1, coordinates.0, format!("{}{}", str, "/").as_str());
}

pub fn display_menu(vec: &Vec<String>, box_coordinates: &Vec<(i32, i32)>) {
    for (index, string_ref) in vec.iter().enumerate() {
        let (x, y) = <Vec<(i32, i32)> as AsRef<Vec<(i32, i32)>>>::as_ref(&box_coordinates)[index];
        if metadata(string_ref).unwrap().is_dir() {
            display_folder(string_ref, (x, y));
        } else {
            mvprintw(y, x, string_ref);
        }
    }
}