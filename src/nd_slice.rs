//! `NdSlice` wraps `&[T]` to represent an n-dimensional array

use std::ops::Index;
use std::slice;
use crate::addressing::{Order, address};
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

impl<'s, T, const N: usize> Index<[usize; N]> for NdSlice<'s, T, N> {
    type Output = T;

    fn index(&self, index: [usize; N]) -> &'s Self::Output {
        let shape = &self.shape;
        let order = &self.order;
        
        &self.slice[address(order, shape, &index)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ARR: [usize; 8] = [1, 2, 3, 4, 5, 6, 7, 8];

    fn new_check<const N: usize>(shape: [usize; N]) {
        NdSlice::new(&ARR, shape).unwrap();
    }

    #[test]
    fn new() {
        new_check([8]);
        new_check([4, 2]);
        new_check([2, 2, 2]);
    }

    #[test]
    #[should_panic]
    fn new_err() {
        new_check([9]);
        new_check([4, 3]);
        new_check([2, 2, 3]);
    }

    #[test]
    fn new_from_ptr() {
        let ptr = ARR.as_ptr();
        unsafe {
            NdSlice::new_from_ptr(ptr, 8, [8]).unwrap();
            NdSlice::new_from_ptr(ptr, 8, [4, 2]).unwrap();
            NdSlice::new_from_ptr(ptr, 8, [2, 2, 2]).unwrap();
        }
    }

    #[test]
    fn index_test() {
        let rm = NdSlice::new(&[1, 2, 3, 4, 5, 6], [2, 3]).unwrap();
        let cm = NdSlice::newc(&[1, 4, 2, 5, 3, 6], [2, 3]).unwrap();

        assert!(rm[[0, 0]] == 1 && rm[[0, 0]] == cm[[0, 0]]);
        assert!(rm[[0, 1]] == 2 && rm[[0, 1]] == cm[[0, 1]]);
        assert!(rm[[0, 2]] == 3 && rm[[0, 2]] == cm[[0, 2]]);
        assert!(rm[[1, 0]] == 4 && rm[[1, 0]] == cm[[1, 0]]);
        assert!(rm[[1, 1]] == 5 && rm[[1, 1]] == cm[[1, 1]]);
        assert!(rm[[1, 2]] == 6 && rm[[1, 2]] == cm[[1, 2]]);
    }
}
