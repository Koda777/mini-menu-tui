use crate::entity::constant::{CursorElement, TextElement};

pub trait DisplayProvider {
    fn generate(&self, elements: &Vec<TextElement>, cursor: &CursorElement);
}
