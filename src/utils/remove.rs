extern crate ncurses;
use crate::constants::data_cursor::DATA_CURSOR;
use crate::constants::data_menu::DATA_MENU;
use ncurses::*;
pub fn remove_cursor() {
    let logo = DATA_CURSOR.get_logo();
    let position = DATA_CURSOR.get_position() as usize;
    let y = DATA_MENU.get_position()[position].1;
    let x = DATA_CURSOR.get_value().0;
    let mut i = 0;

    for _ in logo.chars() {
        mvprintw(y, x + i, " ");
        i += 1;
    }
}

pub fn remove_menu() {
    let vec = DATA_MENU.get_value().clone();
    let box_coordinates = DATA_MENU.get_position().clone();

    for (index, string_ref) in vec.iter().enumerate() {
        let (x, y) = <Vec<(i32, i32)> as AsRef<Vec<(i32, i32)>>>::as_ref(&box_coordinates)[index];
        let mut i = 0;

        for _ in string_ref.chars() {
            mvprintw(y, x + i, " ");
            i += 1;
        }
    }
}