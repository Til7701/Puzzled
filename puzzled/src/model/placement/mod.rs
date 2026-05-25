use crate::app::puzzle::puzzle_area::puzzle_state::{
    Cell, PuzzleState, TileCellPlacement, UnusedTile,
};
use crate::model::extension::PuzzleTypeExtension;
use crate::model::placement::board::PlacedBoard;
use crate::model::placement::grid::{
    MIN_CELLS_TO_THE_SIDES_OF_BOARD, MIN_CELLS_TO_THE_TOP_OF_BOARD,
};
use crate::model::placement::tile::PlacedTile;
use crate::model::puzzle::PuzzleModel;
use crate::offset::{CellOffset, PixelOffset};
use adw::glib;
use adw::prelude::ObjectExt;
use adw::subclass::prelude::*;
use log::debug;
use puzzled_common::Shape;
use std::cell::Ref;
use std::mem::take;

mod board;
mod grid;
mod initial;
mod tile;

const TILE_MOVED_SIGNAL_NAME: &str = "tile-moved";

mod imp {
    use super::*;
    use crate::model::placement::board::PlacedBoard;
    use crate::model::placement::grid::GridConfig;
    use crate::model::placement::tile::PlacedTile;
    use crate::offset::PixelOffset;
    use adw::glib::subclass::Signal;
    use adw::glib::Properties;
    use std::cell::{Cell, RefCell};
    use std::sync::OnceLock;

    #[derive(Debug, Default, Properties)]
    #[properties(wrapper_type = super::PlacementModel)]
    pub struct PuzzledPlacementModel {
        pub(super) puzzle: RefCell<Option<PuzzleModel>>,
        pub(super) area_pixel_size: Cell<PixelOffset>,
        pub(super) min_area_pixel_size: Cell<PixelOffset>,
        pub(super) grid_config: RefCell<GridConfig>,
        pub(super) board: RefCell<PlacedBoard>,
        pub(super) tiles: RefCell<Vec<PlacedTile>>,
        pub(super) hint_tile: RefCell<Option<PlacedTile>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PuzzledPlacementModel {
        const NAME: &'static str = "PuzzledPlacementModel";
        type Type = PlacementModel;
        type ParentType = glib::Object;
    }

    #[glib::derived_properties]
    impl ObjectImpl for PuzzledPlacementModel {
        fn signals() -> &'static [Signal] {
            static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
            SIGNALS.get_or_init(|| vec![Signal::builder(TILE_MOVED_SIGNAL_NAME).build()])
        }
    }
}

glib::wrapper! {
    pub struct PlacementModel(ObjectSubclass<imp::PuzzledPlacementModel>);
}

impl PlacementModel {
    /// Creates a new PlacementModel.
    ///
    /// This calculates initial placements of tiles and the board.
    /// The size of the area where they are placed should be updated as soon as possible using
    /// [Self::update_pixel_size].
    ///
    /// The order of tiles in the given model is relevant, as their indices are used in other
    /// functions to reference them.
    ///
    /// Instances of PlacementModel are not reusable. Create a new one for each puzzle when
    /// they are needed.
    ///
    /// # Arguments
    ///
    /// * `puzzle_model`: the model to prepare positions for
    ///
    /// returns: PlacementModel
    pub fn new(puzzle_model: &PuzzleModel) -> Self {
        let obj: PlacementModel = glib::Object::builder().build();
        obj.imp().puzzle.replace(Some(puzzle_model.clone()));
        let puzzle_config = puzzle_model.config();

        obj.imp()
            .grid_config
            .replace(Self::initial_grid_config(puzzle_config));

        let board_config = puzzle_config.board_config();
        let board = PlacedBoard::new(
            board_config.layout().dim().into(),
            PixelOffset::default(),
            CellOffset(
                MIN_CELLS_TO_THE_SIDES_OF_BOARD,
                MIN_CELLS_TO_THE_TOP_OF_BOARD,
            ),
        );
        obj.imp().board.replace(board);

        let start_positions = initial::calculate_tile_start_positions(
            puzzle_config.tiles(),
            puzzle_config,
            CellOffset(
                MIN_CELLS_TO_THE_SIDES_OF_BOARD,
                MIN_CELLS_TO_THE_TOP_OF_BOARD,
            ),
        );
        let tiles: Vec<PlacedTile> = start_positions
            .iter()
            .enumerate()
            .map(move |(i, pos)| {
                let config = &puzzle_model.config().tiles()[i];
                let shape = config.base().clone();
                let cell_size = shape.dim().into();
                PlacedTile::new(config.name().clone(), shape, cell_size, *pos)
            })
            .collect();
        obj.imp().tiles.replace(tiles);

        obj.update_pixel_size(PixelOffset(100.0, 100.0), 10);
        obj
    }

