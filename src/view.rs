use crate::offset::Offset;
use adw::prelude::Cast;
use gtk::prelude::{FrameExt, GridExt, WidgetExt};
use gtk::{Grid, Widget};
use ndarray::Array2;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct TileView {
    pub elements_with_offset: Rc<RefCell<Vec<(Widget, Offset)>>>,
    pub draggables: Vec<Widget>,
    pub position_pixels: Rc<RefCell<Offset>>,
    pub position_cells: Rc<RefCell<Option<Offset>>>,
}

impl TileView {
    pub fn new(id: i32, base: Array2<bool>) -> Self {
        let mut draggables: Vec<Widget> = Vec::new();
        let (rows_usize, cols_usize) = base.dim();
        let rows = rows_usize as i32;
        let cols = cols_usize as i32;

        let elements_with_offset: Rc<RefCell<Vec<(Widget, Offset)>>> = {
            let mut elements: Vec<(Widget, Offset)> = Vec::new();

            for r in 0..rows {
                for c in 0..cols {
                    if base[[r as usize, c as usize]] {
                        let cell = gtk::Frame::builder()
                            .css_classes(vec!["tile-cell", format!("tile-cell-{}", id).as_str()])
                            .build();

                        elements.push((
                            cell.clone().upcast::<Widget>(),
                            Offset::new(r as f64, c as f64),
                        ));
                        draggables.push(cell.upcast::<Widget>());
                    }
                }
            }

            Rc::new(RefCell::new(elements))
        };

        let tile_view = TileView {
            elements_with_offset,
            draggables,
            position_pixels: Rc::new(RefCell::new(Offset::default())),
            position_cells: Rc::new(RefCell::new(None)),
        };

        tile_view
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
    ) -> Self {
        let grid = Grid::new();
        grid.set_row_homogeneous(true);
        grid.set_column_homogeneous(true);

        let mut elements: Vec<Widget> = Vec::new();
        let (rows, cols) = board_layout.dim();

        for r in 0..rows {
            for c in 0..cols {
                if board_layout[[r, c]] {
                    let cell = gtk::Frame::new(None);

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
                    elements.push(cell.upcast::<Widget>());
                }
            }
        }

        BoardView {
            parent: grid,
            elements,
        }
    }
}
