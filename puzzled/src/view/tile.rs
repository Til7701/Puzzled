use crate::offset::{CellOffset, PixelOffset};
use adw::prelude::Cast;
use gtk::{Frame, Widget};
use ndarray::Array2;

const TILE_CSS_CLASS_COUNT: usize = 28;

#[derive(Debug, Clone)]
pub struct TileView {
    pub elements_with_offset: Vec<(Widget, PixelOffset)>,
    pub draggables: Vec<Widget>,
    pub position_pixels: PixelOffset,
    pub position_cells: Option<CellOffset>,
    pub tile_base: Array2<bool>,
}

impl TileView {
    pub fn new(id: usize, base: Array2<bool>) -> Self {
        let mut draggables: Vec<Widget> = Vec::new();
        let mut elements_with_offset: Vec<(Widget, PixelOffset)> = Vec::new();

        for ((x, y), value) in base.indexed_iter() {
            if *value {
                let css_classes: Vec<String> = vec![
                    "tile-cell".to_string(),
                    format!("tile-cell-{}", id % TILE_CSS_CLASS_COUNT),
                ];
                let cell = Frame::builder().css_classes(css_classes).build();

                elements_with_offset.push((
                    cell.clone().upcast::<Widget>(),
                    PixelOffset(x as f64, y as f64),
                ));
                draggables.push(cell.upcast::<Widget>());
            }
        }

        TileView {
            elements_with_offset,
            draggables,
            position_pixels: PixelOffset::default(),
            position_cells: None,
            tile_base: base,
        }
    }
}
