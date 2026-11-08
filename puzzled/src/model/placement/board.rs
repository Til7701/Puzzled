use crate::model::placement::PixelPosition;
use puzzled_common::polyform::Coord;

#[derive(Clone, Debug, Default)]
pub struct PlacedBoard<C: Coord> {
    cell_size: C,
    pixel_size: PixelPosition,
    position_cells: C,
    position_pixel: PixelPosition,
}

impl<C: Coord> PlacedBoard<C> {
    pub fn new(size: C, position_pixel: PixelPosition, position_cells: C) -> Self {
        PlacedBoard {
            cell_size: size,
            pixel_size: PixelPosition::default(),
            position_pixel,
            position_cells,
        }
    }

    pub fn cell_size(&self) -> C {
        self.cell_size
    }

    pub fn pixel_size(&self) -> PixelPosition {
        self.pixel_size
    }

    pub fn position_cells(&self) -> C {
        self.position_cells
    }

    pub fn position_pixel(&self) -> PixelPosition {
        self.position_pixel
    }

    pub fn set_position_cells(&mut self, position_cells: C) {
        self.position_cells = position_cells;
    }

    pub fn set_position_pixel(&mut self, position_pixel: PixelPosition) {
        self.position_pixel = position_pixel;
    }

    pub fn set_pixel_size(&mut self, pixel_size: PixelPosition) {
        self.pixel_size = pixel_size;
    }
}
