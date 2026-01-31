use crate::array_util;
use crate::bitmask::Bitmask;
use crate::board::Board;
use crate::tile::Tile;
use ndarray::{arr2, Array2};
use std::hash::Hash;

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct BannedBitmask {
    pattern: Bitmask,
    area: Bitmask,
}

impl BannedBitmask {
    pub fn matches(&self, bitmask: &Bitmask) -> bool {
        Bitmask::and_equals(bitmask, &self.area, &self.pattern)
    }
}

pub fn create_banned_bitmasks_for_filling(
    board: &Board,
    tiles: &[Tile],
) -> Vec<Vec<BannedBitmask>> {
    let min_tile_size = tiles
        .iter()
        .map(|tile| tile.base.iter().filter(|&&cell| cell).count())
        .min()
        .unwrap_or(0);

    let mut banned_bitmasks = Vec::with_capacity(board.get_array().len());
    for _ in 0..board.get_array().len() {
        banned_bitmasks.push(Vec::with_capacity(0));
    }
    let (xs, ys) = board.get_array().dim();
    for x in 0..ys {
        for y in 0..xs {
            if !board[[y, x]] {
                // This index has to match the index in the Bitmask. See Bitmask::from(&Array2<bool>)
                let index = x * xs + y;
                let mut banned_bitmasks_for_cell =
                    create_banned_bitmasks_for_cell(board, y, x, min_tile_size);
                banned_bitmasks_for_cell.shrink_to_fit();
                banned_bitmasks.insert(index, banned_bitmasks_for_cell);
            }
        }
    }
    banned_bitmasks
}

fn create_banned_bitmasks_for_cell(
    board: &Board,
    x: usize,
    y: usize,
    min_tile_size: usize,
) -> Vec<BannedBitmask> {
    let mut banned_bitmasks = Vec::new();

    if min_tile_size > 1 {
        banned_bitmasks.extend(banned_bitmasks_1(board, x, y));
    }
    if min_tile_size > 2 {
        banned_bitmasks.extend(banned_bitmasks_D2(board, x, y));
    }
    if min_tile_size > 3 {
        banned_bitmasks.extend(banned_bitmasks_L3(board, x, y));
        banned_bitmasks.extend(banned_bitmasks_I3(board, x, y));
    }
    if min_tile_size > 4 {
        banned_bitmasks.extend(banned_bitmasks_O4(board, x, y));
    }
    banned_bitmasks
}

fn banned_bitmasks_1(board: &Board, x: usize, y: usize) -> Vec<BannedBitmask> {
    let pattern = arr2(&[
        [false, true, false],
        [true, false, true],
        [false, true, false],
    ]);
    let area = arr2(&[
        [false, true, false],
        [true, true, true],
        [false, true, false],
    ]);
    let mut set = Vec::new();
    set.push(create_banned_bitmask_for_pattern_at(
        &pattern,
        &area,
        x as isize - 1,
        y as isize - 1,
        board,
    ));
    set
}

#[allow(non_snake_case)]
fn banned_bitmasks_D2(board: &Board, x: usize, y: usize) -> Vec<BannedBitmask> {
    let pattern = arr2(&[
        [false, true, true, false],
        [true, false, false, true],
        [false, true, true, false],
    ]);
    let area = arr2(&[
        [false, true, true, false],
        [true, true, true, true],
        [false, true, true, false],
    ]);
    let mut banned_bitmasks = Vec::with_capacity(2);
    banned_bitmasks.push(create_banned_bitmask_for_pattern_at(
        &pattern,
        &area,
        x as isize - 1,
        y as isize - 1,
        board,
    ));
    let pattern = array_util::rotate_90(&pattern);
    let area = array_util::rotate_90(&area);
    banned_bitmasks.push(create_banned_bitmask_for_pattern_at(
        &pattern,
        &area,
        x as isize - 1,
        y as isize - 1,
        board,
    ));
    banned_bitmasks
}

