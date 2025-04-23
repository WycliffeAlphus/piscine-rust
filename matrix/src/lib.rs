use num::Zero;
#[derive(Debug)]
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
            data[i][i] = T::one();
        }
        Matrix(data)
	}
}