use std::ops::{Add, Sub};

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T> Add for Matrix<T>
where
    T: Add<Output = T> + Clone,
{
    type Output = Option<Matrix<T>>;

    fn add(self, rhs: Matrix<T>) -> Self::Output {
        if self.0.len() != rhs.0.len() {
            return None;
        }

        let mut result = Vec::new();

        for (row_a, row_b) in self.0.into_iter().zip(rhs.0.into_iter()) {
            if row_a.len() != row_b.len() {
                return None;
            }

            let new_row: Vec<T> = row_a
                .into_iter()
                .zip(row_b.into_iter())
                .map(|(a, b)| a + b)
                .collect();

            result.push(new_row);
        }

        Some(Matrix(result))
    }
}


impl<T> Sub for Matrix<T>
where
    T: Sub<Output = T> + Clone,
{
    type Output = Option<Matrix<T>>;

    fn sub(self, rhs: Matrix<T>) -> Self::Output {
        if self.0.len() != rhs.0.len() {
            return None;
        }

        let mut result = Vec::new();

        for (row_a, row_b) in self.0.into_iter().zip(rhs.0.into_iter()) {
            if row_a.len() != row_b.len() {
                return None;
            }

            let new_row: Vec<T> = row_a
                .into_iter()
                .zip(row_b.into_iter())
                .map(|(a, b)| a - b)
                .collect();

            result.push(new_row);
        }

        Some(Matrix(result))
    }
}
