#![warn(missing_debug_implementations, missing_docs)]

//! `nd-slice` wraps `std::slice` to represent n-dimensional arrays

pub mod errors;
pub mod nd_slice;
mod addressing;