#[allow(non_snake_case)]
fn banned_bitmasks_L3(board: &Board, x: usize, y: usize) -> Vec<BannedBitmask> {
    let pattern = arr2(&[
        [false, true, true, false],
        [true, false, false, true],
        [true, false, true, false],
        [false, true, false, false],
    ]);
    let area = arr2(&[
        [false, true, true, false],
        [true, true, true, true],
        [true, true, true, false],
        [false, true, false, false],
    ]);
    let mut banned_bitmasks = Vec::with_capacity(4);
    let area = array_util::rotate_90(&area);
    banned_bitmasks.push(create_banned_bitmask_for_pattern_at(
        &pattern,
        &area,
        x as isize - 1,
        y as isize - 1,
        board,
    ));
    let pattern = array_util::rotate_90(&pattern);
    let area = array_util::rotate_90(&area);
    let area = array_util::rotate_90(&area);
    banned_bitmasks.push(create_banned_bitmask_for_pattern_at(
        &pattern,
        &area,
        x as isize - 1,
        y as isize - 1,
        board,
    ));
    let pattern = array_util::rotate_90(&pattern);
    let area = array_util::rotate_90(&area);
    let area = array_util::rotate_90(&area);
    banned_bitmasks.push(create_banned_bitmask_for_pattern_at(
        &pattern,
        &area,
        x as isize - 2,
        y as isize - 1,
        board,
    ));
    let pattern = array_util::rotate_90(&pattern);
    let area = array_util::rotate_90(&area);
    let area = array_util::rotate_90(&area);
    banned_bitmasks.push(create_banned_bitmask_for_pattern_at(
        &pattern,
        &area,
        x as isize - 1,
        y as isize - 1,
        board,
    ));
    banned_bitmasks
}

#[allow(non_snake_case)]
fn banned_bitmasks_I3(board: &Board, x: usize, y: usize) -> Vec<BannedBitmask> {
    let pattern = arr2(&[
        [false, true, true, true, false],
        [true, false, false, false, true],
        [false, true, true, true, false],
    ]);
    let area = arr2(&[
        [false, true, true, true, false],
        [true, true, true, true, true],
        [false, true, true, true, false],
    ]);
    let mut banned_bitmasks = Vec::with_capacity(2);
    banned_bitmasks.push(create_banned_bitmask_for_pattern_at(
        &pattern,
        &area,
        x as isize - 1,
        y as isize - 1,
        board,
    ));
    let pattern = array_util::rotate_90(&pattern);
    let area = array_util::rotate_90(&area);
    banned_bitmasks.push(create_banned_bitmask_for_pattern_at(
        &pattern,
        &area,
        x as isize - 1,
        y as isize - 1,
        board,
    ));
    banned_bitmasks
}

#[allow(non_snake_case)]
fn banned_bitmasks_O4(board: &Board, x: usize, y: usize) -> Vec<BannedBitmask> {
    let pattern = arr2(&[
        [false, true, true, false],
        [true, false, false, true],
        [true, false, false, true],
        [false, true, true, false],
    ]);
    let area = arr2(&[
        [false, true, true, false],
        [true, true, true, true],
        [true, true, true, true],
        [false, true, true, false],
    ]);
    let mut banned_bitmasks = Vec::with_capacity(1);
    let area = array_util::rotate_90(&area);
    banned_bitmasks.push(create_banned_bitmask_for_pattern_at(
        &pattern,
        &area,
        x as isize - 1,
        y as isize - 1,
        board,
    ));
    banned_bitmasks
}

