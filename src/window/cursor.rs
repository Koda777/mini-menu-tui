extern crate ncurses;
use crate::constants::data_cursor::DATA_CURSOR;
use crate::constants::data_menu::DATA_MENU;
use ncurses::*;

pub fn display_cursor() {
    let position = DATA_CURSOR.get_position() as usize;
    let y = DATA_MENU.get_position()[position].1;
    let (x, _) = DATA_CURSOR.get_value();
    let logo: String = DATA_CURSOR.get_logo();
    init_pair(1, COLOR_RED, COLOR_BLACK);
    attron(COLOR_PAIR(1));
    mvprintw(y, x, &logo);
    attroff(COLOR_PAIR(1));
}

pub fn init_cursor() {
    let vec: Vec<(i32, i32)> = DATA_MENU.get_position();
    let space_between = 3;
    let (x, y) = vec[0];

    DATA_CURSOR.set_value((x - DATA_CURSOR.get_logo().len() as i32 - space_between, y));
}
