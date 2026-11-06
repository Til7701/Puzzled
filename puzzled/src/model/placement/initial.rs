use puzzle_config::{PuzzleConfig, TileConfig};
use puzzled_common::polyform::coord::coord_box::CoordBox;
use puzzled_common::polyform::coord::regular::RegularCoord;
use puzzled_common::polyform::poly::Poly;

pub fn calculate_tile_start_positions(
    tiles: &[TileConfig],
    puzzle_config: &PuzzleConfig,
    board_offset_cells: RegularCoord,
) -> Vec<RegularCoord> {
    let mut positions: Vec<RegularCoord> = Vec::new();

    let board_dim = match puzzle_config.board_config().layout().dim() {
        CoordBox::Regular(r) => r,
        CoordBox::Hex(_) => unreachable!(),
    };

    // Left
    place_in_column(
        &RegularCoord::new(1, 1),
        board_dim.y(),
        tiles,
        &mut positions,
    );

    // Right
    place_in_column(
        &(board_offset_cells + RegularCoord::new(board_dim.x() + 1, 0)),
        board_dim.y(),
        tiles,
        &mut positions,
    );

    // Bottom Rows
    let mut start_y = 2 + board_dim.y();
    while tiles.len() > positions.len() {
        let end = place_in_row(
            &RegularCoord::new(1, start_y),
            board_offset_cells.x() * 2 + board_dim.x(),
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
    start: &RegularCoord,
    end: u32,
    tiles: &[TileConfig],
    positions: &mut Vec<RegularCoord>,
) {
    if tiles.len() != positions.len() {
        let mut next_pos = start.clone();
        let mut next_tile_index = positions.len();
        while end > next_pos.y() {
            positions.push(next_pos.clone());
            if tiles.len() == positions.len() {
                break;
            }
            let tile = &tiles[next_tile_index];
            let tile_dim = match tile.base().dim() {
                CoordBox::Regular(r) => r,
                CoordBox::Hex(_) => unreachable!(),
            };
            next_pos = next_pos.with_y(next_pos.y() + tile_dim.y());
            next_tile_index += 1;
        }
    }
}

fn place_in_row(
    start: &RegularCoord,
    end: u32,
    tiles: &[TileConfig],
    positions: &mut Vec<RegularCoord>,
) -> u32 {
    let mut highest_tile: u32 = 0;
    if tiles.len() != positions.len() {
        let mut next_pos = start.clone();
        let mut next_tile_index = positions.len();
        let mut tile = &tiles[next_tile_index];
        let mut tile_dim = match tile.base().dim() {
            CoordBox::Regular(r) => r,
            CoordBox::Hex(_) => unreachable!(),
        };
        while end > next_pos.x() + tile_dim.x() {
            positions.push(next_pos.clone());
            if tiles.len() == positions.len() {
                break;
            }
            next_pos = next_pos.with_x(next_pos.x() + tile_dim.x());
            if tile_dim.y() > highest_tile {
                highest_tile = tile_dim.y();
            }
            next_tile_index += 1;
            tile = &tiles[next_tile_index];
            tile_dim = match tile.base().dim() {
                CoordBox::Regular(r) => r,
                CoordBox::Hex(_) => unreachable!(),
            }
        }
    }
    start.y() + highest_tile
}
