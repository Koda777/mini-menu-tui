use crate::event::input;
use crate::parcing::position;
use crate::constants::data_menu::DATA_MENU;
use crate::window::cursor;

extern crate ncurses;
use ncurses::*;
pub fn init_window() {
    initscr();
    raw();
    keypad(stdscr(), true);
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    noecho();
    let vec: Vec<&str> = vec!["Hello", "World", "From", "Rust"];
    let box_coordinates = position::center_by_list(&vec);
    DATA_MENU.set_value(box_coordinates.clone());
    cursor::init_cursor();
    cursor::display_cursor();
    loop {
        for (index, string_ref) in vec.iter().enumerate() {
            let (x, y) = <Vec<(i32, i32)> as AsRef<Vec<(i32, i32)>>>::as_ref(&box_coordinates)[index];
            mvprintw(y, x, string_ref);
        }
        input::get_input();
        cursor::display_cursor();
        refresh();
    }
}