    /// Recalculates tile and board positions for the new area size.
    ///
    /// The given `total_view_size_pixel` must be the actual size of the view the tiles
    /// and board are placed in or should be placed in.
    ///
    /// After this function completed, you can fetch the sizes and positions using the
    /// corresponding getters.
    ///
    /// # Arguments
    ///
    /// * `total_view_size_pixel`: the size of the view in pixels
    /// * `min_cell_size_pixel`: the min cell size for calculating the min area size
    ///
    /// returns: ()
    pub fn update_pixel_size(&self, total_view_size_pixel: PixelOffset, min_cell_size_pixel: u32) {
        if total_view_size_pixel.0 < 100.0 || total_view_size_pixel.1 < 100.0 {
            return;
        }
        self.imp().area_pixel_size.replace(total_view_size_pixel);
        self.update_grid_layout();
        let grid_config = self.imp().grid_config.borrow();
        self.imp().min_area_pixel_size.replace(
            grid_config
                .min_grid_cells
                .mul_scalar(min_cell_size_pixel as f64)
                .into(),
        );
        self.update_pixel_from_cell_data();
    }

    /// Takes all cell positions, calculates pixel positions for them and writes them
    /// back to the tiles and board.
    fn update_pixel_from_cell_data(&self) {
        let mut board = self.imp().board.borrow_mut();
        let position_cells = board.position_cells();
        board.set_position_pixel(self.translate_cells_to_pixels(position_cells));
        let size_cells = board.cell_size();
        board.set_pixel_size(self.translate_cells_to_pixels(size_cells));

        let mut tiles = self.imp().tiles.borrow_mut();
        for tile in tiles.iter_mut() {
            let position_cells = tile.position_cells();
            tile.set_position_pixels(self.translate_cells_to_pixels(position_cells));
            let cells_size = tile.cell_size();
            tile.set_pixel_size(self.translate_cells_to_pixels(cells_size));
        }
        if let Some(hint_tile) = self.imp().hint_tile.borrow_mut().as_mut() {
            let position_cells = hint_tile.position_cells();
            hint_tile.set_position_pixels(self.translate_cells_to_pixels(position_cells));
            let cells_size = hint_tile.cell_size();
            hint_tile.set_pixel_size(self.translate_cells_to_pixels(cells_size));
        }
    }

    /// Returns the minimum size the view needs to place all tiles and board while respecting
    /// the minimum cell size.
    pub fn min_area_size(&self) -> PixelOffset {
        self.imp().min_area_pixel_size.get()
    }

    /// The board position in pixels.
    pub fn board_pixel_position(&self) -> PixelOffset {
        let board = self.imp().board.borrow();
        board.position_pixel()
    }

    /// The board position in cells.
    fn board_cell_position(&self) -> CellOffset {
        let board = self.imp().board.borrow();
        board.position_cells()
    }

    /// The board size in pixels.
    pub fn board_size(&self) -> PixelOffset {
        let board = self.imp().board.borrow();
        board.pixel_size()
    }

    /// The position of the tile with the given index in pixels.
    ///
    /// None, if the tile is being dragged.
    ///
    /// The index is the same as the index of the tile in the [PuzzleModel].
    pub fn tile_pixel_position(&self, idx: usize) -> Option<PixelOffset> {
        let list = self.imp().tiles.borrow();
        let tile = list.get(idx).unwrap();
        if tile.dragged() {
            None
        } else {
            Some(tile.position_pixels())
        }
    }

    /// The position of the tile with the given index in cells.
    ///
    /// None, if the tile is being dragged.
    ///
    /// The index is the same as the index of the tile in the [PuzzleModel].
    fn tile_cell_position(&self, idx: usize) -> Option<CellOffset> {
        let list = self.imp().tiles.borrow();
        let tile = list.get(idx).unwrap();
        if tile.dragged() {
            None
        } else {
            Some(tile.position_cells())
        }
    }

