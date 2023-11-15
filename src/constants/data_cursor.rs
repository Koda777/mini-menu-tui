use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;

#[derive(Debug)]
struct SharedData {
    value: (i32, i32),
    position: i32,
    logo: String,
}

#[derive(Debug)]
pub(crate) struct DataCursor {
    shared_data: Arc<Mutex<SharedData>>,
}

impl DataCursor {
    fn new(initial_value: (i32, i32), logo: &str) -> DataCursor {
        let shared_data = Arc::new(Mutex::new(SharedData { value: initial_value, logo: String::from(logo), position: 0 }));
        DataCursor { shared_data }
    }

    pub(crate) fn get_value(&self) -> (i32, i32) {
        let shared_data = self.shared_data.lock().unwrap();
        shared_data.value
    }

    pub(crate) fn get_position(&self) -> i32 {
        let shared_data = self.shared_data.lock().unwrap();
        shared_data.position
    }


    pub(crate) fn get_logo(&self) -> String {
        let shared_data = self.shared_data.lock().unwrap();
        shared_data.logo.clone()
    }

    pub(crate) fn set_value(&self, new_value: (i32, i32)) {
        let mut shared_data = self.shared_data.lock().unwrap();
        shared_data.value = new_value;
    }

    pub(crate) fn set_position(&self, new_value: i32) {
        let mut shared_data = self.shared_data.lock().unwrap();
        shared_data.position = new_value;
    }

    pub(crate) fn set_logo(&self, new_value: &str) {
        let mut shared_data = self.shared_data.lock().unwrap();
        shared_data.logo = String::from(new_value);
    }
}

lazy_static! {
    pub(crate) static ref DATA_CURSOR: DataCursor = DataCursor::new((0, 0), ">>");
}
