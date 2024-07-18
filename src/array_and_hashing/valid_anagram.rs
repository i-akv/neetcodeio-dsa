pub struct Solution;
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut map = std::collections::HashMap::new();
        for char in t.chars() {
            match map.get(&char) {
                Some(val) => map.insert(char, val + 1),
                None => map.insert(char, 1),
            };
        }

        for char in s.chars() {
            match map.get(&char) {
                Some(val) => match val {
                    &1 => map.remove(&char),
                    _ => map.insert(char, val - 1),
                },
                None => return false,
            };
        }

        true
    }
}

#[test]
fn test_1() {
    assert_eq!(
        true,
        Solution::is_anagram("anagram".to_string(), "nagaram".to_string())
    );
}

#[test]
fn test_2() {
    assert_eq!(
        false,
        Solution::is_anagram("rat".to_string(), "car".to_string())
    );
}
