pub fn get_diamond(c: char) -> Vec<String> {
    if !c.is_ascii_uppercase() {
        return vec![]; 
    }

    let max_index = (c as u8 - b'A') as usize;
    let mut lines = Vec::new();

    for i in 0..=max_index {
        let letter = (b'A' + i as u8) as char;
        let spaces = max_index - i;

        let line = if i == 0 {
            " ".repeat(spaces) + &letter.to_string() + &" ".repeat(spaces)
        } else {
            let inner = 2 * i - 1;
            format!(
                "{}{}{}{}{}",
                " ".repeat(spaces),
                letter,
                " ".repeat(inner),
                letter,
                " ".repeat(spaces)
            )
        };

        lines.push(line);
    }

    // Mirror the top half (excluding the center) to the bottom
    let bottom = lines[..max_index].iter().rev().cloned().collect::<Vec<_>>();
    lines.extend(bottom);
    lines
}
