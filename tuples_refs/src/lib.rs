pub struct Student(pub u32, pub String, pub String);

pub fn id(student: &Student) -> u32 {
    student.0
}

pub fn first_name(student:  &Student) ->  &str {
    &student.1
}

pub fn last_name(student: &Student) -> &str {
    &student.2
}




// #[cfg(test)]
// mod tests {
//     use super::*;
    
//     static FIRST: String = "Wycliffe".to_string();
//     static SECOND: String = "Onyango".to_string();
    
//     static STUDENT: Student = Student(20, FIRST.clone(), SECOND.clone());
    
//     #[test]
//     fn test_id() {
//         assert_eq!(id(&STUDENT), 20);
//     }

//     #[test]
//     fn test_first_name() {
//         assert_eq!(first_name(&STUDENT), "Wycliffe");
//     }

//     #[test]
//     fn test_last_name() {
//         assert_eq!(last_name(&STUDENT), "Onyango");
//     }
// }