/// Creates a BannedBitmask for a given pattern and area at position (x, y) on the board.
/// The coordinates (x, y) represent the top-left corner where the pattern is placed.
///
/// # Arguments
///
/// * `pattern`: The pattern array where true indicates the cells occupied by the pattern.
/// * `area`: The area array where true indicates the cells that define the area of influence.
/// * `x`: The x-coordinate on the board where the pattern is placed.
/// * `y`: The y-coordinate on the board where the pattern is placed.
/// * `board`: The board on which the pattern is placed.
///
/// returns: BannedBitmask
fn create_banned_bitmask_for_pattern_at(
    pattern: &Array2<bool>,
    area: &Array2<bool>,
    x: isize,
    y: isize,
    board: &Board,
) -> BannedBitmask {
    let mut board_array = board.get_array().clone();
    board_array.fill(false);

    let pattern_board = array_util::or_arrays_at(&board_array, pattern, x, y);
    let pattern_bitmask = Bitmask::from(&pattern_board);

    let area_board = array_util::or_arrays_at(&board_array, area, x, y);
    let area_bitmask = Bitmask::from(&area_board);

    println!("Created BannedBitmask at ({}, {})", x, y);
    print!("Pattern Board:\n");
    array_util::debug_print(&pattern_board);
    print!("Area Board:\n");
    array_util::debug_print(&area_board);

    BannedBitmask {
        pattern: pattern_bitmask,
        area: area_bitmask,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::Board;
    use ndarray::arr2;

    #[test]
    fn test_create_banned_bitmask_for_pattern_at_middle() {
        let pattern = arr2(&[
            [false, true, false],
            [true, false, true],
            [false, true, false],
        ]);
        let area = arr2(&[
            [false, true, false],
            [true, true, true],
            [false, true, false],
        ]);
        let board = Board::new((5, 5));

        let banned_bitmask = create_banned_bitmask_for_pattern_at(&pattern, &area, 1, 1, &board);
        let expected_pattern_board = arr2(&[
            [false, false, false, false, false],
            [false, false, true, false, false],
            [false, true, false, true, false],
            [false, false, true, false, false],
            [false, false, false, false, false],
        ]);
        let expected_area_board = arr2(&[
            [false, false, false, false, false],
            [false, false, true, false, false],
            [false, true, true, true, false],
            [false, false, true, false, false],
            [false, false, false, false, false],
        ]);
        let expected_pattern_bitmask = Bitmask::from(&expected_pattern_board);
        let expected_area_bitmask = Bitmask::from(&expected_area_board);

        assert_eq!(banned_bitmask.pattern, expected_pattern_bitmask);
        assert_eq!(banned_bitmask.area, expected_area_bitmask);
    }

    #[test]
    fn test_create_banned_bitmask_for_pattern_at_minus1_minus1() {
        let pattern = arr2(&[
            [false, true, false],
            [true, false, true],
            [false, true, false],
        ]);
        let area = arr2(&[
            [false, true, false],
            [true, true, true],
            [false, true, false],
        ]);
        let board = Board::new((5, 5));

        let banned_bitmask = create_banned_bitmask_for_pattern_at(&pattern, &area, -1, -1, &board);
        let expected_pattern_board = arr2(&[
            [false, true, false, false, false],
            [true, false, false, false, false],
            [false, false, false, false, false],
            [false, false, false, false, false],
            [false, false, false, false, false],
        ]);
        let expected_area_board = arr2(&[
            [true, true, false, false, false],
            [true, false, false, false, false],
            [false, false, false, false, false],
            [false, false, false, false, false],
            [false, false, false, false, false],
        ]);
        let expected_pattern_bitmask = Bitmask::from(&expected_pattern_board);
        let expected_area_bitmask = Bitmask::from(&expected_area_board);

        assert_eq!(banned_bitmask.pattern, expected_pattern_bitmask);
        assert_eq!(banned_bitmask.area, expected_area_bitmask);
    }

    #[test]
    fn test_create_banned_bitmask_for_pattern_at_3_3() {
        let pattern = arr2(&[
            [false, true, false],
            [true, false, true],
            [false, true, false],
        ]);
        let area = arr2(&[
            [false, true, false],
            [true, true, true],
            [false, true, false],
        ]);
        let board = Board::new((5, 5));

        let banned_bitmask = create_banned_bitmask_for_pattern_at(&pattern, &area, 3, 3, &board);
        let expected_pattern_board = arr2(&[
            [false, false, false, false, false],
            [false, false, false, false, false],
            [false, false, false, false, false],
            [false, false, false, false, true],
            [false, false, false, true, false],
        ]);
        let expected_area_board = arr2(&[
            [false, false, false, false, false],
            [false, false, false, false, false],
            [false, false, false, false, false],
            [false, false, false, false, true],
            [false, false, false, true, true],
        ]);
        let expected_pattern_bitmask = Bitmask::from(&expected_pattern_board);
        let expected_area_bitmask = Bitmask::from(&expected_area_board);

        assert_eq!(banned_bitmask.pattern, expected_pattern_bitmask);
        assert_eq!(banned_bitmask.area, expected_area_bitmask);
    }
}
