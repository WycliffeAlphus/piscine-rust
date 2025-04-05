use std::collections::HashMap;

pub fn is_permutation(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    fn char_count_map(s: &str) -> HashMap<char, usize> {
        let mut map = HashMap::new();
        for c in s.chars() {
            *map.entry(c).or_insert(0) += 1;
        }
        map
    }

    char_count_map(s1) == char_count_map(s2)
}
