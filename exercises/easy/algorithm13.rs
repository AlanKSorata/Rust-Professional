/*
    Anagram Check
    Given two strings, check if they are anagrams of each other.
    Anagrams are words or phrases formed by rearranging the letters of another,
    using all the original letters exactly once.
    The strings may contain spaces or punctuation, but you need to ignore them while checking.

    You need to implement the function `are_anagrams(s1: String, s2: String) -> bool`.
    The function should return `true` if the two strings are anagrams, and `false` otherwise.

    Hint: Consider normalizing the strings by removing non-alphabetical characters and converting to lowercase before checking.
*/

use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};

pub fn are_anagrams(s1: String, s2: String) -> bool {
    // TODO: Implement the logic to check if two strings are anagrams
    // 过滤并转换为小写
    let normalize = |s: String| -> Vec<char> {
        s.chars()
            .filter(|c| c.is_ascii_alphabetic()) // 只保留字母
            .map(|c| c.to_ascii_lowercase()) // 转换为小写
            .collect()
    };

    let chars1 = normalize(s1);
    let chars2 = normalize(s2);

    // 如果字符数量不同，直接返回 false
    if chars1.len() != chars2.len() {
        return false;
    }

    // 统计字符频率
    let mut freq1 = HashMap::new();
    let mut freq2 = HashMap::new();

    for &c in &chars1 {
        *freq1.entry(c).or_insert(0) += 1;
    }

    for &c in &chars2 {
        *freq2.entry(c).or_insert(0) += 1;
    }

    // 比较两个哈希表
    freq1 == freq2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anagram_1() {
        let s1 = "listen".to_string();
        let s2 = "silent".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_anagram_2() {
        let s1 = "evil".to_string();
        let s2 = "vile".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_anagram_3() {
        let s1 = "hello".to_string();
        let s2 = "world".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert_eq!(result, false);
    }

    #[test]
    fn test_anagram_4() {
        let s1 = "Clint Eastwood".to_string();
        let s2 = "Old West Action".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_anagram_5() {
        let s1 = "Astronomer".to_string();
        let s2 = "Moon starer".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert_eq!(result, true);
    }
}
