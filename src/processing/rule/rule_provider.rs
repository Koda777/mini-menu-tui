use crate::entity::status::EventStatus;

pub trait RuleProvider {
    fn provide_rules(&mut self, status: EventStatus);
}
