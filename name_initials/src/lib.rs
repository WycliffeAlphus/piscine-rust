pub fn initials(names: Vec<&str>) -> Vec<String> {

    let mut result = Vec::new();

   
    for name in names {
        
        let words: Vec<&str> = name.split_whitespace().collect();

        let mut initials = String::new();

        
        for word in words {
            if let Some(first_char) = word.chars().next() {
               
                initials.push(first_char);
                initials.push('.');
                initials.push(' '); 
            }
        }

        
        if !initials.is_empty() {
            initials.pop(); 
     
        result.push(initials);
    }

    result
}
}