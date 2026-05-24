use crate::app::puzzle::puzzle_area::puzzle_state::{
    Cell, PuzzleState, TileCellPlacement, UnusedTile,
};
use crate::model::extension::PuzzleTypeExtension;
use crate::model::placement::grid::GridConfig;
use crate::model::puzzle::PuzzleModel;
use crate::offset::{CellOffset, PixelOffset};
use adw::glib;
use adw::prelude::ObjectExt;
use adw::subclass::prelude::*;
use log::debug;
use puzzled_common::Shape;
use std::cell::Ref;
use std::mem::take;

mod grid;
mod initial;

const TILE_MOVED_SIGNAL_NAME: &str = "tile-moved";

mod imp {
    use super::*;
    use crate::model::placement::grid::GridConfig;
    use crate::offset::PixelOffset;
    use adw::glib::subclass::Signal;
    use adw::glib::Properties;
    use puzzled_common::Shape;
    use std::cell::{Cell, RefCell};
    use std::sync::OnceLock;

    #[derive(Debug, Default, Properties)]
    #[properties(wrapper_type = super::PlacementModel)]
    pub struct PuzzledPlacementModel {
        pub(super) puzzle: RefCell<Option<PuzzleModel>>,
        pub(super) area_pixel_size: Cell<PixelOffset>,
        pub(super) grid_config: RefCell<GridConfig>,
        pub(super) board_position_cells: Cell<CellOffset>,
        pub(super) tile_positions_shapes: RefCell<Vec<(CellOffset, Shape)>>,
        pub(super) hint_tile_position: Cell<Option<CellOffset>>,
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
        let start_positions_shapes: Vec<(CellOffset, Shape)> = start_positions
            .iter()
            .enumerate()
            .map(move |(i, pos)| (*pos, puzzle_model.config().tiles()[i].base().clone()))
            .collect();
        obj.imp()
            .tile_positions_shapes
            .replace(start_positions_shapes);

        obj.update_pixel_size(PixelOffset(100.0, 100.0), 10);
        obj
    }

    pub fn update_pixel_size(&self, total_view_size_pixel: PixelOffset, min_cell_size_pixel: u32) {
        self.imp().area_pixel_size.replace(total_view_size_pixel);
        todo!()
    }

    pub fn cell_size(&self) -> u32 {
        self.imp().grid_config.borrow().cell_size_pixel
    }

    pub fn min_area_size(&self) -> PixelOffset {
        todo!()
    }

    pub fn board_pixel_position(&self) -> PixelOffset {
        todo!()
    }

    pub fn board_cel_position(&self) -> CellOffset {
        todo!()
    }

    pub fn board_size(&self) -> PixelOffset {
        todo!()
    }

    /// None, if the tile is being dragged
    pub fn tile_pixel_position(&self, idx: usize) -> Option<PixelOffset> {
        todo!();
        let list: Ref<Vec<(CellOffset, Shape)>> = self.imp().tile_positions_shapes.borrow();
        let tile = list.get(idx).unwrap();
        let pixel_size = self.imp().grid_config.borrow().cell_size_pixel;
        Some(PixelOffset::from(tile.0).mul_scalar(pixel_size as f64))
    }

    /// None, if the tile is being dragged
    pub fn tile_cell_position(&self, idx: usize) -> Option<CellOffset> {
        todo!();
        let list: Ref<Vec<(CellOffset, Shape)>> = self.imp().tile_positions_shapes.borrow();
        let tile = list.get(idx).unwrap();
        let pixel_size = self.imp().grid_config.borrow().cell_size_pixel;
        Some(PixelOffset::from(tile.0).mul_scalar(pixel_size as f64))
    }

    pub fn tile_size(&self, idx: usize) -> PixelOffset {
        todo!()
    }

    pub fn update_tile_pixel_position(&self, idx: usize, position: PixelOffset) {
        todo!()
    }

    pub fn update_tile_shape(&self, idx: usize, shape: Shape) {
        let mut list = self.imp().tile_positions_shapes.borrow_mut();
        let old = list.get_mut(idx).unwrap();
        old.1 = shape;
    }

    pub fn init_hint_tile_position(&self, board_position: CellOffset) {
        let position = self.imp().grid_config.borrow().board_offset_cells + board_position;
        self.imp().hint_tile_position.replace(Some(position));
    }

    pub fn hint_tile_position(&self) -> PixelOffset {
        todo!()
    }

    pub fn hint_tile_size(&self) -> PixelOffset {
        todo!()
    }

    pub fn remove_hint_tile(&self) {
        self.imp().hint_tile_position.replace(None);
    }

    pub fn connect_tile_moved<F: Fn() + 'static>(&self, callback: F) {
        self.connect_local(TILE_MOVED_SIGNAL_NAME, false, move |_| {
            callback();
            None
        });
    }

    fn emit_tile_moved(&self) {
        debug!("Emitting tile moved signal",);
        self.emit_by_name::<()>(TILE_MOVED_SIGNAL_NAME, &[]);
    }
}
