use crate::model::placement::PixelPosition;
use puzzled_common::polyform::Polyform;
use puzzled_common::polyform::grid::Coord;

#[derive(Clone, Debug, Default)]
pub struct PlacedTile {
    name: Option<String>,
    base: Polyform<()>,
    current_rotation: Polyform<()>,
    // cell_size: Coord,
    pixel_size: PixelPosition,
    // position_cells: Coord,
    position_pixels: PixelPosition,
    dragged: bool,
}

impl PlacedTile {
    pub fn new(
        name: Option<String>,
        base: Polyform<()>,
        cell_size: Coord,
        position_cells: Coord,
    ) -> Self {
        // PlacedTile {
        //     name,
        //     base: base.clone(),
        //     current_rotation: base,
        //     cell_size,
        //     pixel_size: PixelOffset::default(),
        //     position_cells,
        //     position_pixels: PixelOffset::default(),
        //     dragged: false,
        // }
        todo!()
    }

    pub fn name(&self) -> &Option<String> {
        &self.name
    }

    pub fn base(&self) -> &Polyform<()> {
        &self.base
    }

    pub fn current_rotation(&self) -> &Polyform<()> {
        &self.current_rotation
    }

    pub fn cell_size(&self) -> Coord {
        // self.cell_size
        todo!()
    }

    pub fn pixel_size(&self) -> PixelPosition {
        self.pixel_size
    }

    pub fn position_cells(&self) -> Coord {
        // self.position_cells
        todo!()
    }

    pub fn position_pixels(&self) -> PixelPosition {
        self.position_pixels
    }

    pub fn dragged(&self) -> bool {
        self.dragged
    }

    pub fn set_current_rotation(&mut self, current_rotation: Polyform<()>) {
        self.current_rotation = current_rotation;
    }

    pub fn set_cell_size(&mut self, cell_size: Coord) {
        // self.cell_size = cell_size;
    }

    pub fn set_pixel_size(&mut self, pixel_size: PixelPosition) {
        self.pixel_size = pixel_size;
    }

    pub fn set_position_cells(&mut self, position_cells: Coord) {
        // self.position_cells = position_cells;
    }

    pub fn set_position_pixels(&mut self, position_pixels: PixelPosition) {
        self.position_pixels = position_pixels;
    }

    pub fn set_dragged(&mut self, dragged: bool) {
        self.dragged = dragged;
    }
}
