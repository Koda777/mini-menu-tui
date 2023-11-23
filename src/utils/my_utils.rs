use ncurses::*;

pub fn get_screen_size() -> (i32, i32) {
    let mut screen_size: (i32, i32) = (0, 0);

    getmaxyx(stdscr(), &mut screen_size.1, &mut screen_size.0);
    screen_size
}

pub fn get_longest_world(vec: Vec<String>) -> i32 {
    let max_size = vec.iter().map(|x| x.len()).max().unwrap_or(0) as i32;

    max_size
}