use crate::global::state::PuzzleTypeExtension;
use crate::offset::CellOffset;
use gtk::Widget;
use ndarray::Array2;
use puzzle_config::PuzzleConfig;
use std::collections::HashSet;

/// Represents data associated with a cell in the puzzle grid.
#[derive(Default, Debug)]
pub struct CellData {
    /// The position of the cell in the grid.
    pub position: CellOffset,
    /// Indicates whether the cell is part of the playable board area.
    pub is_on_board: bool,
    /// Indicates whether placing a tile in this cell is allowed.
    pub allowed: bool,
}

/// Represents a cell in the puzzle grid.
///
/// It can be empty, contain one widget, or contain multiple widgets.
/// A widget is an element of a tile that occupies the cell.
///
/// A cell is not always a part of the playable board area.
/// It may be part of the border area used to indicate out-of-bounds or the board design blocks
/// placing a tile there.
#[derive(Debug)]
pub enum Cell {
    Empty(CellData),
    One(CellData, Widget),
    Many(CellData, Vec<Widget>),
}

impl Default for Cell {
    fn default() -> Self {
        Cell::Empty(CellData::default())
    }
}

/// Represents a tile that has not been placed on the puzzle grid.
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct UnusedTile {
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
        puzzle_config: &PuzzleConfig,
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
                position: CellOffset(x as i32, y as i32),
                is_on_board: on_board,
                allowed,
            });
        }

        PuzzleState {
            grid,
            unused_tiles: HashSet::new(),
        }
    }

    fn is_adjacent_to_board(position: (i32, i32), puzzle_config: &PuzzleConfig) -> bool {
        const DELTAS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        let this_is_on_board = puzzle_config
            .board_config()
            .layout()
            .get::<(usize, usize)>((position.0 as usize, position.1 as usize).into())
            .unwrap_or(&false);
        for (dr, dc) in DELTAS.iter() {
            let neighbor_pos = ((position.0 + dr) as usize, (position.1 + dc) as usize);
            if let Some(neighbour_on_board) = puzzle_config
                .board_config()
                .layout()
                .get::<(usize, usize)>(neighbor_pos.into())
            {
                if !this_is_on_board && *neighbour_on_board {
                    return true;
                }
            }
        }
        false
    }

    fn handle_extension(&mut self, puzzle_type_extension: &PuzzleTypeExtension) {
        match puzzle_type_extension {
            PuzzleTypeExtension::Area { target } => {
                for index in &target.indices {
                    let cell = self.grid.get_mut((index.0, index.1));
                    if let Some(cell) = cell {
                        let data = match cell {
                            Cell::Empty(data) => data,
                            Cell::One(data, _) => data,
                            Cell::Many(data, _) => data,
                        };
                        data.allowed = false;
                    }
                }
            }
            _ => {}
        }
    }
}
