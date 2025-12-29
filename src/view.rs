use crate::application::GRID_SIZE;
use adw::prelude::Cast;
use gtk::prelude::{FrameExt, GridExt, WidgetExt};
use gtk::{Grid, Widget};
use ndarray::Array2;

pub struct TileView {
    pub parent: Grid,
    pub draggables: Vec<Widget>,
}

impl TileView {
    pub fn new(id: i32, base: Array2<bool>) -> Self {
        let grid = Grid::new();
        grid.set_row_homogeneous(true);
        grid.set_column_homogeneous(true);

        let (rows, cols) = base.dim();
        let mut draggables: Vec<Widget> = Vec::new();

        for r in 0..rows {
            for c in 0..cols {
                if base[[r, c]] {
                    let cell = gtk::Frame::builder()
                        .css_classes(vec!["tile-cell", format!("tile-cell-{}", id).as_str()])
                        .build();
                    cell.set_width_request(GRID_SIZE);
                    cell.set_height_request(GRID_SIZE);

                    grid.attach(&cell, c as i32, r as i32, 1, 1);
                    draggables.push(cell.upcast::<Widget>());
                }
            }
        }

        TileView {
            parent: grid,
            draggables,
        }
    }
}
pub struct BoardView {
    pub parent: Grid,
}

impl BoardView {
    pub fn new(
        board_layout: Array2<bool>,
        meaning_areas: Array2<i32>,
        meaning_values: Array2<i32>,
    ) -> Self {
        let grid = Grid::new();
        grid.set_row_homogeneous(true);
        grid.set_column_homogeneous(true);

        let (rows, cols) = board_layout.dim();

        for r in 0..rows {
            for c in 0..cols {
                if board_layout[[r, c]] {
                    let cell = gtk::Frame::new(None);
                    cell.set_width_request(GRID_SIZE);
                    cell.set_height_request(GRID_SIZE);

                    if meaning_areas[[r, c]] != -1 {
                        let label = gtk::Label::new(Some(&meaning_values[[r, c]].to_string()));
                        cell.set_child(Some(&label));
                    } else {
                        panic!(
                            "Meaning area is -1 while board layout is true at position ({}, {})",
                            r, c
                        );
                    }

                    grid.attach(&cell, c as i32, r as i32, 1, 1);
                }
            }
        }

        BoardView { parent: grid }
    }
}
