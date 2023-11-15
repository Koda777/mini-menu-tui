use ncurses::{clear, mvprintw, refresh};
use crate::entity::constant::{TextElement, CursorElement};
use crate::processing::display::display_provider::DisplayProvider;

pub struct Display {
}

impl Display {
    pub fn new() -> Self {
        Display { }
    }
}

impl DisplayProvider for Display {
    fn generate(&self, elements: &Vec<TextElement>, cursor: &CursorElement) {
        clear();
        for (_, element) in elements.iter().enumerate() {
            mvprintw(element.position.y, element.position.x, &element.name);
        }
        mvprintw(cursor.position.y, cursor.position.x, &cursor.name);
        refresh();
    }
}
