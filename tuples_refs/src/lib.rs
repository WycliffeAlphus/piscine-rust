pub struct Student<'a>(pub u32, pub &'a str, pub &'a str);

pub fn id(student: &Student) -> u32 {
    student.0
}

pub fn first_name<'a>(student: &'a Student<'a>) -> &'a str {
    student.1
}

pub fn last_name<'a>(student: &'a Student<'a>) -> &'a str {
    student.2
}




#[cfg(test)]
mod tests {
    use super::*;
    
    const FIRST: &str = "Wycliffe";
    const SECOND: &str = "Onyango";
    
    const STUDENT: Student = Student(20, FIRST, SECOND);
    
    #[test]
    fn test_id() {
        assert_eq!(id(&STUDENT), 20);
    }

    #[test]
    fn test_first_name() {
        assert_eq!(first_name(&STUDENT), "Wycliffe");
    }

    #[test]
    fn test_last_name() {
        assert_eq!(last_name(&STUDENT), "Onyango");
    }
}