    /// The size of the tile in pixels.
    pub fn tile_size(&self, idx: usize) -> PixelOffset {
        let list = self.imp().tiles.borrow();
        let tile = list.get(idx).unwrap();
        tile.pixel_size()
    }

    /// Updates the tile position by calculating the new position in cells from the given
    /// position in pixels. This calculation snaps the position to the nearest cell
    /// position.
    ///
    /// # Arguments
    ///
    /// * `idx`: the index of the tile
    /// * `position`: the new position in pixels
    ///
    /// returns: ()
    pub fn update_tile_pixel_position(&self, idx: usize, position: PixelOffset) {
        {
            let mut list = self.imp().tiles.borrow_mut();
            let tile = list.get_mut(idx).unwrap();
            let position_cells = self.translate_pixels_to_cells(position);
            tile.set_position_pixels(self.translate_cells_to_pixels(position_cells));
            tile.set_position_cells(position_cells);
        }
        self.emit_tile_moved();
    }

    /// Marks the tile as being dragged or not.
    ///
    /// If a tile is marked as dragged, its position is not valid and should not be recalculated.
    ///
    /// # Arguments
    ///
    /// * `idx`: the index of the tile
    /// * `dragged`: true, if the tile is being dragged, otherwise false
    ///
    /// returns: ()
    pub fn update_tile_dragged(&self, idx: usize, dragged: bool) {
        let mut list = self.imp().tiles.borrow_mut();
        let tile = list.get_mut(idx).unwrap();
        tile.set_dragged(dragged);
    }

    /// Updates the shape for the given tile. This must be called, if the tile rotated
    /// or flipped. The given shape must be a valid result of flipping or rotating the base
    /// shape.
    ///
    /// # Arguments
    ///
    /// * `idx`: the index of the tile to update
    /// * `shape`: the new rotation
    ///
    /// returns: ()
    pub fn update_tile_shape(&self, idx: usize, shape: Shape) {
        {
            let mut list = self.imp().tiles.borrow_mut();
            let old = list.get_mut(idx).unwrap();
            old.set_cell_size(shape.dim().into());
            old.set_current_rotation(shape);
        }
        self.emit_tile_moved();
    }

    /// Initializes a new hint tile at the given position on the board and with the
    /// given shape.
    /// This replaces the hint tile currently stored in the PlacementModel.
    /// Make sure to call [Self::remove_hint_tile] before this call and remove the
    /// old hint tile from the view.
    ///
    /// # Arguments
    ///
    /// * `position_on_board`: the position of the hint tile relative to the board
    /// * `shape`: the shape of the hint tile
    ///
    /// returns: ()
    pub fn init_hint_tile(&self, position_on_board: CellOffset, shape: Shape) {
        let position = self.imp().board.borrow().position_cells() + position_on_board;
        let size = shape.dim().into();
        self.imp()
            .hint_tile
            .replace(Some(PlacedTile::new(None, shape, size, position)));
    }

    /// Returns the position of the hint tile in pixels.
    ///
    /// None, if no hint tile is registered.
    pub fn hint_tile_position(&self) -> Option<PixelOffset> {
        let hint_tile_borrow = self.imp().hint_tile.borrow();
        hint_tile_borrow.as_ref().map(|tile| tile.position_pixels())
    }

    /// Returns the size of the hint tile in pixels.
    ///
    /// None, if no hint tile is registered.
    pub fn hint_tile_size(&self) -> Option<PixelOffset> {
        let hint_tile_borrow = self.imp().hint_tile.borrow();
        hint_tile_borrow.as_ref().map(|tile| tile.pixel_size())
    }

    /// Removes the hint tile from the placement model.
    /// Positions will no longer be calculated for the old hint tile.
    pub fn remove_hint_tile(&self) {
        self.imp().hint_tile.replace(None);
    }

