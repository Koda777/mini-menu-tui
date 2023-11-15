use crate::utils::my_utils;

pub fn center_by_list(vec: &Vec<&str>) -> Vec<(i32, i32)> {
    let screen_size = my_utils::get_screen_size();
    let mut coordinates: Vec<(i32, i32)> = Vec::new();

    for (index, _) in vec.iter().enumerate() {
        let x = (screen_size.0) / 2;
        let y = (screen_size.1 / 2) + index as i32;
        coordinates.push((x, y));
    }
    if coordinates.len() == 0 {
        coordinates.push((0, 0));
    }
    coordinates
}