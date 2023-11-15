mod utils;
mod processing;
mod entity;

use ncurses::{curs_set, initscr, keypad, noecho, raw, start_color, stdscr, CURSOR_VISIBILITY};
use crate::processing::init::init;
extern crate ncurses;

fn main() {
    initscr();
    raw();
    keypad(stdscr(), true);
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    noecho();
    start_color();
    init();
    println!("Program finished successfully")
}
