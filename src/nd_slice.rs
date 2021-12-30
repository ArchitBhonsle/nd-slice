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

type ConstructionResult<'s, T, const N: usize> = Result<NdSlice<'s, T, N>, ShapeError<'s, T, N>>;

impl<'s, T, const N: usize> NdSlice<'s, T, N> {
    /// Creates a new `NdSlice` with the specified ordering from a given slice and the expected shape
    pub fn new(slice: &'s [T], shape: [usize; N], order: Order) -> ConstructionResult<'s, T, N> {
        if slice.len() == shape.iter().fold(1, |acc, &x| acc * x) {
            Ok(Self { slice, shape, order })
        } else {
            Err(ShapeError::new(slice, shape))
        }
    }

    /// Creates a new `NdSlice` with the specified ordering from a raw pointer, it's length and the expected shape
    pub unsafe fn from_ptr(ptr: *const T, len: usize, shape: [usize; N], order: Order) -> ConstructionResult<'s, T, N> {
        NdSlice::new(slice::from_raw_parts(ptr, len), shape, order)
    }

    /// Creates a new `NdSlice` with row-major ordering from a given slice and the expected shape
    pub fn new_row_ordered(slice: &'s [T], shape: [usize; N]) -> ConstructionResult<'s, T, N> {
        NdSlice::new(slice, shape, Order::RowMajor)
    }

    /// Creates a new `NdSlice` with row-major ordering from a raw pointer, it's length and the expected shape
    pub unsafe fn row_ordered_from_ptr(ptr: *const T, len: usize, shape: [usize; N]) -> ConstructionResult<'s, T, N> {
        NdSlice::from_ptr(ptr, len, shape, Order::RowMajor)
    }

    /// Creates a new `NdSlice` with column-major ordering from a given slice and the expected shape
    pub fn col_ordered(slice: &'s [T], shape: [usize; N]) -> ConstructionResult<'s, T, N> {
        NdSlice::new(slice, shape, Order::ColumnMajor)
    }

    /// Creates a new `NdSlice` with column-major ordering from a raw pointer, it's length and the expected shape.
    pub unsafe fn col_ordered_from_ptr(ptr: *const T, len: usize, shape: [usize; N]) -> ConstructionResult<'s, T, N> {
        NdSlice::from_ptr(ptr, len, shape, Order::ColumnMajor)
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
        NdSlice::new_row_ordered(&ARR, shape).unwrap();
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
            NdSlice::row_ordered_from_ptr(ptr, 8, [8]).unwrap();
            NdSlice::row_ordered_from_ptr(ptr, 8, [4, 2]).unwrap();
            NdSlice::row_ordered_from_ptr(ptr, 8, [2, 2, 2]).unwrap();
        }
    }

    #[test]
    fn index_test() {
        let rm = NdSlice::new_row_ordered(&[1, 2, 3, 4, 5, 6], [2, 3]).unwrap();
        let cm = NdSlice::col_ordered(&[1, 4, 2, 5, 3, 6], [2, 3]).unwrap();

        assert!(rm[[0, 0]] == 1 && rm[[0, 0]] == cm[[0, 0]]);
        assert!(rm[[0, 1]] == 2 && rm[[0, 1]] == cm[[0, 1]]);
        assert!(rm[[0, 2]] == 3 && rm[[0, 2]] == cm[[0, 2]]);
        assert!(rm[[1, 0]] == 4 && rm[[1, 0]] == cm[[1, 0]]);
        assert!(rm[[1, 1]] == 5 && rm[[1, 1]] == cm[[1, 1]]);
        assert!(rm[[1, 2]] == 6 && rm[[1, 2]] == cm[[1, 2]]);
    }
}
