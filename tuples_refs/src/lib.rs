pub struct Student<'a>(pub u32, pub &'a str, pub &'a str);


pub fn id(student: &Student) -> u32 {
    student.0
}


pub fn first_name(student: &Student) -> &str {
    student.1
}


pub fn last_name(student: &Student) -> &str {
    student.2
}


// #[cfg(test)]
// mod tests {
//     use super::*;
    
//     #[test]
//    fn test_id(){
//     let student = Student(20, "Pedro".to_string(), "Domingos".to_string());
//     assert_eq!(id(&student),20 );
//    }

//    #[test]
//    fn test_first_name(){
//     let student = Student(20, "Pedro".to_string(), "Domingos".to_string());
//     assert_eq!(first_name(&student),"Pedro" );
//    }

//    fn test_last_name(){
//     let student = Student(20, "Pedro".to_string(), "Domingos".to_string());
//     assert_eq!(last_name(&student),"Domingos");
//    }
// }
