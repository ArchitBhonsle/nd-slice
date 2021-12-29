//! This modules contains all the errors associated with this crate

use std::fmt;

/// Errors involving a shape mismatch
#[derive(Debug)]
pub struct ShapeError<'s, T, const N: usize> {
    slice: &'s [T],
    shape: [usize; N] // INFO might have to change this to a const generic later
}

impl<'s, T, const N: usize> fmt::Display for ShapeError<'s, T, N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f, 
            "Constructing an NdSlice of shape: {:?} would need a slice of length {}, but a slice of length {} was provided",
            self.shape,
            self.shape.iter().fold(1, |acc, &x| acc * x),
            self.slice.len()
        )
    }
}

impl<'s, T, const N: usize> ShapeError<'s, T, N> {
    /// Create a new `ShapeError` given the length of the slice and the expected shape.
    pub fn new(slice: &'s [T], shape: [usize; N]) -> Self {
        Self {
            slice,
            shape
        }
    }
}
