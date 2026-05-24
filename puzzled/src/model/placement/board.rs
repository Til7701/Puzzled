use crate::offset::{CellOffset, PixelOffset};

#[derive(Clone, Debug, Default)]
pub struct PlacedBoard {
    cell_size: CellOffset,
    pixel_size: PixelOffset,
    position_cells: CellOffset,
    position_pixel: PixelOffset,
}

impl PlacedBoard {
    pub fn new(size: CellOffset, position_pixel: PixelOffset, position_cells: CellOffset) -> Self {
        PlacedBoard {
            cell_size: size,
            pixel_size: PixelOffset::default(),
            position_pixel,
            position_cells,
        }
    }

    pub fn cell_size(&self) -> CellOffset {
        self.cell_size
    }

    pub fn pixel_size(&self) -> PixelOffset {
        self.pixel_size
    }

    pub fn position_cells(&self) -> CellOffset {
        self.position_cells
    }

    pub fn position_pixel(&self) -> PixelOffset {
        self.position_pixel
    }

    pub fn set_position_cells(&mut self, position_cells: CellOffset) {
        self.position_cells = position_cells;
    }

    pub fn set_position_pixel(&mut self, position_pixel: PixelOffset) {
        self.position_pixel = position_pixel;
    }

    pub fn set_pixel_size(&mut self, pixel_size: PixelOffset) {
        self.pixel_size = pixel_size;
    }
}
