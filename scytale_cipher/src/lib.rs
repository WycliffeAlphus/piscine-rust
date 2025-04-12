pub fn scytale_cipher(message: String, i: u32) -> String {
    let cols = i as usize;
    let chars: Vec<char> = message.chars().collect();
    let len = chars.len();
    let rows = ((len as f32) / (cols as f32)).ceil() as usize;

    // Fill the grid row by row
    let mut grid = vec![vec![' '; cols]; rows];
    for (index, ch) in chars.iter().enumerate() {
        let row = index / cols;
        let col = index % cols;
        grid[row][col] = *ch;
    }

    // Read column by column
    let mut result = String::new();
    for col in 0..cols {
        for row in 0..rows {
            if result.len() == len {
                break;
            }
            let ch = grid[row][col];
            result.push(ch);
        }
    }

    result
}
