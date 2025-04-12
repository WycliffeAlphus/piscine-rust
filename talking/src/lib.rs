pub fn talking(text: &str) -> &str {

let trimmed = text.trim();

if trimmed.is_empty(){
    return "Just say something!";
}

let is_question = trimmed.ends_with('?');
let is_yelling = trimmed
    .chars()
    .any(|c| c.is_alphabetic()) &&
    trimmed.chars()
    .filter(|c| c.is_alphabetic())
    .all(|c| c.is_uppercase());

    match (is_question, is_yelling) {
        (true, true) => "Quiet, I am thinking!",
        (true, false) => "Sure.",
        (false, true) => "There is no need to yell, calm down!",
        _=> "Interesting",
    }
}
    