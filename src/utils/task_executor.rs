use crate::entity::constant::TextElement;

#[derive(Debug)]
pub struct TaskExecutor<F>
where
    F: Fn(&mut Vec<TextElement>),
{
    name: String,
    task: F,
}

impl<F> TaskExecutor<F>
where
    F: Fn(&mut Vec<TextElement>),
{
    pub fn new(name: String, task: F) -> Self {
        TaskExecutor { name, task }
    }

    pub fn execute(&self, elements: &mut Vec<TextElement>) {
        (self.task)(elements);
    }
}
