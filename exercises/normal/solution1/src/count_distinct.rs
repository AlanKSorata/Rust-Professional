use std::collections::HashSet;

pub fn new_count_distinct(input_str: &str) -> usize {
    let arr = input_str.trim().split(',').collect::<Vec<&str>>();
    let have = HashSet::<&str>::from_iter(arr.into_iter());
    have.len()
}
