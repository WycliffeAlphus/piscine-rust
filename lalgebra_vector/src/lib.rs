use std::fmt::Debug;
use std::ops::{Add, Mul};

// Trait bounds: T needs to support basic operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vector<T: Scalar>(pub Vec<T>);

// Trait alias for all the capabilities T must have
pub trait Scalar: Copy + Add<Output = Self> + Mul<Output = Self> + Debug + PartialEq + Eq {}
impl<T> Scalar for T where T: Copy + Add<Output = T> + Mul<Output = T> + Debug + PartialEq + Eq {}

impl<T: Scalar> Vector<T> {
    pub fn new() -> Self {
        Vector(Vec::new())
    }

    pub fn dot(&self, other: &Self) -> Option<T> {
        if self.0.len() != other.0.len() {
            return None;
        }

        let mut result = self.0[0] * other.0[0];
        for i in 1..self.0.len() {
            result = result + self.0[i] * other.0[i];
        }
        Some(result)
    }
}

// Adding two vectors
impl<T: Scalar> Add for Vector<T> {
    type Output = Option<Vector<T>>;

    fn add(self, other: Self) -> Option<Vector<T>> {
        if self.0.len() != other.0.len() {
            return None;
        }

        let result: Vec<T> = self.0.iter()
            .zip(other.0.iter())
            .map(|(a, b)| *a + *b)
            .collect();

        Some(Vector(result))
    }
}
