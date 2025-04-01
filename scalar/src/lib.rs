pub fn sum(a: u8, b: u8) -> u8 {
    a + b
}

pub fn diff(a :i16, b: i16) -> i16 {
    a - b 
}

pub fn pro(a:i8 , b:i8 ) -> i8 {
    a * b
}

pub fn quo(a:f32, b:f32) -> f32 {
    a /b
}

pub fn rem(a: f32, b:f32) -> f32 {
    a % b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        let result = sum(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_diff(){
        let result: i16 = diff(4, 2);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_pro(){
        let result: i8 = pro(4, 2);
        assert_eq!(result, 8);
}
    #[test]
    fn test_quo(){
        let result: f32 = quo(4.0, 2.0);
        assert_eq!(result, 2.0);
    }

    #[test]
    fn test_rem(){
        let result: f32 = rem(4.0, 2.0);
        assert_eq!(result, 0.0);
    }
}