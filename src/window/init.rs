extern crate ncurses;

use crate::event::input;
use crate::parcing::position;
use crate::constants::data_menu::DATA_MENU;
use crate::window::{cursor, menu};
use crate::utils::path;
use crate::utils::sort;
use crate::utils::remove;
use ncurses::*;
use crate::utils::sort::sort_files;

pub fn init_window() {
    initscr();
    raw();
    keypad(stdscr(), true);
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    noecho();
    start_color();
    let mut vec: Vec<String> = path::get_current_dir();
    sort_files(&mut vec);
    let box_coordinates = position::center_by_list(&vec);
    DATA_MENU.set_position(box_coordinates.clone());
    DATA_MENU.set_value(vec.clone());
    cursor::init_cursor();
    loop {
        let mut vec: Vec<String> = path::get_current_dir();
        sort_files(&mut vec);
        let box_coordinates = position::center_by_list(&vec);
        cursor::display_cursor();
        menu::display_menu(&vec, &box_coordinates.clone());
        input::get_input();
        refresh();
    }
}