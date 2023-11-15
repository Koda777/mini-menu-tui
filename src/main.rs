mod utils;
mod parcing;
mod event;
mod constants;
mod window;

use crate::window::init::{init_window};
extern crate ncurses;

fn main() {
    init_window();
}