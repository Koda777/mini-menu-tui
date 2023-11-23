use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;

#[derive(Debug)]
struct SharedData {
    position: Vec<(i32, i32)>,
    value: Vec<String>,
}

#[derive(Debug)]
pub(crate) struct DataMenu {
    shared_data: Arc<Mutex<SharedData>>,
}

impl DataMenu {
    fn new() -> DataMenu {
        let shared_data = Arc::new(Mutex::new(SharedData { position: vec![], value: vec![] }));
        DataMenu { shared_data }
    }

    pub(crate) fn get_position(&self) -> Vec<(i32, i32)> {
        let shared_data = self.shared_data.lock().unwrap();
        shared_data.position.clone()
    }

    pub(crate) fn set_position(&self, new_value: Vec<(i32, i32)>) {
        let mut shared_data = self.shared_data.lock().unwrap();
        shared_data.position = new_value;
    }

    pub(crate) fn get_value(&self) -> Vec<String> {
        let shared_data = self.shared_data.lock().unwrap();
        shared_data.value.clone()
    }

    pub(crate) fn set_value(&self, new_value: Vec<String>) {
        let mut shared_data = self.shared_data.lock().unwrap();
        shared_data.value = new_value;
    }

    pub(crate) fn sort_files(&self) {
        let mut shared_data = self.shared_data.lock().unwrap();
        shared_data.value.sort();
    }
}

lazy_static! {
    pub(crate) static ref DATA_MENU: DataMenu = DataMenu::new();
}