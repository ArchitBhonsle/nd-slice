#![warn(missing_debug_implementations, missing_docs)]

//! `nd-slice` wraps `std::slice` to represent n-dimensional arrays

pub mod errors;

mod addressing;
mod nd_slice;
mod nd_slice_mut;

pub use crate::addressing::Order;
pub use crate::nd_slice::NdSlice;
pub use crate::nd_slice_mut::NdSliceMut;
