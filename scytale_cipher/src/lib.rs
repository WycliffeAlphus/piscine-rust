pub fn scytale_cipher(message: String, i: u32) -> String {


let i = i as usize;

let msg_len = message.len();


let cols = (msg_len as f32 / i as f32).ceil() as usize;

let mut grid = vec![vec![' ';cols]; i];
for (k, ch) in message.chars().enumerate(){
    let row = k%i;
    let col = k/i;
    grid[row][col] = ch;
}


// reading is column by column


let mut result = String::new();


for col in 0..cols {
    for row in 0..i {
        let ch = grid[row][col];
        if ch != ' ' {
            result.push(ch);
        }
    }
}
result
}
