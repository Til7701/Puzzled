use crate::offset::{CellOffset, PixelOffset};
use puzzled_common::Shape;

#[derive(Clone, Debug, Default)]
pub struct PlacedTile {
    name: Option<String>,
    base: Shape,
    current_rotation: Shape,
    cell_size: CellOffset,
    pixel_size: PixelOffset,
    position_cells: CellOffset,
    position_pixels: PixelOffset,
    dragged: bool,
}

impl PlacedTile {
    pub fn new(
        name: Option<String>,
        base: Shape,
        cell_size: CellOffset,
        position_cells: CellOffset,
    ) -> Self {
        PlacedTile {
            name,
            base: base.clone(),
            current_rotation: base,
            cell_size,
            pixel_size: PixelOffset::default(),
            position_cells,
            position_pixels: PixelOffset::default(),
            dragged: false,
        }
    }

    pub fn name(&self) -> &Option<String> {
        &self.name
    }

    pub fn base(&self) -> &Shape {
        &self.base
    }

    pub fn current_rotation(&self) -> &Shape {
        &self.current_rotation
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

    pub fn position_pixels(&self) -> PixelOffset {
        self.position_pixels
    }

    pub fn dragged(&self) -> bool {
        self.dragged
    }

    pub fn set_current_rotation(&mut self, current_rotation: Shape) {
        self.current_rotation = current_rotation;
    }

    pub fn set_cell_size(&mut self, cell_size: CellOffset) {
        self.cell_size = cell_size;
    }

    pub fn set_pixel_size(&mut self, pixel_size: PixelOffset) {
        self.pixel_size = pixel_size;
    }

    pub fn set_position_cells(&mut self, position_cells: CellOffset) {
        self.position_cells = position_cells;
    }

    pub fn set_position_pixels(&mut self, position_pixels: PixelOffset) {
        self.position_pixels = position_pixels;
    }

    pub fn set_dragged(&mut self, dragged: bool) {
        self.dragged = dragged;
    }
}
