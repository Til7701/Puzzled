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

    pub fn update_pixel_size(&self, total_view_size_pixel: PixelOffset, min_cell_size_pixel: u32) {
        if total_view_size_pixel.0 < 100.0
            || total_view_size_pixel.1 < 100.0
            || min_cell_size_pixel < 10
        {
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

    pub fn min_area_size(&self) -> PixelOffset {
        self.imp().min_area_pixel_size.get()
    }

    pub fn board_pixel_position(&self) -> PixelOffset {
        let board = self.imp().board.borrow();
        board.position_pixel()
    }

    pub fn board_cell_position(&self) -> CellOffset {
        let board = self.imp().board.borrow();
        board.position_cells()
    }

    pub fn board_size(&self) -> PixelOffset {
        let board = self.imp().board.borrow();
        board.pixel_size()
    }

    /// None, if the tile is being dragged
    pub fn tile_pixel_position(&self, idx: usize) -> Option<PixelOffset> {
        let list = self.imp().tiles.borrow();
        let tile = list.get(idx).unwrap();
        if tile.dragged() {
            None
        } else {
            Some(tile.position_pixels())
        }
    }

    /// None, if the tile is being dragged
    pub fn tile_cell_position(&self, idx: usize) -> Option<CellOffset> {
        let list = self.imp().tiles.borrow();
        let tile = list.get(idx).unwrap();
        if tile.dragged() {
            None
        } else {
            Some(tile.position_cells())
        }
    }

    pub fn tile_size(&self, idx: usize) -> PixelOffset {
        let list = self.imp().tiles.borrow();
        let tile = list.get(idx).unwrap();
        tile.pixel_size()
    }

    pub fn update_tile_pixel_position(&self, idx: usize, position: PixelOffset) {
        {
            let mut list = self.imp().tiles.borrow_mut();
            let tile = list.get_mut(idx).unwrap();
            let position_cells = self.translate_pixels_to_cells(position);
            tile.set_position_pixels(position);
            tile.set_position_cells(position_cells);
        }
        self.emit_tile_moved();
    }

    pub fn update_tile_dragged(&self, idx: usize, dragged: bool) {
        let mut list = self.imp().tiles.borrow_mut();
        let tile = list.get_mut(idx).unwrap();
        tile.set_dragged(dragged);
    }

    pub fn update_tile_shape(&self, idx: usize, shape: Shape) {
        {
            let mut list = self.imp().tiles.borrow_mut();
            let old = list.get_mut(idx).unwrap();
            old.set_cell_size(shape.dim().into());
            old.set_current_rotation(shape);
        }
        self.emit_tile_moved();
    }

    pub fn init_hint_tile(&self, position_on_board: CellOffset, shape: Shape) {
        let position = self.imp().board.borrow().position_cells() + position_on_board;
        let size = shape.dim().into();
        self.imp()
            .hint_tile
            .replace(Some(PlacedTile::new(None, shape, size, position)));
    }

    pub fn hint_tile_position(&self) -> PixelOffset {
        let hint_tile_borrow = self.imp().hint_tile.borrow();
        hint_tile_borrow.as_ref().unwrap().position_pixels()
    }

    pub fn hint_tile_size(&self) -> PixelOffset {
        let hint_tile_borrow = self.imp().hint_tile.borrow();
        hint_tile_borrow.as_ref().unwrap().pixel_size()
    }

    pub fn remove_hint_tile(&self) {
        self.imp().hint_tile.replace(None);
    }

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

    fn translate_pixels_to_cells(&self, position: PixelOffset) -> CellOffset {
        let cell_size = self.imp().grid_config.borrow().cell_size_pixel;
        position.div_scalar(cell_size as f64).round().into()
    }

    fn translate_cells_to_pixels(&self, position: CellOffset) -> PixelOffset {
        let cell_size = self.imp().grid_config.borrow().cell_size_pixel;
        position.mul_scalar(cell_size as f64).into()
    }

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
