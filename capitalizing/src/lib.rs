pub fn capitalize_first(input: &str) -> String {

    let mut chars = input.chars();
    match chars.next(){
        Some(first_char) => first_char.to_uppercase().collect::<String>()+chars.as_str(),
        None => String::new(),
    }
}

pub fn title_case(input: &str) -> String {

    input
        .split_whitespace()
        .map(|word| capitalize_first(word.trim()))
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn change_case(input: &str) -> String {
    input
        .chars()
        .map(|c| {

            if c.is_lowercase(){
                c.to_uppercase().to_string()
            } else{
                c.to_lowercase().to_string()
            }

        })
        .collect::<String>()
}