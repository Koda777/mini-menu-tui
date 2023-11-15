use ncurses::{getmaxyx, stdscr};

pub fn get_screen_size() -> (i32, i32) {
    let mut screen_size: (i32, i32) = (0, 0);
    getmaxyx(stdscr(), &mut screen_size.1, &mut screen_size.0);
    screen_size
}
