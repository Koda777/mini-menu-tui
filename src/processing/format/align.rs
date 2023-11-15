use crate::entity::constant::{CursorElement, TextElement};
use crate::utils::screen::get_screen_size;
use crate::utils::word::get_longest_word;

pub fn put_centered_position(elements: &mut Vec<TextElement>) -> () {
    let len_screen = get_screen_size();
    let size = elements.len();
    let longest_word = get_longest_word(&elements.iter().map(|a| {a.name.to_string()}).collect());
    for (index, mut element) in elements.iter_mut().enumerate() {
        let x = (len_screen.0) / 2 - (longest_word / 2);
        let y = (len_screen.1 / 2) + index as i32 - (size as i32 /2);
        element.position.x = x;
        element.position.y = y;
    }
}

pub fn put_cursor_position(cursor: &mut CursorElement, element: &TextElement) -> () {
    cursor.position.y = element.position.y;
    cursor.position.x = element.position.x - 3;
}
