extern crate ncurses;

use crate::constants::data_cursor::DATA_CURSOR;
use crate::constants::data_menu::DATA_MENU;
use ncurses::*;

pub fn sort_files(vec: &mut Vec<String>) {
    vec.sort();
}
