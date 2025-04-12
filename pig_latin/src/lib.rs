pub fn pig_latin(text: &str) -> String {

    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let word = text.to_lowercase();
    let chars:Vec<char> = word.chars().collect();

    if vowels.contains(&chars[0]){
        return format!("{}ay", word);
    }

    if chars.len() >= 3 && !vowels.contains(&chars[0]) && chars[1] == 'q' && chars[2] == 'u'{
        let head = &word[3..];
        let tail = &word[0..3];
        return format!("{}{}ay", head, tail);
    }


    for (i, c) in chars.iter().enumerate(){
        if vowels.contains(c) {
            let head = &word[i..];
            let tail = &word[..i];
            return format!("{}{}ay", head, tail);
        }
    }
    format!("{}ay",word)
}