use crate::application::GRID_SIZE;
use adw::gdk::{BUTTON_MIDDLE, BUTTON_SECONDARY};
use adw::prelude::Cast;
use gtk::prelude::{FrameExt, GestureSingleExt, GridExt, WidgetExt};
use gtk::{EventController, GestureClick, Grid, Widget};
use ndarray::Array2;
use std::cell::RefCell;
use std::rc::Rc;

pub struct TileView {
    pub parent: Grid,
    pub draggables: Vec<Widget>,
    positions: Rc<RefCell<Vec<(Widget, i32, i32)>>>,
    rows: i32,
    cols: i32,
}

impl TileView {
    pub fn new(id: i32, base: Array2<bool>) -> Self {
        let grid = Grid::new();
        grid.set_row_homogeneous(true);
        grid.set_column_homogeneous(true);

        let (rows_usize, cols_usize) = base.dim();
        let rows = rows_usize as i32;
        let cols = cols_usize as i32;

        let positions: Rc<RefCell<Vec<(Widget, i32, i32)>>> = Rc::new(RefCell::new(Vec::new()));
        let mut draggables: Vec<Widget> = Vec::new();

        for r in 0..rows {
            for c in 0..cols {
                if base[[r as usize, c as usize]] {
                    let cell = gtk::Frame::builder()
                        .css_classes(vec!["tile-cell", format!("tile-cell-{}", id).as_str()])
                        .build();
                    cell.set_width_request(GRID_SIZE);
                    cell.set_height_request(GRID_SIZE);

                    grid.attach(&cell, c, r, 1, 1);

                    positions
                        .borrow_mut()
                        .push((cell.clone().upcast::<Widget>(), r, c));
                    draggables.push(cell.upcast::<Widget>());
                }
            }
        }

        for draggable in draggables.iter() {
            Self::setup_rotation_for_draggable(draggable, &grid, positions.clone(), rows, cols);
            Self::setup_flip_for_draggable(draggable, &grid, positions.clone(), rows, cols);
        }

        TileView {
            parent: grid,
            draggables,
            positions,
            rows,
            cols,
        }
    }

    fn setup_rotation_for_draggable(
        draggable: &Widget,
        grid: &Grid,
        positions: Rc<RefCell<Vec<(Widget, i32, i32)>>>,
        rows: i32,
        cols: i32,
    ) {
        let positions_clone = positions.clone();
        let grid_clone = grid.clone();
        let dims = Rc::new(RefCell::new((rows, cols)));
        let dims_clone = dims.clone();

        let gesture = GestureClick::new();
        gesture.set_button(BUTTON_SECONDARY);
        gesture.connect_pressed(move |_, _n_press, _x, _y| {
            let mut pos_ref = positions_clone.borrow_mut();
            // take old positions
            let old_positions = pos_ref.drain(..).collect::<Vec<_>>();
            let (rows_now, _cols_now) = {
                let d = dims_clone.borrow();
                (d.0, d.1)
            };

            let mut new_positions: Vec<(Widget, i32, i32)> =
                Vec::with_capacity(old_positions.len());

            for (widget, r, c) in old_positions.into_iter() {
                // compute rotated coords (90° clockwise)
                let new_r = c;
                let new_c = rows_now - 1 - r;

                // remove and reattach at new position
                grid_clone.remove(&widget);
                grid_clone.attach(&widget, new_c, new_r, 1, 1);

                new_positions.push((widget, new_r, new_c));
            }

            // swap dims (rows, cols)
            let mut dmut = dims_clone.borrow_mut();
            let (r_old, c_old) = *dmut;
            *dmut = (c_old, r_old);

            // restore positions
            pos_ref.extend(new_positions.into_iter());
        });

        draggable.add_controller(gesture.clone().upcast::<EventController>());
    }
    fn setup_flip_for_draggable(
        draggable: &Widget,
        grid: &Grid,
        positions: Rc<RefCell<Vec<(Widget, i32, i32)>>>,
        rows: i32,
        cols: i32,
    ) {
        let positions_clone = positions.clone();
        let grid_clone = grid.clone();
        let dims = Rc::new(RefCell::new((rows, cols)));
        let dims_clone = dims.clone();

        let gesture = GestureClick::new();
        gesture.set_button(BUTTON_MIDDLE);
        gesture.connect_pressed(move |_, _n_press, _x, _y| {
            let mut pos_ref = positions_clone.borrow_mut();
            // take old positions
            let old_positions = pos_ref.drain(..).collect::<Vec<_>>();
            let (rows_now, cols_now) = {
                let d = dims_clone.borrow();
                (d.0, d.1)
            };

            let mut new_positions: Vec<(Widget, i32, i32)> =
                Vec::with_capacity(old_positions.len());

            for (widget, r, c) in old_positions.into_iter() {
                // compute flipped coords (horizontal flip)
                let new_r = r;
                let new_c = cols_now - 1 - c;

                // remove and reattach at new position
                grid_clone.remove(&widget);
                grid_clone.attach(&widget, new_c, new_r, 1, 1);

                new_positions.push((widget, new_r, new_c));
            }

            // dims remain the same for flip

            // restore positions
            pos_ref.extend(new_positions.into_iter());
        });

        draggable.add_controller(gesture.clone().upcast::<EventController>());
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
