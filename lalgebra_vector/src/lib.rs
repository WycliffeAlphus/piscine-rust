use std::ops::{Add, Mul};
use std::iter::Sum;
use std::clone::Clone;
use std::cmp::PartialEq;
use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vector<T: Scalar>(pub Vec<T>);

pub trait Scalar: 
    Add<Output = Self> + 
    Mul<Output = Self> + 
    Sum + 
    Clone + 
    Debug + 
    PartialEq + 
    Eq 
{}

impl<T> Scalar for T where
    T: Add<Output = T> + 
       Mul<Output = T> + 
       Sum + 
       Clone + 
       Debug + 
       PartialEq + 
       Eq 
{}

impl<T: Scalar> Add for Vector<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        if self.0.len() != other.0.len() {
            panic!("Vectors must be of the same length for addition.");
        }

        let data = self.0.into_iter()
                         .zip(other.0.into_iter())
                         .map(|(a, b)| a + b)
                         .collect();
        Vector(data)
    }
}

impl<T: Scalar> Vector<T> {
    pub fn new() -> Self {
        Vector(Vec::new())
    }

 
    pub fn dot(&self, other: &Self) -> Option<T> {
        if self.0.len() != other.0.len() {
            return None;
        }

        Some(
            self.0.iter()
                 .cloned()
                 .zip(other.0.iter().cloned())
                 .map(|(a, b)| a * b)
                 .sum()
        )
    }

    pub fn try_add(&self, other: &Self) -> Option<Self> {
        if self.0.len() != other.0.len() {
            return None;
        }

        let data = self.0.iter()
                         .cloned()
                         .zip(other.0.iter().cloned())
                         .map(|(a, b)| a + b)
                         .collect();
        Some(Vector(data))
    }
}
