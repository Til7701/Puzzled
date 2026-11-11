use crate::model::placement::PixelPosition;
use puzzled_polyform::{Coord, UnitPoly};

#[derive(Clone, Debug, Default)]
pub struct PlacedTile<P: UnitPoly, C: Coord> {
    name: Option<String>,
    base: P,
    current_rotation: P,
    cell_size: C,
    pixel_size: PixelPosition,
    position_cells: C,
    position_pixels: PixelPosition,
    dragged: bool,
}

impl<P: UnitPoly, C: Coord> PlacedTile<P, C> {
    pub fn new(name: Option<String>, base: P, cell_size: C, position_cells: C) -> Self {
        PlacedTile {
            name,
            base: base.clone(),
            current_rotation: base,
            cell_size,
            pixel_size: PixelPosition::default(),
            position_cells,
            position_pixels: PixelPosition::default(),
            dragged: false,
        }
    }

    pub fn name(&self) -> &Option<String> {
        &self.name
    }

    pub fn base(&self) -> &P {
        &self.base
    }

    pub fn current_rotation(&self) -> &P {
        &self.current_rotation
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

    pub fn position_pixels(&self) -> PixelPosition {
        self.position_pixels
    }

    pub fn dragged(&self) -> bool {
        self.dragged
    }

    pub fn set_current_rotation(&mut self, current_rotation: P) {
        self.current_rotation = current_rotation;
    }

    pub fn set_cell_size(&mut self, cell_size: C) {
        self.cell_size = cell_size;
    }

    pub fn set_pixel_size(&mut self, pixel_size: PixelPosition) {
        self.pixel_size = pixel_size;
    }

    pub fn set_position_cells(&mut self, position_cells: C) {
        self.position_cells = position_cells;
    }

    pub fn set_position_pixels(&mut self, position_pixels: PixelPosition) {
        self.position_pixels = position_pixels;
    }

    pub fn set_dragged(&mut self, dragged: bool) {
        self.dragged = dragged;
    }
}
