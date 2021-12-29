//! `NdSlice` wraps `&[T]` to represent an n-dimensional array

use std::slice;
use crate::addressing::Order;
use crate::errors::ShapeError;

#[derive(Debug, Clone)]
/// `NdSlice` wraps `&[T]` to represent an n-dimensional array
pub struct NdSlice<'s, T, const N: usize> {
    slice: &'s [T],
    shape: [usize; N],
    order: Order,
}

impl<'s, T, const N: usize> NdSlice<'s, T, N> {
    /// Creates a new `NdSlice` with row-major ordering from a given slice and the expected shape
    pub fn new(slice: &'s [T], shape: [usize; N]) -> Result<Self, ShapeError<'s, T, N>> {
        if slice.len() == shape.iter().fold(1, |acc, &x| acc * x) {
            Ok(Self { slice, shape, order: Order::RowMajor })
        } else {
            Err(ShapeError::new(slice, shape))
        }
    }

    /// Creates a new `NdSlice` with row-major ordering from a raw pointer, it's length and the expected shape
    pub unsafe fn new_from_ptr(ptr: *const T, len: usize, shape: [usize; N]) -> Result<Self, ShapeError<'s, T, N>> {
        let slice = slice::from_raw_parts(ptr, len);

        NdSlice::new(slice, shape)
    }

    /// Creates a new `NdSlice` with column-major ordering from a given slice and the expected shape
    pub fn newc(slice: &'s [T], shape: [usize; N]) -> Result<Self, ShapeError<'s, T, N>> {
        if slice.len() == shape.iter().fold(1, |acc, &x| acc * x) {
            Ok(Self { slice, shape, order: Order::ColumnMajor })
        } else {
            Err(ShapeError::new(slice, shape))
        }
    }

    /// Creates a new `NdSlice` with column-major ordering from a raw pointer, it's length and the expected shape.
    pub unsafe fn newc_from_ptr(ptr: *const T, len: usize, shape: [usize; N]) -> Result<Self, ShapeError<'s, T, N>> {
        let slice = slice::from_raw_parts(ptr, len);

        NdSlice::newc(slice, shape)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ARR: [usize; 8] = [1, 2, 3, 4, 5, 6, 7, 8];

    #[test]
    fn new() {
        let _ = NdSlice::new(&ARR, [8]).unwrap();
        let _ = NdSlice::new(&ARR, [4, 2]).unwrap();
        let _ = NdSlice::new(&ARR, [2, 2, 2]).unwrap();
    }

    #[test]
    #[should_panic]
    fn new_err() {
        // There's probably a better way to do this
        let _ = NdSlice::new(&ARR, [9]).unwrap();
        let _ = NdSlice::new(&ARR, [4, 3]).unwrap();
        let _ = NdSlice::new(&ARR, [2, 2, 3]).unwrap();
    }

    #[test]
    fn new_from_ptr() {
        let ptr = ARR.as_ptr();
        unsafe {
            let _ = NdSlice::new_from_ptr(ptr, 8, [8]);
            let _ = NdSlice::new_from_ptr(ptr, 8, [4, 2]);
            let _ = NdSlice::new_from_ptr(ptr, 8, [2, 2, 2]);
        }

    }
}
