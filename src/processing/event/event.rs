use ncurses::{endwin, getch};
use crate::entity::status::EventStatus::{Back, CursorDownward, CursorUpward, Enter, Init, ResetCursor};
use crate::processing::event::event_provider::{EventProvider};
use crate::processing::rule::rule_provider::RuleProvider;

pub struct Event<'a> {
    listener: Box<dyn RuleProvider + 'a>,
}

impl<'a> Event<'a> {
    pub fn new(mut listener: Box<dyn RuleProvider + 'a>) -> Self {
        listener.provide_rules(Init);
        Event { listener }
    }

    fn execute_user_input(&mut self, input : i32) -> i32 {
        match input {
            113 => {
                endwin();
                -1
            },
            106 => {
                self.listener.provide_rules(CursorDownward);
                1
            },
            107 => {
                self.listener.provide_rules(CursorUpward);
                1
            },
            10 => {
                self.listener.provide_rules(Enter);
                1
            },
            104 => {
                if self.listener.provide_rules(Back) == 1 {
                    self.listener.provide_rules(ResetCursor);
                }
                1
            },
            _ => {
                println!("Unknown input: {}", input); 
                0
            }
        }
    }
}

impl<'a> EventProvider for Event<'a> {
    fn generate(&mut self) {
        loop {
            if self.execute_user_input(getch()) == -1 {
                break;
            }
        }
    }
}
