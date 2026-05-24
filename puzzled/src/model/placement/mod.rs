use crate::app::puzzle::puzzle_area::puzzle_state::{PuzzleState, TileCellPlacement, UnusedTile};
use crate::model::placement::tile::PlacedTile;
use crate::model::puzzle::PuzzleModel;
use crate::offset::{CellOffset, PixelOffset};
use adw::glib;
use adw::prelude::ObjectExt;
use adw::subclass::prelude::*;
use log::debug;
use puzzled_common::shape::shape_square;
use puzzled_common::Shape;

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
        let puzzle_config = puzzle_model.config();

        obj.imp()
            .grid_config
            .replace(Self::initial_grid_config(puzzle_config));

        let start_positions = initial::calculate_tile_start_positions(
            puzzle_config.tiles(),
            puzzle_config,
            obj.imp().grid_config.borrow().board_offset_cells,
        );
        let tiles: Vec<PlacedTile> = start_positions
            .iter()
            .enumerate()
            .map(move |(i, pos)| {
                let shape = puzzle_model.config().tiles()[i].base().clone();
                let cell_size = shape.dim().into();
                PlacedTile::new(shape, cell_size, *pos)
            })
            .collect();
        obj.imp().tiles.replace(tiles);

        obj.update_pixel_size(PixelOffset(100.0, 100.0), 10);
        obj
    }

    pub fn update_pixel_size(&self, total_view_size_pixel: PixelOffset, min_cell_size_pixel: u32) {
        self.imp().area_pixel_size.replace(total_view_size_pixel);
        self.update_grid_layout();
        let grid_config = self.imp().grid_config.borrow();
        self.imp().min_area_pixel_size.replace(
            CellOffset(
                grid_config.min_grid_h_cell_count as i32,
                grid_config.min_grid_v_cell_count as i32,
            )
            .mul_scalar(min_cell_size_pixel as f64)
            .into(),
        );
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
        let mut list = self.imp().tiles.borrow_mut();
        let tile = list.get_mut(idx).unwrap();
        let position_cells = self.translate_pixel_to_cells(position);
        tile.set_position_pixels(position);
        tile.set_position_cells(position_cells);
        self.emit_tile_moved();
    }

    pub fn update_tile_dragged(&self, idx: usize, dragged: bool) {
        let mut list = self.imp().tiles.borrow_mut();
        let tile = list.get_mut(idx).unwrap();
        tile.set_dragged(dragged);
    }

    pub fn update_tile_shape(&self, idx: usize, shape: Shape) {
        let mut list = self.imp().tiles.borrow_mut();
        let old = list.get_mut(idx).unwrap();
        old.set_current_rotation(shape);
        self.emit_tile_moved();
    }

    pub fn init_hint_tile(&self, position_on_board: CellOffset) {
        let position = self.imp().grid_config.borrow().board_offset_cells + position_on_board;
        self.imp().hint_tile.replace(Some(PlacedTile::new(
            shape_square(&[[]]),
            CellOffset::default(),
            position,
        )));
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

    fn translate_pixel_to_cells(&self, position: PixelOffset) -> CellOffset {
        let cell_size = self.imp().grid_config.borrow().cell_size_pixel;
        position.div_scalar(cell_size as f64).into() // TODO round
    }
}
