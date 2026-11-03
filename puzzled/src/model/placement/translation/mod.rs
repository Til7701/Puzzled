pub mod polyomino;

use crate::model::placement::PixelPosition;
use puzzled_common::polyform::coord::coord_box::CoordBox;

pub trait CoordTranslator {
    /// Converts a PixelOffset to a CellOffset respecting the current grid layout.
    ///
    /// # Arguments
    ///
    /// * `position`: the position to convert
    ///
    /// returns: CellOffset
    fn translate_pixels_to_cells(&self, position: PixelPosition) -> CoordBox;

    /// Converts a CellOffset to a PixelOffset respecting the current grid layout.
    ///
    /// # Arguments
    ///
    /// * `position`: the position to convert
    ///
    /// returns: PixelOffset
    fn translate_cells_to_pixels(&self, position: CoordBox) -> PixelPosition;
}
