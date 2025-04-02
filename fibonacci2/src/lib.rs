pub fn fibonacci(n: u32) -> u32 {

    if n == 1 {
        return 1;
    }

    if n == 0 {
        return 0;
    }

    fibonacci(n-1) + fibonacci(n-2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonnacci(){
        assert_eq!(fibonacci(4), 3);
    }
}
