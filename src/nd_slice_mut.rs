//! `NdSliceMut` wraps `&mut [T]` to represent a mutable n-dimensional array

use std::ops::{Index, IndexMut};
use std::slice;
use crate::addressing::{Order, address};
use crate::errors::ShapeError;

#[derive(Debug)]
/// `NdSliceMut` wraps `&mut [T]` to represent a mutable n-dimensional array
///
/// ```
/// # use nd_slice::{NdSliceMut, Order};
/// let mut arr = [7, 2, 3, 4, 5, 8];
/// let mut n = NdSliceMut::new(&mut arr, [2, 3], Order::RowMajor).unwrap();
/// n[[0, 0]] = 1;
/// n[[1, 2]] = 6;
/// assert_eq!(n[[0, 0]], 1);
/// assert_eq!(n[[1, 2]], 6);
///
/// let mut arr = [9, 2, 3, 4, 5, 6, 7, 10];
/// let mut n = NdSliceMut::new(&mut arr, [2, 2, 2], Order::RowMajor).unwrap();
/// n[[0, 0, 0]] = 1;
/// n[[1, 1, 1]] = 8;
/// assert_eq!(n[[0, 0, 0]], 1);
/// assert_eq!(n[[1, 1, 1]], 8);
/// ```
///
/// If the slice doesn't have enough elements to represent an array of that shape, it will
/// return an `Err(ShapeError)`.
///
/// ```should_panic
/// # use nd_slice::{NdSliceMut, Order};
/// let n = NdSliceMut::new(&mut [1, 2, 3, 4, 5, 6], [2, 2], Order::RowMajor).unwrap(); // more elements
/// let n = NdSliceMut::new(&mut [1, 2, 3, 4, 5, 6], [2, 4], Order::RowMajor).unwrap(); // less elements
/// ```
pub struct NdSliceMut<'s, T, const N: usize> {
    slice: &'s mut [T],
    shape: [usize; N],
    order: Order,
}

type ConstructionResult<'s, T, const N: usize> = Result<NdSliceMut<'s, T, N>, ShapeError<'s, T, N>>;

impl<'s, T, const N: usize> NdSliceMut<'s, T, N> {
    /// Creates a new `NdSliceMut` with the specified ordering from a given slice and the expected shape
    pub fn new(slice: &'s mut [T], shape: [usize; N], order: Order) -> ConstructionResult<'s, T, N> {
        if slice.len() == shape.iter().fold(1, |acc, &x| acc * x) {
            Ok(Self { slice, shape, order })
        } else {
            Err(ShapeError::new(slice, shape))
        }
    }

    /// Creates a new `NdSliceMut` with the specified ordering from a raw pointer, it's length and the expected shape
    pub unsafe fn from_ptr(ptr: *mut T, len: usize, shape: [usize; N], order: Order) -> ConstructionResult<'s, T, N> {
        NdSliceMut::new(slice::from_raw_parts_mut(ptr, len), shape, order)
    }

    /// Creates a new `NdSliceMut` with row-major ordering from a given slice and the expected shape
    pub fn new_row_ordered(slice: &'s mut [T], shape: [usize; N]) -> ConstructionResult<'s, T, N> {
        NdSliceMut::new(slice, shape, Order::RowMajor)
    }

    /// Creates a new `NdSliceMut` with row-major ordering from a raw pointer, it's length and the expected shape
    pub unsafe fn row_ordered_from_ptr(ptr: *mut T, len: usize, shape: [usize; N]) -> ConstructionResult<'s, T, N> {
        NdSliceMut::from_ptr(ptr, len, shape, Order::RowMajor)
    }

    /// Creates a new `NdSliceMut` with column-major ordering from a given slice and the expected shape
    pub fn col_ordered(slice: &'s mut [T], shape: [usize; N]) -> ConstructionResult<'s, T, N> {
        NdSliceMut::new(slice, shape, Order::ColumnMajor)
    }

    /// Creates a new `NdSliceMut` with column-major ordering from a raw pointer, it's length and the expected shape.
    pub unsafe fn col_ordered_from_ptr(ptr: *mut T, len: usize, shape: [usize; N]) -> ConstructionResult<'s, T, N> {
        NdSliceMut::from_ptr(ptr, len, shape, Order::ColumnMajor)
    }
}

impl<T, const N: usize> Index<[usize; N]> for NdSliceMut<'_, T, N> {
    type Output = T;

    fn index(&self, index: [usize; N]) -> &Self::Output {
        &self.slice[address(&self.order, &self.shape, &index)]
    }
}

impl<T, const N: usize> IndexMut<[usize; N]> for NdSliceMut<'_, T, N> {
    fn index_mut(&mut self, index: [usize; N]) -> &mut Self::Output {
        &mut self.slice[address(&self.order, &self.shape, &index)]
    }
}
