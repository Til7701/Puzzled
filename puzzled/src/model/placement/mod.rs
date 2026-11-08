use crate::app::puzzle::puzzle_area::puzzle_state::PuzzleState;
use crate::model::extension::PuzzleTypeExtension;
use crate::model::placement::model::PlacementModel;
use crate::model::placement::prelude::{FullPlacementModel, SimplePlacementModel};
use crate::model::puzzle::PuzzleModel;
use adw::glib;
use adw::prelude::ObjectExt;
use adw::subclass::prelude::*;
use log::debug;
use model::hexomino::HexominoPlacementModel;
use model::polyomino::PolyominoPlacementModel;
use puzzled_common::polyform::{CoordBox, Polyform};
use std::cell::Ref;

mod board;
mod grid;
mod initial;
mod model;
mod pixel;
mod prelude;
mod tile;

pub use pixel::PixelPosition;

const TILE_MOVED_SIGNAL_NAME: &str = "tile-moved";

mod imp {
    use super::*;
    use crate::model::placement::model::PlacementModel;
    use adw::glib::Properties;
    use adw::glib::subclass::Signal;
    use std::cell::RefCell;
    use std::sync::OnceLock;

    #[derive(Debug, Default, Properties)]
    #[properties(wrapper_type = super::PlacementModelWrapper)]
    pub struct PuzzledPlacementModel {
        pub(super) puzzle: RefCell<Option<PuzzleModel>>,
        pub(super) placement_model: RefCell<PlacementModel>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PuzzledPlacementModel {
        const NAME: &'static str = "PuzzledPlacementModel";
        type Type = PlacementModelWrapper;
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
    pub struct PlacementModelWrapper(ObjectSubclass<imp::PuzzledPlacementModel>);
}

impl PlacementModelWrapper {
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
        let obj: PlacementModelWrapper = glib::Object::builder().build();
        obj.imp().puzzle.replace(Some(puzzle_model.clone()));
        let puzzle_config = puzzle_model.config();

        match puzzle_config.board_config().layout() {
            Polyform::Polyomino(_) => obj.imp().placement_model.replace(PlacementModel::Polyomino(
                PolyominoPlacementModel::new(puzzle_model),
            )),
            Polyform::Hexomino(_) => obj.imp().placement_model.replace(PlacementModel::Hexomino(
                HexominoPlacementModel::new(puzzle_model),
            )),
        };

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
    pub fn update_pixel_size(
        &self,
        total_view_size_pixel: PixelPosition,
        min_cell_size_pixel: u32,
    ) {
        self.imp()
            .placement_model
            .borrow_mut()
            .update_pixel_size(total_view_size_pixel, min_cell_size_pixel);
    }

    /// Returns the minimum size the view needs to place all tiles and board while respecting
    /// the minimum cell size.
    pub fn min_area_size(&self) -> PixelPosition {
        self.imp().placement_model.borrow().min_area_size()
    }

    /// The board position in pixels.
    pub fn board_pixel_position(&self) -> PixelPosition {
        self.imp().placement_model.borrow().board_pixel_position()
    }

    /// The board position in cells.
    fn board_cell_position(&self) -> CoordBox {
        self.imp().placement_model.borrow().board_cell_position()
    }

    /// The board size in pixels.
    pub fn board_size(&self) -> PixelPosition {
        self.imp().placement_model.borrow().board_size()
    }

    /// The position of the tile with the given index in pixels.
    ///
    /// None, if the tile is being dragged.
    ///
    /// The index is the same as the index of the tile in the [PuzzleModel].
    pub fn tile_pixel_position(&self, idx: usize) -> Option<PixelPosition> {
        self.imp().placement_model.borrow().tile_pixel_position(idx)
    }

    /// The position of the tile with the given index in cells.
    ///
    /// None, if the tile is being dragged.
    ///
    /// The index is the same as the index of the tile in the [PuzzleModel].
    fn tile_cell_position(&self, idx: usize) -> Option<CoordBox> {
        self.imp().placement_model.borrow().tile_cell_position(idx)
    }

    /// The size of the tile in pixels.
    pub fn tile_size(&self, idx: usize) -> PixelPosition {
        self.imp().placement_model.borrow().tile_size(idx)
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
    pub fn update_tile_pixel_position(&self, idx: usize, position: PixelPosition) {
        self.imp()
            .placement_model
            .borrow_mut()
            .update_tile_pixel_position(idx, position);
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
        self.imp()
            .placement_model
            .borrow_mut()
            .update_tile_dragged(idx, dragged);
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
    pub fn update_tile_shape(&self, idx: usize, shape: Polyform<()>) {
        self.imp()
            .placement_model
            .borrow_mut()
            .update_tile_shape(idx, shape);
        self.emit_tile_moved();
    }

    pub fn prototile_coord(&self, idx: usize, pixel_position: PixelPosition) -> Option<CoordBox> {
        self.imp()
            .placement_model
            .borrow_mut()
            .prototile_coord(idx, pixel_position)
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
    pub fn init_hint_tile(&self, position_on_board: CoordBox, shape: Polyform<()>) {
        self.imp()
            .placement_model
            .borrow_mut()
            .init_hint_tile(position_on_board, shape);
    }

    /// Returns the position of the hint tile in pixels.
    ///
    /// None, if no hint tile is registered.
    pub fn hint_tile_position(&self) -> Option<PixelPosition> {
        self.imp().placement_model.borrow().hint_tile_position()
    }

    /// Returns the size of the hint tile in pixels.
    ///
    /// None, if no hint tile is registered.
    pub fn hint_tile_size(&self) -> Option<PixelPosition> {
        self.imp().placement_model.borrow().hint_tile_size()
    }

    /// Removes the hint tile from the placement model.
    /// Positions will no longer be calculated for the old hint tile.
    pub fn remove_hint_tile(&self) {
        self.imp().placement_model.borrow_mut().remove_hint_tile();
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

    /// Finds the index of a tile that has the same base shape.
    ///
    /// None, if no tile was found.
    ///
    /// # Arguments
    ///
    /// * `base`: the base shape to search for
    ///
    /// returns: Option<usize>
    pub fn find_tile_matching_base(&self, base: &Polyform<()>) -> Option<usize> {
        self.imp()
            .placement_model
            .borrow()
            .find_tile_matching_base(base)
    }

    pub fn extract_puzzle_state(
        &self,
        puzzle_type_extension: Ref<Option<PuzzleTypeExtension>>,
    ) -> Result<PuzzleState, String> {
        let puzzle = self.imp().puzzle.borrow();
        if let Some(p) = puzzle.as_ref() {
            self.imp()
                .placement_model
                .borrow()
                .extract_puzzle_state(p, puzzle_type_extension)
        } else {
            Err("No puzzle set".to_string())
        }
    }
}
