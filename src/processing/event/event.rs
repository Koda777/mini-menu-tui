use ncurses::{endwin, getch};
use crate::entity::status::EventStatus::{CursorDownward, CursorUpward, Init};
use crate::utils::convert::convert_i32_to_char;
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

    fn execute_user_input(&mut self, input : char) -> i32 {
        match input {
            'q'  => {
                endwin();
                -1
            },
            'j' => {
                self.listener.provide_rules(CursorDownward);
                1
            },
            'k' => {
                self.listener.provide_rules(CursorUpward);
                1
            },
            _ => {0}
        }
    }
}

impl<'a> EventProvider for Event<'a> {
    fn generate(&mut self) {
        loop {
            match convert_i32_to_char(getch()) {
                Some(c) => {
                    if self.execute_user_input(c) == -1 {
                        break;
                    }
                },
                None => {
                    println!("Command not supported")
                }
            }
        }
    }
}
