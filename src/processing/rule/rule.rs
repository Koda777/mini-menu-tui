use std::fmt::Display;
use crate::entity::constant::{CursorElement, TextElement};
use crate::entity::status::EventStatus;
use crate::entity::status::EventStatus::{CursorDownward, CursorUpward, Init};
use crate::processing::display::display_provider::DisplayProvider;
use crate::processing::format::align::{put_centered_position, put_cursor_position};
use crate::processing::rule::rule_provider::RuleProvider;

pub struct RuleExecutor<'a> {
    display: Box<dyn DisplayProvider + 'a>,
    texts: &'a mut Vec<TextElement>,
    cursor: &'a mut CursorElement
}

impl<'a> RuleExecutor<'a> {
    pub fn new(display: Box<dyn DisplayProvider + 'a>, texts: &'a mut Vec<TextElement>, cursor: &'a mut CursorElement) -> Self {
        RuleExecutor { display, texts, cursor }
    }
}

impl<'a> RuleProvider for RuleExecutor<'a> {
    fn provide_rules(&mut self, status: EventStatus) {
        match status {
            Init => {
                put_centered_position(&mut self.texts);
                put_cursor_position(&mut self.cursor, self.texts.get(0).expect("Cannot get text first element of text elements"));
            }
            CursorUpward  => {
                let first_element = self.texts.first().expect("Cannot get first element of text elements");
                if self.cursor.position.y > first_element.position.y {
                    self.cursor.position.y -= 1;
                }
            },
            CursorDownward => {
                let last_element = self.texts.last().expect("Cannot get last element of text elements");
                if self.cursor.position.y < last_element.position.y {
                    self.cursor.position.y += 1;
               }
            }
        }
        self.display.generate(self.texts, self.cursor);
    }
}
