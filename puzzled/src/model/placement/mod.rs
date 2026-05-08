use crate::model::collection::CollectionModel;
use crate::model::placement::grid::GridConfig;
use crate::model::puzzle::PuzzleModel;
use crate::offset::{CellOffset, PixelOffset};
use adw::glib;
use adw::prelude::ObjectExt;
use adw::subclass::prelude::*;
use log::debug;

mod grid;
mod initial;

const TILE_MOVED_SIGNAL_NAME: &str = "tile-moved";

mod imp {
    use super::*;
    use crate::model::placement::grid::GridConfig;
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
        pub(super) grid_config: RefCell<GridConfig>,
        pub(super) board_position_cells: Cell<CellOffset>,
        pub(super) tile_positions_cells: RefCell<Vec<CellOffset>>,
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
            .replace(GridConfig::initial_grid_config(puzzle_config));

        let start_positions = initial::calculate_tile_start_positions(
            puzzle_config.tiles(),
            puzzle_config,
            obj.imp().grid_config.borrow().board_offset_cells,
        );
        obj.imp().tile_positions_cells.replace(start_positions);

        obj.update_pixel_size(PixelOffset(100.0, 100.0));
        obj
    }

    pub fn update_pixel_size(&self, size: PixelOffset) {
        self.imp().area_pixel_size.replace(size);
    }

    pub fn tile_pixel_position(&self, idx: usize) -> PixelOffset {
        let cell_position = self.imp().tile_positions_cells.borrow()[idx];
        let pixel_size = self.imp().grid_config.borrow().cell_size_pixel;
        PixelOffset::from(cell_position).mul_scalar(pixel_size as f64)
    }

    pub fn update_tile_pixel_position(&self, idx: usize, position: PixelOffset) {
        todo!()
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

    /// Get the dimensions of the board in cells.
    fn board_size_cells(&self) -> CellOffset {
        let puzzle = self.imp().puzzle.borrow();
        let board_size = puzzle
            .as_ref()
            .map(|p| p.config().board_config().layout().dim())
            .unwrap_or((1, 1));
        CellOffset(board_size.0 as i32, board_size.1 as i32)
    }
}
