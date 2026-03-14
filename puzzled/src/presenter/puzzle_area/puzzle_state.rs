use crate::global::state::PuzzleTypeExtension;
use crate::model::puzzle::PuzzleModel;
use crate::offset::CellOffset;
use ndarray::Array2;
use std::collections::HashSet;

/// Represents data associated with a cell in the puzzle grid.
#[derive(Default, Debug)]
pub struct CellData {
    /// Indicates whether the cell is part of the playable board area.
    pub is_on_board: bool,
    /// Indicates whether placing a tile in this cell is allowed.
    pub allowed: bool,
}

/// Represents the presence of a cell of a tile in the puzzle grid.
///
/// The tile_id is used to identify which tile is present, and the cell_position indicates
/// the position of the cell of the tile inside the tile.
#[derive(Debug)]
pub struct TileCellPlacement {
    pub tile_id: usize,
    /// The position of the cell of the tile inside the tile.
    pub cell_position: CellOffset,
}

/// Represents a cell in the puzzle grid.
///
/// It can be empty, contain one tile id, or contain multiple tile ids.
///
/// A cell is not always a part of the playable board area.
/// It may be part of the border area used to indicate out-of-bounds or the board design blocks
/// placing a tile there.
#[derive(Debug)]
pub enum Cell {
    Empty(CellData),
    One(CellData, TileCellPlacement),
    Many(CellData, Vec<TileCellPlacement>),
}

impl Default for Cell {
    fn default() -> Self {
        Cell::Empty(CellData::default())
    }
}

/// Represents a tile that has not been placed on the puzzle grid.
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct UnusedTile {
    /// Used to identify the tile when having multiple identical tiles.
    pub id: usize,
    pub base: Array2<bool>,
}

/// Represents the current state of the puzzle.
///
/// The grid contains information about each cell, and unused_tiles keeps track of tiles that have
/// not been placed yet.
#[derive(Debug)]
pub struct PuzzleState {
    pub grid: Array2<Cell>,
    pub unused_tiles: HashSet<UnusedTile>,
}

impl PuzzleState {
    pub fn new(
        puzzle_config: &PuzzleModel,
        puzzle_type_extension: &Option<PuzzleTypeExtension>,
    ) -> Self {
        let board_config = &puzzle_config.board_config();
        let layout = &board_config.layout();

        let dim = layout.dim();
        // Add border to have a zone where tiles are not allowed to be placed to indicate out-of-bounds
        let dim = (dim.0 + 2, dim.1 + 2);
        let mut grid: Array2<Cell> = Array2::default(dim);

        for ((x, y), cell) in grid.indexed_iter_mut() {
            let board_index: (i32, i32) = (x as i32 - 1, y as i32 - 1);
            let on_board = *layout
                .get((board_index.0 as usize, board_index.1 as usize))
                .unwrap_or(&false);
            let is_adjacent = Self::is_adjacent_to_board(board_index, puzzle_config);
            let allowed = !is_adjacent;
            *cell = Cell::Empty(CellData {
                is_on_board: on_board,
                allowed,
            });
        }

        let mut puzzle_state = PuzzleState {
            grid,
            unused_tiles: HashSet::new(),
        };
        if let Some(extension) = puzzle_type_extension {
            puzzle_state.handle_extension(extension);
        }
        puzzle_state
    }

    fn is_adjacent_to_board(position: (i32, i32), puzzle_config: &PuzzleModel) -> bool {
        const DELTAS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        let board_config = puzzle_config.board_config();
        let this_is_on_board = board_config
            .layout()
            .get::<(usize, usize)>((position.0 as usize, position.1 as usize))
            .unwrap_or(&false);
        for (dr, dc) in DELTAS.iter() {
            let neighbor_pos = ((position.0 + dr) as usize, (position.1 + dc) as usize);
            if let Some(neighbour_on_board) = puzzle_config
                .board_config()
                .layout()
                .get::<(usize, usize)>(neighbor_pos)
                && !this_is_on_board
                && *neighbour_on_board
            {
                return true;
            }
        }
        false
    }

    fn handle_extension(&mut self, puzzle_type_extension: &PuzzleTypeExtension) {
        if let PuzzleTypeExtension::Area {
            target: Some(target),
        } = puzzle_type_extension
        {
            for index in &target.indices {
                let cell = self.grid.get_mut((index.0 + 1, index.1 + 1));
                if let Some(cell) = cell {
                    let data = match cell {
                        Cell::Empty(data) => data,
                        Cell::One(data, _) => data,
                        Cell::Many(data, _) => data,
                    };
                    data.allowed = false;
                    data.is_on_board = false;
                }
            }
        }
    }
}
