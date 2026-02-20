use crate::offset::CellOffset;
use puzzle_config::{PuzzleConfig, TileConfig};

pub fn calculate_tile_start_positions(
    tiles: &[TileConfig],
    puzzle_config: &PuzzleConfig,
    board_offset_cells: CellOffset,
) -> Vec<CellOffset> {
    let mut positions: Vec<CellOffset> = Vec::new();

    // Left
    place_in_column(
        CellOffset(1, 1),
        puzzle_config.board_config().layout().dim().1 as i32,
        tiles,
        &mut positions,
    );

    // Right
    place_in_column(
        board_offset_cells
            + CellOffset(puzzle_config.board_config().layout().dim().0 as i32 + 1, 0),
        puzzle_config.board_config().layout().dim().1 as i32,
        tiles,
        &mut positions,
    );

    // Bottom Rows
    let mut start_y = 2 + puzzle_config.board_config().layout().dim().1 as i32;
    while tiles.len() > positions.len() {
        let end = place_in_row(
            CellOffset(1, start_y),
            board_offset_cells.0 * 2 + puzzle_config.board_config().layout().dim().0 as i32,
            tiles,
            &mut positions,
        );
        start_y = end;
    }

    if tiles.len() != positions.len() {
        panic!("Not enough space to place all tiles around the board");
    }

    positions
}

fn place_in_column(
    start: CellOffset,
    end: i32,
    tiles: &[TileConfig],
    positions: &mut Vec<CellOffset>,
) {
    if tiles.len() != positions.len() {
        let mut next_pos = start;
        let mut next_tile_index = positions.len();
        while end > next_pos.1 {
            positions.push(next_pos.clone());
            if tiles.len() == positions.len() {
                break;
            }
            let tile = &tiles[next_tile_index];
            next_pos.1 += tile.base().dim().1 as i32;
            next_tile_index += 1;
        }
    }
}

fn place_in_row(
    start: CellOffset,
    end: i32,
    tiles: &[TileConfig],
    positions: &mut Vec<CellOffset>,
) -> i32 {
    let mut highest_tile: i32 = 0;
    if tiles.len() != positions.len() {
        let mut next_pos = start;
        let mut next_tile_index = positions.len();
        while end > next_pos.0 + tiles[next_tile_index].base().dim().0 as i32 {
            positions.push(next_pos.clone());
            if tiles.len() == positions.len() {
                break;
            }
            let tile = &tiles[next_tile_index];
            next_pos.0 += tile.base().dim().0 as i32;
            next_tile_index += 1;
            if tile.base().dim().1 as i32 > highest_tile {
                highest_tile = tile.base().dim().1 as i32;
            }
        }
    }
    start.1 + highest_tile
}
