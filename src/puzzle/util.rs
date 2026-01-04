use ndarray::Array2;
use std::mem::take;

pub fn rotate_90(array: &Array2<bool>) -> Array2<bool> {
    let (rows, cols) = array.dim();
    let mut rotated = Array2::from_elem((cols, rows), false);
    for r in 0..rows {
        for c in 0..cols {
            rotated[[c, rows - 1 - r]] = array[[r, c]];
        }
    }
    rotated
}

pub fn transform<T: Default + Clone>(array: &mut Array2<T>) -> Array2<T> {
    let (rows, cols) = array.dim();
    let mut rotated = Array2::from_elem((cols, rows), T::default());
    for r in 0..rows {
        for c in 0..cols {
            let value = take(&mut array[[r, c]]);
            rotated[[c, r]] = value;
        }
    }
    rotated
}
