use std::env;
mod utils;
mod parcing;
mod event;
mod constants;
mod window;

use crate::window::init::{init_window};
use crate::utils::path::{get_current_dir};
extern crate ncurses;

fn main() {
    init_window();
}