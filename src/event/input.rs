use crate::constants::data_cursor::DATA_CURSOR;
use crate::constants::data_menu::DATA_MENU;

extern crate ncurses;

use ncurses::*;

fn is_valid_horizontal(new_y: i32) -> bool {
    if (new_y >= 0) && (new_y < DATA_MENU.get_value().len() as i32) {
        return true;
    }
    return false;
}

fn remove_cursor() {
    let logo = DATA_CURSOR.get_logo();
    let position = DATA_CURSOR.get_position() as usize;
    let y = DATA_MENU.get_value()[position].1;
    let x = DATA_CURSOR.get_value().0;
    let mut i = 0;

    for _ in logo.chars() {
        mvprintw(y, x + i, " ");
        i += 1;
    }
}

pub fn get_input() -> i32 {
    let ch = getch();

    match ch {
        107 => {
            let y = DATA_CURSOR.get_position();
            if is_valid_horizontal(y  - 1) {
                remove_cursor();
                DATA_CURSOR.set_position(y - 1);
            }
        }
        106 => {
            let y = DATA_CURSOR.get_position();
            if is_valid_horizontal(y  + 1) {
                remove_cursor();
                DATA_CURSOR.set_position(y + 1);
            }
        }
        KEY_LEFT => {
        }
        KEY_RIGHT => {
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