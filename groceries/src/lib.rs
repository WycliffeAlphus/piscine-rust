pub fn insert(vec: &mut Vec<String>, val: String) {
    vec.push(val);
}

pub fn at_index(slice: &[String], index: usize) -> &str {
    &slice[index]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut groceries = vec![
        "yogurt".to_string(),
        "panettone".to_string(),
        "bread".to_string(),
        "cheese".to_string(),
    ];

    let expected = vec![
        "yogurt".to_string(),
        "panettone".to_string(),
        "bread".to_string(),
        "cheese".to_string(),
        "nuts".to_string(),
    ];
        insert(&mut groceries, String::from("nuts"));
        assert_eq!(groceries, expected);
    }
    #[test]
    fn test_at_index(){
       let groceries: &[String] = &[
        "yogurt".to_string(),
        "panettone".to_string(),
        "bread".to_string(),
        "cheese".to_string(),
        "nuts".to_string(),
       ]; 
       let result =  at_index(&groceries, 2);
        assert_eq!(result, &"bread".to_string());
    }
}
