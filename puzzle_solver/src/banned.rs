use crate::array_util;
use crate::bitmask::Bitmask;
use crate::board::Board;
use crate::tile::Tile;
use log::debug;
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

    if min_tile_size > 1 {
        banned_bitmasks.extend(banned_bitmasks_1x1(board));
    }
    if min_tile_size > 2 {
        banned_bitmasks.extend(banned_bitmasks_1x2(board));
    }
    if min_tile_size > 3 {
        banned_bitmasks.extend(banned_bitmasks_2x2_corner(board));
        banned_bitmasks.extend(banned_bitmasks_1x3(board));
    }
    if min_tile_size > 4 {
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
    banned_bitmasks_with_all_rotations(board, pattern, area)
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
    banned_bitmasks_with_all_rotations(board, pattern, area)
}

fn banned_bitmasks_1x3(board: &Board) -> HashSet<BannedBitmask> {
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
    banned_bitmasks_with_all_rotations(board, pattern, area)
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
    pattern: Array2<bool>,
    area: Array2<bool>,
) -> HashSet<BannedBitmask> {
    debug!("Pattern:");
    array_util::debug_print(&pattern);
    debug!("Area:");
    array_util::debug_print(&area);

    let mut banned_bitmasks = HashSet::new();
    let mut current_pattern = pattern;
    let mut current_area = area;

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

    if pattern.dim().0 > board.get_array().dim().0 || pattern.dim().1 > board.get_array().dim().1 {
        return banned_bitmasks;
    }

    for x in 0..board.get_array().dim().0 - pattern.dim().0 {
        for y in 0..board.get_array().dim().1 - pattern.dim().1 {
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
    let mut board_array = board.get_array().clone();
    board_array.fill(false);

    let pattern_board = array_util::or_arrays_at(&board_array, pattern, x, y);
    let pattern_bitmask = Bitmask::from(&pattern_board);

    let area_board = array_util::or_arrays_at(&board_array, area, x, y);
    let area_bitmask = Bitmask::from(&area_board);

    if log::log_enabled!(log::Level::Debug) {
        debug!("pattern_board:");
        array_util::debug_print(&pattern_board);
        debug!("area_board:");
        array_util::debug_print(&area_board);
    }

    BannedBitmask {
        pattern: pattern_bitmask,
        area: area_bitmask,
    }
}
