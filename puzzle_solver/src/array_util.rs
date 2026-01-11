use log::debug;
use ndarray::{s, Array2, Axis};

pub fn rotate_90(array: &Array2<bool>) -> Array2<bool> {
    let mut array = array.clone().reversed_axes();
    array.invert_axis(Axis(1));
    array
}

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

pub fn remove_parent(parent: &Array2<bool>, child: &mut Array2<bool>) {
    for row in 0..parent.nrows() {
        for col in 0..parent.ncols() {
            if parent[[row, col]] {
                child[[row, col]] = false;
            }
        }
    }
}

pub fn debug_print(array: &Array2<bool>) {
    for col in array.columns() {
        let row_str: String = col
            .iter()
            .map(|&cell| if cell { '█' } else { '░' })
            .collect();
        debug!("{}", row_str);
    }
}

#[cfg(test)]
mod test {
    use crate::array_util::{remove_true_rows_cols_from_sides, rotate_90};
    use ndarray::{arr2, Array2};

    #[test]
    fn test_rotate_90_size_1() {
        dbg!("test_rotate_90_size_1");
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
}
