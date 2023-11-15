use std::fmt::Display;
use std::process::{exit, Command};
use ncurses::ll::endwin;
use walkdir::WalkDir;
use crate::entity::constant::{CursorElement, Metadata, Position, TextElement};
use crate::entity::status::{EventStatus, ResourceType};
use crate::entity::status::EventStatus::{CursorDownward, CursorUpward, Init, Enter, Back, ResetCursor};
use crate::processing::display::display_provider::DisplayProvider;
use crate::processing::format::align::{put_centered_position, put_cursor_position};
use crate::processing::rule::rule_provider::RuleProvider;

pub struct RuleExecutor<'a> {
    display: Box<dyn DisplayProvider + 'a>,
    texts: &'a mut Vec<Vec<TextElement>>,
    cursor: &'a mut CursorElement,
}

impl<'a> RuleExecutor<'a> {
    pub fn new(
        display: Box<dyn DisplayProvider + 'a>,
        texts: &'a mut Vec<Vec<TextElement>>,
        cursor: &'a mut CursorElement,
    ) -> Self {
        RuleExecutor { display, texts, cursor }
    }

    fn perform_workdir(&mut self, path: &str) {
        let mut node: Vec<TextElement> = vec![];
        for entry in WalkDir::new(path).max_depth(1).into_iter().filter_map(|e| e.ok()).skip(1) {
            let resource_type = if entry.path().is_dir() { ResourceType::Directory } else { ResourceType::None };
            let details = if entry.path().is_dir() { entry.path().to_str().expect("Something wrong during the folder construction process") } else { "" };
            let element = TextElement::new(
                Position::DEFAULT_POSITION,
                entry.path().file_name().unwrap().to_str().unwrap().to_string(),
                Metadata::new(resource_type, details.to_string())
            );
            node.push(element);
        }
        if !node.is_empty() {
            self.texts.push(node);
            self.handle_status(Init);
        }
    }

    fn perform_task(&mut self) -> i32 {
        let element_position = self.cursor.position.y;
        let last_row = self.texts.last_mut().expect("No rows available");
        let text_requested = last_row.iter_mut()
            .find(|a| a.position.y == element_position)
            .expect("Something wrong, cursor out of range..");
        let metadata_requested = text_requested.metadata.clone();
        match metadata_requested.resource {
            ResourceType::None => 0,
            ResourceType::Directory => {
                Command::new("cd")
                    .arg(metadata_requested.details)
                    .output()
                    .expect("Something wrong during the 'cd Project' process");
                exit(1)
            },
            ResourceType::Workdir => {
                self.perform_workdir(&metadata_requested.details);
               1
            },
            ResourceType::Command => 1,
        }
    }

    fn handle_status(&mut self, status: EventStatus) -> i32 {
        let mut last_row = self.texts.last_mut().expect("No rows available");
        match status {
            Enter => {
                self.perform_task();
                1
            },
            Init => {
                put_centered_position(&mut last_row);
                put_cursor_position(&mut self.cursor, last_row.get(0).expect("Cannot get text first element of text elements"));
                1
            },
            ResetCursor => {
                put_cursor_position(&mut self.cursor, last_row.get(0).expect("Cannot get text first element of text elements"));
                2
            },
            CursorUpward  => {
                let first_element = last_row.first().expect("Cannot get first element of text elements");
                if self.cursor.position.y > first_element.position.y {
                    self.cursor.position.y -= 1;
                }
                1
            },
            CursorDownward => {
                let last_element = last_row.last().expect("Cannot get last element of text elements");
                if self.cursor.position.y < last_element.position.y {
                    self.cursor.position.y += 1;
                }
                1
            },
            Back => {
                let len = self.texts.len();
                if len > 1 {
                    self.texts.pop();
                    return 1
                }
                -1
            }
        }
    }
}

impl<'a> RuleProvider for RuleExecutor<'a> {
    fn provide_rules(&mut self, status: EventStatus) -> i32 {
        let out = self.handle_status(status);
        // TODO Have to avoid to display if return from cursor reset status (display twice otherwise)
        if out == 1 && out != 2 {
            self.display.generate(self.texts.last_mut().expect("No rows available"), self.cursor);
        }
        out
    }
}
