use crate::model::extension::PuzzleTypeExtension;
use puzzle_config::{BoardConfig, PuzzleConfig};
use puzzled_common::polyform::Polyform;
use puzzled_common::polyform::grid::Coord;
use std::cell::Ref;
use std::collections::HashSet;

/// Represents data associated with a cell in the puzzle grid.
#[derive(Default, Debug, Clone)]
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
#[derive(Debug, Clone)]
pub struct TileCellPlacement {
    pub tile_id: usize,
    /// The position of the cell of the tile inside the tile.
    pub cell_position: Coord,
}

/// Represents a cell in the puzzle grid.
///
/// It can be empty, contain one tile id, or contain multiple tile ids.
///
/// A cell is not always a part of the playable board area.
/// It may be part of the border area used to indicate out-of-bounds or the board design blocks
/// placing a tile there.
#[derive(Debug, Clone)]
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
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct UnusedTile {
    /// Used to identify the tile when having multiple identical tiles.
    pub id: usize,
    pub base: Polyform<()>,
    pub name: Option<String>,
}

/// Represents the current state of the puzzle.
///
/// The grid contains information about each cell, and unused_tiles keeps track of tiles that have
/// not been placed yet.
#[derive(Debug)]
pub struct PuzzleState {
    pub grid: Polyform<Cell>,
    pub unused_tiles: HashSet<UnusedTile>,
}

impl PuzzleState {
    pub fn new(
        puzzle_config: &PuzzleConfig,
        puzzle_type_extension: Ref<Option<PuzzleTypeExtension>>,
    ) -> Self {
        let board_config = &puzzle_config.board_config();

        let mut grid = match (board_config, puzzle_type_extension.as_ref()) {
            (BoardConfig::Simple { layout }, _) => {
                layout.clone().map(|_| {
                    CellData {
                        is_on_board: true,
                        allowed: true,
                    }
                })
            }
            (BoardConfig::Area { layout, .. }, Some(PuzzleTypeExtension::Area { target: Some(target) })) => {
                layout.clone().map_indexed(&|_, coord| {
                    let allowed = target.indices.iter().any(|t| t.coord() == coord);
                    CellData {
                        is_on_board: true,
                        allowed,
                    }
                })
            }
            (BoardConfig::Area { layout, .. }, _) => {
                layout.clone().map(|_| {
                    CellData {
                        is_on_board: true,
                        allowed: true,
                    }
                })
            }
        }.map(|cell_data| Cell::Empty(cell_data));
        grid.extend_adjacent(Cell::Empty(CellData {
            is_on_board: false,
            allowed: false,
        }));

        let puzzle_state = PuzzleState {
            grid,
            unused_tiles: HashSet::new(),
        };
        puzzle_state
    }
}
