extern crate ncurses;

use sysinfo::{ProcessExt, SystemExt};
use crate::entity::constant::{TextElement, Position, CursorElement, Metadata};
use crate::entity::status::ResourceType;
use crate::processing::display::display::Display;
use crate::processing::display::display_provider::DisplayProvider;
use crate::processing::event::event::Event;
use crate::processing::event::event_provider::EventProvider;
use crate::processing::rule::rule::RuleExecutor;

pub fn init() {
    let mut elements = vec![vec![
        TextElement::new(Position::DEFAULT_POSITION, "Project".to_string(), Metadata::new(ResourceType::Workdir, "/Users/koda/Project/".to_string())),
        TextElement::new(Position::DEFAULT_POSITION, "Description".to_string(), Metadata::new(ResourceType::None, "".to_string())),
    ]];
    let mut cursor = CursorElement::new(Position::DEFAULT_POSITION, ">>".to_string());
    let menu_display: Box<dyn DisplayProvider> = Box::new(Display::new());
    let rule = Box::new(RuleExecutor::new(menu_display, &mut elements, &mut cursor));
    Event::new(rule).generate();
}
