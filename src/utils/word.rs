pub fn get_longest_word(vec: &Vec<String>) -> i32 {
    vec.iter().map(|x| x.len()).max().unwrap_or(0) as i32
}
