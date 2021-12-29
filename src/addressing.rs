//! Different ways to index a n-dimensional array
//!
//! https://en.wikipedia.org/wiki/Row-_and_column-major_order#Address_calculation_in_general

#[derive(Debug, Clone)]
/// Enum to indicate the underlying memory order of the `NdSlice` or `NdSliceMut`
///
/// In row-major order, the last dimension is contiguous; in column-major order, the first
/// dimension is contiguous. Read more: https://en.wikipedia.org/wiki/Row-_and_column-major_order
pub enum Order {
    RowMajor,
    ColumnMajor,
}

/// Given the shape and index calculate the "address" for a given memory order.
pub fn address(order: &Order, shape: &[usize], index: &[usize]) -> usize {
    match order {
        Order::RowMajor => row_major_address(shape, index),
        Order::ColumnMajor => col_major_address(shape, index),
    }

}

fn row_major_address(shape: &[usize], index: &[usize]) -> usize {
    let d = {
        assert_eq!(shape.len(), index.len());
        shape.len()
    };

    let mut res = index[0];
    for i in 1..d {
        res = res * shape[i] + index[i];
    }

    res
}

fn col_major_address(shape: &[usize], index: &[usize]) -> usize {
    let d = {
        assert_eq!(shape.len(), index.len());
        shape.len()
    };

    let mut res = index[d - 1];
    for i in (0..(d - 1)).rev() {
        res = res * shape[i] + index[i];
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn row_major_test_2d() {
        assert_eq!(row_major(&[2, 3], &[0, 0]), 0);
        assert_eq!(row_major(&[2, 3], &[0, 1]), 1);
        assert_eq!(row_major(&[2, 3], &[0, 2]), 2);
        assert_eq!(row_major(&[2, 3], &[1, 0]), 3);
        assert_eq!(row_major(&[2, 3], &[1, 1]), 4);
        assert_eq!(row_major(&[2, 3], &[1, 2]), 5);
    }

    #[test]
    fn col_major_test_2d() {
        assert_eq!(col_major(&[2, 3], &[0, 0]), 0);
        assert_eq!(col_major(&[2, 3], &[1, 0]), 1);
        assert_eq!(col_major(&[2, 3], &[0, 1]), 2);
        assert_eq!(col_major(&[2, 3], &[1, 1]), 3);
        assert_eq!(col_major(&[2, 3], &[0, 2]), 4);
        assert_eq!(col_major(&[2, 3], &[1, 2]), 5);
    }
}

