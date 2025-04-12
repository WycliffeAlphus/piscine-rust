pub fn rotate(input: &str, key: i8) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_ascii_lowercase() {
                (((c as i8) - (b'a' as i8) + key).rem_euclid(26) + (b'a' as i8)) as u8 as char
            } else if c.is_ascii_uppercase() {
                (((c as i8) - (b'A' as i8) + key).rem_euclid(26) + (b'A' as i8)) as u8 as char
            } else {
                c
            }
        })
        .collect()
}
