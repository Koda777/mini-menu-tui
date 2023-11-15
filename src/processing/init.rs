extern crate ncurses;

use crate::entity::constant::{TextElement, Position, CursorElement};
use crate::processing::display::display::Display;
use crate::processing::display::display_provider::DisplayProvider;
use crate::processing::event::event::Event;
use crate::processing::event::event_provider::EventProvider;
use crate::processing::rule::rule::RuleExecutor;

pub fn init() {
    let mut elements = vec![
        TextElement::new(Position::DEFAULT_POSITION, "Project".to_string()),
        TextElement::new(Position::DEFAULT_POSITION, "Description".to_string())
    ];
    let mut cursor = CursorElement::new(Position::DEFAULT_POSITION, ">>".to_string());
    let menu_display: Box<dyn DisplayProvider> = Box::new(Display::new());
    let rule= Box::new(RuleExecutor::new(menu_display, &mut elements, &mut cursor));
    Event::new(rule).generate();
}
