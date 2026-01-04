use crate::offset::{CellOffset, PixelOffset};
use adw::prelude::Cast;
use gtk::prelude::{FrameExt, GridExt};
use gtk::{Frame, Grid, Label, Widget};
use ndarray::Array2;

#[derive(Debug, Clone)]
pub struct TileView {
    pub elements_with_offset: Vec<(Widget, PixelOffset)>,
    pub draggables: Vec<Widget>,
    pub position_pixels: PixelOffset,
    pub position_cells: Option<CellOffset>,
}

impl TileView {
    pub fn new(id: i32, base: Array2<bool>) -> Self {
        let mut draggables: Vec<Widget> = Vec::new();
        let mut elements_with_offset: Vec<(Widget, PixelOffset)> = Vec::new();

        for ((x, y), value) in base.indexed_iter() {
            if *value {
                let css_classes: Vec<String> =
                    vec!["tile-cell".to_string(), format!("tile-cell-{}", id)];
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
        }
    }
}

#[derive(Debug, Clone)]
pub struct BoardView {
    pub parent: Grid,
    pub elements: Vec<Widget>,
}

impl BoardView {
    pub fn new(
        board_layout: Array2<bool>,
        meaning_areas: Array2<i32>,
        meaning_values: Array2<i32>,
    ) -> Result<BoardView, String> {
        if board_layout.dim() != meaning_areas.dim() || board_layout.dim() != meaning_values.dim() {
            return Err(
                "Dimensions of board_layout, meaning_areas, and meaning_values must match"
                    .to_string(),
            );
        }

        let grid = Grid::new();
        grid.set_row_homogeneous(true);
        grid.set_column_homogeneous(true);

        let mut elements: Vec<Widget> = Vec::new();

        for ((x, y), value) in board_layout.indexed_iter() {
            if *value {
                let cell = Frame::new(None);

                if meaning_areas[[x, y]] != -1 {
                    let label = Label::new(Some(&meaning_values[[x, y]].to_string()));
                    cell.set_child(Some(&label));
                } else {
                    return Err(format!(
                        "Meaning area is -1 while board layout is true at position ({}, {})",
                        x, y,
                    ));
                }

                grid.attach(&cell, x as i32, y as i32, 1, 1);
                elements.push(cell.upcast::<Widget>());
            }
        }

        Ok(BoardView {
            parent: grid,
            elements,
        })
    }
}
