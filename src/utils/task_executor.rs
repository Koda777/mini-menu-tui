#[derive(Debug)]
pub struct TaskExecutor <T, F> where F: Fn() -> T,
{
    name: String,
    task: F
}

impl <T, F> TaskExecutor <T, F> where F: Fn() -> T
{
    pub fn new(name: String, task: F) -> Self {
        TaskExecutor {name, task} }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}
