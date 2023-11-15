use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;

#[derive(Debug)]
struct SharedData {
    value: Vec<(i32, i32)>
}

#[derive(Debug)]
pub(crate) struct DataMenu {
    shared_data: Arc<Mutex<SharedData>>,
}

impl DataMenu {
    fn new(initial_value: Vec<(i32, i32)>) -> DataMenu {
        let shared_data = Arc::new(Mutex::new(SharedData { value: initial_value }));
        DataMenu { shared_data }
    }

    pub(crate) fn get_value(&self) -> Vec<(i32, i32)> {
        let shared_data = self.shared_data.lock().unwrap();
        shared_data.value.clone()
    }

    pub(crate) fn set_value(&self, new_value: Vec<(i32, i32)>) {
        let mut shared_data = self.shared_data.lock().unwrap();
        shared_data.value = new_value;
    }
}

lazy_static! {
    pub(crate) static ref DATA_MENU: DataMenu = DataMenu::new(vec![(0, 0)]);
}
