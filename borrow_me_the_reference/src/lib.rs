pub fn delete_and_backspace(s: &mut String) {
    let mut chars: Vec<char> = s.chars().collect();
    let mut i = chars.len();
    while i > 0 {
        i -= 1;
        if chars[i] == '+' {
            chars.remove(i);
            if i < chars.len() {
                chars.remove(i);
            }
        }
    }
    i = 0;
    while i < chars.len() {
        if chars[i] == '-' {
            if i > 0 {
                chars.remove(i - 1);
                i -= 1;
            }
            chars.remove(i);
        } else {
            i += 1;
        }
    }

    *s = chars.into_iter().collect();
}


pub fn do_operations(v: &mut [String]) {
 let mut prev = 0;
 let mut ahead = 0; 
 let mut result = 0;  

for op in v {

    for (i, c) in op.char_indices(){
        if c == '+' || c == '-' {
            let num = &op[..i];
            let num1 = &op[i+1..];
            if let Ok(n) = num.parse::<i32>(){
                prev = n;
            }

            if let Ok(n2) = num1.parse::<i32>(){
                ahead = n2;
            }

            if c == '+'{
                result = prev+ahead;
            }
            if c == '-'{
                result = prev-ahead;
            }
        }
    }
    *op = result.to_string();
}
}