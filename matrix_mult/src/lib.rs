use std::ops::{Add, Sub, Mul, Div};

pub trait Scalar: Add<Output = Self>
                + Sub<Output = Self>
                + Mul<Output = Self>
                + Div<Output = Self>
                + Copy
                + PartialEq {
	type Item;
	fn zero() -> Self::Item;
	fn one() -> Self::Item;
}


impl Scalar for u32 {
    type Item = u32;

    fn zero() -> Self::Item {
        0
    }

    fn one() -> Self::Item {
        1
    }
}


impl Scalar for u64 {
	type Item = u64;

	fn zero() -> Self::Item {
		0
	}

	fn one() -> Self::Item {
		1
	}
}

impl Scalar for i32 {
	type Item = i32;

	fn zero() -> Self::Item {
		0
	}

	fn one() -> Self::Item {
		1
	}
}


impl Scalar for i64 {
	type Item = i64;

	fn zero() -> Self::Item {
		0
	}

	fn one() -> Self::Item {
		1
	}
}

impl Scalar for f32 {
	type Item = f32;

	fn zero() -> Self::Item {
		0.0
	}

	fn one() -> Self::Item {
		1.0
	}
}

impl Scalar for f64 {
	type Item = f64;

	fn zero() -> Self::Item {
		0.0
	}

	fn one() -> Self::Item {
		1.0
	}
}


#[derive(Debug, PartialEq, Clone)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl <T: Scalar<Item = T>> Matrix<T> {
	pub fn new() -> Matrix<T> {
        Matrix(vec![vec![T::zero()]])
	}

	pub fn zero(row: usize, col: usize) -> Matrix<T> {
        let row_vec = vec![T::zero(); col];
        let data = vec![row_vec; row];
        Matrix(data)
	}


	pub fn identity(n: usize) -> Matrix<T> {
        let mut data = vec![vec![T::zero();n]; n];
        for i in 0..n {
            data[i][i] = T::one()
        }
        Matrix(data)
	}
}


///
impl<T: Clone> Matrix<T> {
	pub fn number_of_cols(&self) -> usize {
        if self.0.is_empty(){
            0
        } else {
            self.0[0].len()
        }
	}

	pub fn number_of_rows(&self) -> usize {
        self.0.len()
	}

	pub fn row(&self, n: usize) -> Vec<T> {
        self.0[n].clone()
	}

	pub fn col(&self, n: usize) -> Vec<T> {
       self.0.iter().map(|row| row[n].clone()).collect()
	}
}

impl<T> Mul for Matrix<T>
where
    T: Clone + Default + std::ops::Add<Output = T> + std::ops::Mul<Output = T>,
{
    type Output = Option<Matrix<T>>;

    fn mul(self, rhs: Matrix<T>) -> Self::Output {
        let a_rows = self.number_of_rows();
        let a_cols = self.number_of_cols();
        let b_rows = rhs.number_of_rows();
        let b_cols = rhs.number_of_cols();

        if a_cols != b_rows {
            return None;
        }

        let mut result = vec![vec![T::default(); b_cols]; a_rows];

        for i in 0..a_rows {
            for j in 0..b_cols {
                for k in 0..a_cols {
                    result[i][j] =
                        result[i][j].clone() + self.0[i][k].clone() * rhs.0[k][j].clone();
                }
            }
        }

        Some(Matrix(result))
    }
}
