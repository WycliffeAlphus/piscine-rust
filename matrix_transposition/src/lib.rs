#[derive(Debug, PartialEq, Eq)]

pub struct Matrix((i32, i32), (i32, i32));


pub fn transpose(m: Matrix) -> Matrix {

let res = Matrix((m.0.0, m.1.0), (m.0.1, m.1.1));
res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transpose(){
        assert_eq!(transpose(Matrix((1,3), (4,5))), Matrix((1, 4), (3, 5)));
    }
}


