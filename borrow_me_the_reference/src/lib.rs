pub fn delete_and_backspace(s: &mut String) {
    let mut i = 0;
    while i < s.len() {
        let c = s.chars().nth(i).unwrap();
        
        if c == '-' {
         
            s.remove(i); 
            if i > 0 {
                i -= 1; 
                s.remove(i); 
            }
        } else if c == '+' {
            
            s.remove(i); 
            if i < s.len() {
                s.remove(i);
            }
        } else {
            i += 1; 
        }
    }
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