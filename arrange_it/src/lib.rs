pub fn arrange_phrase(phrase: &str) -> String {
    // Split the phrase into words
    let words: Vec<&str> = phrase.split_whitespace().collect();
    
    // Create a vector of tuples (position, word without number)
    let mut word_positions: Vec<(usize, String)> = Vec::with_capacity(words.len());
    
    for word in words {
        // Find the position number in the word
        let mut position = 0;
        let mut word_without_number = String::with_capacity(word.len());
        
        for c in word.chars() {
            if c.is_digit(10) {
                position = c.to_digit(10).unwrap() as usize;
            } else {
                word_without_number.push(c);
            }
        }
        
        word_positions.push((position, word_without_number));
    }
    
    // Sort by position
    word_positions.sort_by_key(|&(pos, _)| pos);
    
    // Join the words back together
    let mut result = String::with_capacity(phrase.len());
    
    for (i, (_, word)) in word_positions.iter().enumerate() {
        result.push_str(word);
        if i < word_positions.len() - 1 {
            result.push(' ');
        }
    }
    
    result
}