    /// Connects to the `tile_moved` signal which is emitted when a tile changes positon
    /// or rotation.
    pub fn connect_tile_moved<F: Fn() + 'static>(&self, callback: F) {
        self.connect_local(TILE_MOVED_SIGNAL_NAME, false, move |_| {
            callback();
            None
        });
    }

    fn emit_tile_moved(&self) {
        debug!("Emitting tile moved signal");
        self.emit_by_name::<()>(TILE_MOVED_SIGNAL_NAME, &[]);
    }

    /// Converts a PixelOffset to a CellOffset respecting the current grid layout.
    ///
    /// # Arguments
    ///
    /// * `position`: the position to convert
    ///
    /// returns: CellOffset
    fn translate_pixels_to_cells(&self, position: PixelOffset) -> CellOffset {
        let cell_size = self.imp().grid_config.borrow().cell_size_pixel;
        position.div_scalar(cell_size as f64).round().into()
    }

    /// Converts a CellOffset to a PixelOffset respecting the current grid layout.
    ///
    /// # Arguments
    ///
    /// * `position`: the position to convert
    ///
    /// returns: PixelOffset
    fn translate_cells_to_pixels(&self, position: CellOffset) -> PixelOffset {
        let cell_size = self.imp().grid_config.borrow().cell_size_pixel;
        position.mul_scalar(cell_size as f64).into()
    }

    /// Finds the index of a tile that has the same base shape.
    ///
    /// None, if no tile was found.
    ///
    /// # Arguments
    ///
    /// * `base`: the base shape to search for
    ///
    /// returns: Option<usize>
    pub fn find_tile_matching_base(&self, base: &Shape) -> Option<usize> {
        let tiles = self.imp().tiles.borrow();
        tiles
            .iter()
            .enumerate()
            .find(|t| t.1.base() == base)
            .map(|t| t.0)
    }

    pub fn extract_puzzle_state(
        &self,
        puzzle_type_extension: Ref<Option<PuzzleTypeExtension>>,
    ) -> Result<PuzzleState, String> {
        let puzzle = self.imp().puzzle.borrow();
        if puzzle.is_none() {
            return Err("No puzzle set".to_string());
        }
        let puzzle = puzzle.as_ref().unwrap();
        let puzzle_config = puzzle.config();

        let mut state = PuzzleState::new(puzzle_config, puzzle_type_extension);

        let tiles = self.imp().tiles.borrow();
        let board_position = self.board_cell_position();

        for (i, tile) in tiles.iter().enumerate() {
            let tile_position = self
                .tile_cell_position(i)
                .ok_or_else(|| "Tile position not set".to_string())?;
            let tile_position = tile_position - board_position + CellOffset(1, 1);
            let mut any_cell_on_board = false;
            for ((x, y), cell) in tile.current_rotation().indexed_iter() {
                if !*cell {
                    continue;
                }

                let cell_position = tile_position + CellOffset(x as i32, y as i32);
                if cell_position.0 >= 0
                    && cell_position.1 >= 0
                    && (cell_position.0 as usize) < state.grid.dim().0
                    && (cell_position.1 as usize) < state.grid.dim().1
                {
                    let idx: (usize, usize) = cell_position.into();
                    let new = match state.grid.get_mut(idx) {
                        None => return Err("Index out of bounds".to_string()),
                        Some(cell_ref) => {
                            let old = take(cell_ref);
                            let tile_cell_placement = TileCellPlacement {
                                tile_id: i,
                                cell_position: CellOffset(x as i32, y as i32),
                            };
                            match old {
                                Cell::Empty(data) => {
                                    any_cell_on_board = any_cell_on_board || data.is_on_board;
                                    Cell::One(data, tile_cell_placement)
                                }
                                Cell::One(data, existing_widget) => {
                                    any_cell_on_board = any_cell_on_board || data.is_on_board;
                                    let widgets = vec![existing_widget, tile_cell_placement];
                                    Cell::Many(data, widgets)
                                }
                                Cell::Many(data, mut widgets) => {
                                    any_cell_on_board = any_cell_on_board || data.is_on_board;
                                    widgets.push(tile_cell_placement);
                                    Cell::Many(data, widgets)
                                }
                            }
                        }
                    };
                    state.grid[idx] = new;
                }
            }
            if !any_cell_on_board {
                let unused_tile = UnusedTile {
                    id: i,
                    base: tile.base().clone(),
                    name: tile.name().clone(),
                };
                state.unused_tiles.insert(unused_tile);
            }
        }
        Ok(state)
    }
}
