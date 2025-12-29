use ndarray::Array2;

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

pub fn transform(array: &Array2<bool>) -> Array2<bool> {
    let (rows, cols) = array.dim();
    let mut rotated = Array2::from_elem((cols, rows), false);
    for r in 0..rows {
        for c in 0..cols {
            rotated[[c, r]] = array[[r, c]];
        }
    }
    rotated
}

