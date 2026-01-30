use log::debug;
use ndarray::{s, Array2, Axis};

/// Rotates a 2D boolean array 90 degrees clockwise and returns the new array.
///
/// This is a convenience function to allow semantically clearer code when rotating arrays.
///
/// # Arguments
///
/// * `array`: The rotated 2D boolean array.
///
/// returns: ArrayBase<OwnedRepr<bool>, Dim<[usize; 2]>, <OwnedRepr<bool> as RawData>::Elem>
pub(crate) fn rotate_90<T: Clone>(array: &Array2<T>) -> Array2<T> {
    let mut array = array.clone().reversed_axes();
    array.invert_axis(Axis(1));
    array
}

/// Removes rows and columns from the sides of a 2D boolean array where all cells are `true`.
///
/// # Arguments
///
/// * `array`: The mutable reference to the 2D boolean array to be modified.
///
/// returns: ()
pub fn remove_true_rows_cols_from_sides(array: &mut Array2<bool>) {
    loop {
        if array.nrows() == 0 || array.ncols() == 0 {
            break;
        }

        let left_col_all_true = array.column(0).iter().all(|&cell| cell);
        if left_col_all_true {
            *array = array.slice(s![.., 1..]).to_owned();
            continue;
        }

        let right_col_all_true = array.column(array.ncols() - 1).iter().all(|&cell| cell);
        if right_col_all_true {
            *array = array.slice(s![.., ..array.ncols() - 1]).to_owned();
            continue;
        }

        let top_row_all_true = array.row(0).iter().all(|&cell| cell);
        if top_row_all_true {
            *array = array.slice(s![1.., ..]).to_owned();
            continue;
        }

        let bottom_row_all_true = array.row(array.nrows() - 1).iter().all(|&cell| cell);
        if bottom_row_all_true {
            *array = array.slice(s![..array.nrows() - 1, ..]).to_owned();
            continue;
        }

        break;
    }
}

/// Places the `child` array onto the `parent` array at the specified offsets using a logical OR
/// operation.
/// This means that if either the parent or child cell is `true`, the resulting cell will be `true`.
///
/// # Arguments
///
/// * `parent`: The parent 2D boolean array.
/// * `child`: The child 2D boolean array to be placed onto the parent.
/// * `x_offset`: The x-axis offset for placing the child array.
/// * `y_offset`: The y-axis offset for placing the child array.
///
/// returns: Array2<bool>
pub fn or_arrays_at(
    parent: &Array2<bool>,
    child: &Array2<bool>,
    x_offset: isize,
    y_offset: isize,
) -> Array2<bool> {
    let mut new_array = parent.clone();
    let child_xs = child.nrows();
    let child_ys = child.ncols();

    for x in 0..child_xs {
        for y in 0..child_ys {
            let parent_x = x as isize + x_offset;
            let parent_y = y as isize + y_offset;
            if parent_x >= 0
                && parent_x < parent.nrows() as isize
                && parent_y >= 0
                && parent_y < parent.ncols() as isize
            {
                new_array[[parent_x as usize, parent_y as usize]] |= child[[x, y]];
            }
        }
    }

    new_array
}

/// Generates all possible placements of the `child` array onto the `parent` array
/// using a logical OR operation.
///
/// # Arguments
///
/// * `parent`: The parent 2D boolean array.
/// * `child`: The child 2D boolean array to be placed onto the parent.
///
/// returns: Vec<Array2<bool>>
pub fn place_on_all_positions(parent: &Array2<bool>, child: &Array2<bool>) -> Vec<Array2<bool>> {
    let mut placements = Vec::new();
    let parent_rows = parent.nrows();
    let parent_cols = parent.ncols();
    let child_rows = child.nrows();
    let child_cols = child.ncols();

    if child_rows > parent_rows || child_cols > parent_cols {
        return placements;
    }

    for row_offset in 0..=(parent_rows - child_rows) {
        for col_offset in 0..=(parent_cols - child_cols) {
            let mut new_array = parent.clone();
            let mut valid = true;
            for r in 0..child_rows {
                for c in 0..child_cols {
                    if child[[r, c]] && parent[[row_offset + r, col_offset + c]] {
                        valid = false;
                        break;
                    }
                    new_array[[row_offset + r, col_offset + c]] |= child[[r, c]];
                }
                if !valid {
                    break;
                }
            }
            if valid {
                placements.push(new_array);
            }
        }
    }

    placements
}

