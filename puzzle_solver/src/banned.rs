use crate::array_util;
use crate::bitmask::Bitmask;
use crate::board::Board;
use crate::tile::Tile;
use ndarray::{arr2, Array2};
use std::collections::HashSet;
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
    positioned_tiles: &[Tile],
) -> HashSet<BannedBitmask> {
    let min_tile_size = positioned_tiles
        .iter()
        .map(|tile| tile.base.iter().filter(|&&cell| cell).count())
        .min()
        .unwrap_or(0);

    let mut banned_bitmasks = HashSet::new();

    if min_tile_size == 0 {
        // No tiles
        return banned_bitmasks;
    } else if min_tile_size == 1 {
        // Tiles of size 1 can fill any gaps
        return banned_bitmasks;
    } else {
        banned_bitmasks.extend(banned_bitmasks_1x1(board));
        banned_bitmasks.extend(banned_bitmasks_1x2(board));
        // banned_bitmasks.extend(banned_bitmasks_2x2_corner(board)); TODO this causes the solver to always fail
        banned_bitmasks.extend(banned_bitmasks_2x2(board));
    }

    banned_bitmasks
}

fn banned_bitmasks_1x1(board: &Board) -> HashSet<BannedBitmask> {
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
    banned_bitmasks_with(board, &pattern, &area)
}

fn banned_bitmasks_1x2(board: &Board) -> HashSet<BannedBitmask> {
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
    banned_bitmasks_with_all_rotations(board, &pattern, &area)
}

fn banned_bitmasks_2x2_corner(board: &Board) -> HashSet<BannedBitmask> {
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
    banned_bitmasks_with_all_rotations(board, &pattern, &area)
}

fn banned_bitmasks_2x2(board: &Board) -> HashSet<BannedBitmask> {
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
    banned_bitmasks_with(board, &pattern, &area)
}

fn banned_bitmasks_with_all_rotations(
    board: &Board,
    pattern: &Array2<bool>,
    area: &Array2<bool>,
) -> HashSet<BannedBitmask> {
    let mut banned_bitmasks = HashSet::new();
    let mut current_pattern = pattern.clone();
    let mut current_area = area.clone();

    for _ in 0..4 {
        let new_banned_bitmasks = banned_bitmasks_with(board, &current_pattern, &current_area);
        banned_bitmasks.extend(new_banned_bitmasks);

        current_pattern = array_util::rotate_90(&current_pattern);
        current_area = array_util::rotate_90(&current_area);
    }

    banned_bitmasks
}

fn banned_bitmasks_with(
    board: &Board,
    pattern: &Array2<bool>,
    area: &Array2<bool>,
) -> HashSet<BannedBitmask> {
    let mut banned_bitmasks = HashSet::new();

    for x in 0..board.get_array().dim().0 {
        for y in 0..board.get_array().dim().1 {
            if !board.get_array()[(x, y)] {
                let banned_bitmask = create_banned_bitmask_for_pattern_at(
                    &pattern,
                    &area,
                    x as isize - 1,
                    y as isize - 1,
                    board,
                );
                banned_bitmasks.insert(banned_bitmask);
            }
        }
    }

    banned_bitmasks
}

fn create_banned_bitmask_for_pattern_at(
    pattern: &Array2<bool>,
    area: &Array2<bool>,
    x: isize,
    y: isize,
    board: &Board,
) -> BannedBitmask {
    let board_array = board.get_array().clone();

    let pattern_board = array_util::or_arrays_at(&board_array, pattern, x, y);
    let pattern_bitmask = Bitmask::from(&pattern_board);

    let area_board = array_util::or_arrays_at(&board_array, area, x, y);
    let area_bitmask = Bitmask::from(&area_board);

    BannedBitmask {
        pattern: pattern_bitmask,
        area: area_bitmask,
    }
}
