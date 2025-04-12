use std::collections::HashMap;

pub fn is_pangram(s: &str) -> bool {

let mut map = HashMap::new();

for c in s.chars() {
    if c.is_alphabetic() {
        map.insert(c.to_ascii_lowercase(), true);
    }
}

if map.len() == 26 {
    return true;
} 
false
}