/// Removes the `true` values from the `child` array wherever the `parent` array has `true` values.
///
/// # Arguments
///
/// * `parent`: The parent 2D boolean array.
/// * `child`: The mutable reference to the child 2D boolean array to be modified.
///
/// returns: ()
pub fn remove_parent(parent: &Array2<bool>, child: &mut Array2<bool>) {
    for row in 0..parent.nrows() {
        for col in 0..parent.ncols() {
            if parent[[row, col]] {
                child[[row, col]] = false;
            }
        }
    }
}

/// Prints a 2D boolean array to the debug log, using '█' for `true` and '░' for `false`.
#[allow(dead_code)]
pub fn debug_print(array: &Array2<bool>) {
    for row in array.rows() {
        let row_str: String = row
            .iter()
            .map(|&cell| if cell { '#' } else { '-' })
            .collect();
        debug!("{}", row_str);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use ndarray::{arr2, Array2};

    #[test]
    fn test_rotate_90_size_1() {
        let array = arr2(&[[true]]);
        let rotated = rotate_90(&array);
        let expected = arr2(&[[true]]);
        assert_eq!(expected, rotated);
    }

    #[test]
    fn test_rotate_90_size_2() {
        let array = arr2(&[[true, false]]);
        let rotated = rotate_90(&array);
        let expected = arr2(&[[true], [false]]);
        assert_eq!(expected, rotated);
    }

    #[test]
    fn test_rotate_90() {
        let array = arr2(&[
            [true, false, false],
            [true, true, true],
            [true, false, true],
        ]);
        let rotated = rotate_90(&array);
        let expected = arr2(&[
            [true, true, true],
            [false, true, false],
            [true, true, false],
        ]);
        assert_eq!(expected, rotated);
    }

    #[test]
    fn test_remove_true_rows_cols_from_sides_empty() {
        let mut array: Array2<bool> = Array2::default((0, 0));
        remove_true_rows_cols_from_sides(&mut array);
        let expected: Array2<bool> = Array2::default((0, 0));
        assert_eq!(expected, array);
    }

    #[test]
    fn test_remove_true_rows_cols_from_sides_true() {
        let mut array = arr2(&[[true]]);
        remove_true_rows_cols_from_sides(&mut array);
        let expected: Array2<bool> = arr2(&[[]]);
        assert_eq!(expected, array);
    }

    #[test]
    fn test_remove_true_rows_cols_from_sides_false() {
        let mut array = arr2(&[[false]]);
        remove_true_rows_cols_from_sides(&mut array);
        let expected: Array2<bool> = arr2(&[[false]]);
        assert_eq!(expected, array);
    }

    #[test]
    fn test_remove_true_rows_cols_from_sides_left_right() {
        let mut array = arr2(&[
            [true, true, false, true],
            [true, false, false, true],
            [true, true, false, true],
        ]);
        remove_true_rows_cols_from_sides(&mut array);
        let expected = arr2(&[[true, false], [false, false], [true, false]]);
        assert_eq!(expected, array);
    }

    #[test]
    fn test_remove_true_rows_cols_from_sides_top_bottom() {
        let mut array = arr2(&[
            [true, true, true, true],
            [false, true, false, false],
            [true, true, true, true],
        ]);
        remove_true_rows_cols_from_sides(&mut array);
        let expected = arr2(&[[false, true, false, false]]);
        assert_eq!(expected, array);
    }

    #[test]
    fn test_remove_true_rows_cols_from_sides_all_sides() {
        let mut array = arr2(&[
            [true, true, true, true, true],
            [true, true, false, false, true],
            [true, false, true, false, true],
            [true, true, true, true, true],
        ]);
        remove_true_rows_cols_from_sides(&mut array);
        let expected = arr2(&[[true, false, false], [false, true, false]]);
        assert_eq!(expected, array);
    }

    #[test]
    fn test_remove_true_rows_cols_from_left() {
        let mut array = arr2(&[[true, true, false, false], [true, false, true, false]]);
        remove_true_rows_cols_from_sides(&mut array);
        let expected = arr2(&[[true, false, false], [false, true, false]]);
        assert_eq!(expected, array);
    }

    #[test]
    fn test_remove_true_rows_cols_from_right() {
        let mut array = arr2(&[[false, false, true, true], [false, true, false, true]]);
        remove_true_rows_cols_from_sides(&mut array);
        let expected = arr2(&[[false, false, true], [false, true, false]]);
        assert_eq!(expected, array);
    }

    #[test]
    fn test_remove_true_rows_cols_from_top() {
        let mut array = arr2(&[
            [true, true, true],
            [false, true, false],
            [true, false, true],
        ]);
        remove_true_rows_cols_from_sides(&mut array);
        let expected = arr2(&[[false, true, false], [true, false, true]]);
        assert_eq!(expected, array);
    }

    #[test]
    fn test_remove_true_rows_cols_from_bottom() {
        let mut array = arr2(&[
            [false, true, false],
            [true, false, true],
            [true, true, true],
        ]);
        remove_true_rows_cols_from_sides(&mut array);
        let expected = arr2(&[[false, true, false], [true, false, true]]);
        assert_eq!(expected, array);
    }

    #[test]
    fn test_remove_true_rows_cols_test() {
        let mut array = arr2(&[
            [false, false, false, false],
            [false, false, false, false],
            [true, true, true, true],
            [false, true, false, true],
            [true, true, true, true],
        ]);
        remove_true_rows_cols_from_sides(&mut array);
        let expected = arr2(&[
            [false, false, false, false],
            [false, false, false, false],
            [true, true, true, true],
            [false, true, false, true],
        ]);
        assert_eq!(expected, array);
    }

    #[test]
    fn test_or_arrays_at() {
        let parent = arr2(&[
            [false, false, false],
            [false, false, false],
            [false, false, false],
        ]);
        let child = arr2(&[[true, false], [false, true]]);
        let result = or_arrays_at(&parent, &child, 1, 1);
        let expected = arr2(&[
            [false, false, false],
            [false, true, false],
            [false, false, true],
        ]);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_or_arrays_at_empty_child() {
        let parent = arr2(&[
            [false, false, false],
            [false, false, false],
            [false, false, false],
        ]);
        let child = arr2(&[[]]);
        let result = or_arrays_at(&parent, &child, 1, 1);
        let expected = arr2(&[
            [false, false, false],
            [false, false, false],
            [false, false, false],
        ]);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_or_arrays_at_child_1x1() {
        let parent = arr2(&[
            [false, false, false],
            [false, false, false],
            [false, false, false],
        ]);
        let child = arr2(&[[true]]);
        let result = or_arrays_at(&parent, &child, 1, 1);
        let expected = arr2(&[
            [false, false, false],
            [false, true, false],
            [false, false, false],
        ]);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_or_arrays_at_child_off_parent() {
        let parent = arr2(&[
            [false, false, false],
            [false, false, false],
            [false, false, false],
        ]);
        let child = arr2(&[[true, true], [true, true]]);
        let result = or_arrays_at(&parent, &child, 2, 2);
        let expected = arr2(&[
            [false, false, false],
            [false, false, false],
            [false, false, true],
        ]);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_or_arrays_at_true_parent() {
        let parent = arr2(&[[true, true, true], [true, true, true], [true, true, true]]);
        let child = arr2(&[[true, false], [true, false]]);
        let result = or_arrays_at(&parent, &child, 1, 1);
        let expected = arr2(&[[true, true, true], [true, true, true], [true, true, true]]);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_or_arrays_at_smaller_parent() {
        let parent = arr2(&[[false, false], [false, false]]);
        let child = arr2(&[[true, true, true], [true, true, true], [true, true, true]]);
        let result = or_arrays_at(&parent, &child, 0, 0);
        let expected = arr2(&[[true, true], [true, true]]);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_place_on_all_positions() {
        let parent = arr2(&[
            [false, false, false],
            [false, false, false],
            [false, false, false],
        ]);
        let child = arr2(&[[true, false], [false, true]]);
        let placements = place_on_all_positions(&parent, &child);
        assert_eq!(placements.len(), 4);
        assert!(placements.contains(&arr2(&[
            [true, false, false],
            [false, true, false],
            [false, false, false],
        ])));
        assert!(placements.contains(&arr2(&[
            [false, true, false],
            [false, false, true],
            [false, false, false],
        ])));
        assert!(placements.contains(&arr2(&[
            [false, false, false],
            [true, false, false],
            [false, true, false],
        ])));
        assert!(placements.contains(&arr2(&[
            [false, false, false],
            [false, true, false],
            [false, false, true],
        ])));
    }

    #[test]
    fn test_place_on_all_positions_same_size() {
        let parent = arr2(&[[false, false], [false, false]]);
        let child = arr2(&[[true, false], [false, true]]);
        let placements = place_on_all_positions(&parent, &child);
        assert_eq!(placements.len(), 1);
        assert!(placements.contains(&arr2(&[[true, false], [false, true],])));
    }

    #[test]
    fn test_place_on_all_positions_smaller_parent() {
        let parent = arr2(&[[false, false], [false, false]]);
        let child = arr2(&[[true, false, true], [false, true, false]]);
        let placements = place_on_all_positions(&parent, &child);
        assert_eq!(placements.len(), 0);
    }

    #[test]
    fn test_place_on_all_positions_with_blocking() {
        let parent = arr2(&[
            [false, false, false],
            [false, true, false],
            [false, false, false],
        ]);
        let child = arr2(&[[true, false], [false, true]]);
        let placements = place_on_all_positions(&parent, &child);
        assert_eq!(placements.len(), 2);
        assert!(placements.contains(&arr2(&[
            [false, true, false],
            [false, true, true],
            [false, false, false],
        ])));
        assert!(placements.contains(&arr2(&[
            [false, false, false],
            [true, true, false],
            [false, true, false],
        ])));
    }

    #[test]
    fn test_remove_parent() {
        let parent = arr2(&[
            [true, false, true],
            [false, true, false],
            [true, true, true],
        ]);
        let mut child = arr2(&[[true, true, true], [true, true, true], [true, true, true]]);
        remove_parent(&parent, &mut child);
        let expected = arr2(&[
            [false, true, false],
            [true, false, true],
            [false, false, false],
        ]);
        assert_eq!(expected, child);
    }

    #[test]
    fn test_remove_parent_smaller_parent() {
        let parent = arr2(&[[true, false], [false, true]]);
        let mut child = arr2(&[[true, true, true], [true, true, true], [true, true, true]]);
        remove_parent(&parent, &mut child);
        let expected = arr2(&[[false, true, true], [true, false, true], [true, true, true]]);
        assert_eq!(expected, child);
    }

    #[test]
    #[should_panic]
    fn test_remove_parent_bigger_parent_panic() {
        let parent = arr2(&[
            [true, false, true],
            [false, true, false],
            [true, true, true],
        ]);
        let mut child = arr2(&[[true, true], [true, true]]);
        remove_parent(&parent, &mut child);
    }
}
