pub fn convert_i32_to_char(value: i32) -> Option<char> {
    if (0..=255).contains(&value) {
        let u8_value = value as u8;
        if u8_value.is_ascii() {
            return Some(u8_value as char);
        }
    }
    None
}
