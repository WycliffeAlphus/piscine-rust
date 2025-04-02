pub fn rev_str(input: &str) -> String {

let mut res = String::new();

for ch in input.chars().rev(){
    res.push(ch);
}
res
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rev_str(){
        let result = rev_str("Hello, world!");
    assert_eq!("!dlrow ,olleH".to_string(), result)
    }
    
}
