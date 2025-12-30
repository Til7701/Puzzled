use crate::application::GRID_SIZE;
use adw::gdk::BUTTON_SECONDARY;
use adw::prelude::{Cast, ListModelExtManual};
use gtk::gdk::BUTTON_MIDDLE;
use gtk::prelude::{FixedExt, FrameExt, GestureSingleExt, GridExt, WidgetExt};
use gtk::{EventController, Fixed, GestureClick, Grid, Widget};
use ndarray::Array2;
use std::cell::{RefCell, RefMut};
use std::rc::Rc;

#[derive(Clone)]
pub struct TileView {
    pub elements_with_offset: Rc<RefCell<Vec<(Widget, f64, f64)>>>,
    pub draggables: Vec<Widget>,
    pub x: Rc<RefCell<f64>>,
    pub y: Rc<RefCell<f64>>,
}

impl TileView {
    pub fn new(id: i32, base: Array2<bool>) -> Self {
        let mut draggables: Vec<Widget> = Vec::new();
        let (rows_usize, cols_usize) = base.dim();
        let rows = rows_usize as i32;
        let cols = cols_usize as i32;

        let elements_with_offset: Rc<RefCell<Vec<(Widget, f64, f64)>>> = {
            let mut elements: Vec<(Widget, f64, f64)> = Vec::new();

            for r in 0..rows {
                for c in 0..cols {
                    if base[[r as usize, c as usize]] {
                        let cell = gtk::Frame::builder()
                            .css_classes(vec!["tile-cell", format!("tile-cell-{}", id).as_str()])
                            .build();
                        cell.set_width_request(GRID_SIZE);
                        cell.set_height_request(GRID_SIZE);

                        elements.push((cell.clone().upcast::<Widget>(), r as f64, c as f64));
                        draggables.push(cell.upcast::<Widget>());
                    }
                }
            }

            Rc::new(RefCell::new(elements))
        };

        for (_, x, y) in elements_with_offset.borrow().iter() {
            println!("Initial Offset: ({}, {})", x, y);
        }
        println!();

        let tile_view = TileView {
            elements_with_offset,
            draggables,
            x: Rc::new(RefCell::new(0.0)),
            y: Rc::new(RefCell::new(0.0)),
        };

        for draggable in tile_view.draggables.iter() {
            tile_view.setup_rotation_for_draggable(draggable);
            tile_view.setup_flip_for_draggable(draggable);
        }

        tile_view
    }

    fn setup_rotation_for_draggable(&self, draggable: &Widget) {
        let gesture = GestureClick::new();
        gesture.set_button(BUTTON_SECONDARY);

        let draggable_clone = draggable.clone();
        let self_clone = self.clone();
        gesture.connect_pressed(move |_, _n_press, _x, _y| {
            let (_, cols) = self_clone.get_rows_cols();
            let mut new_offsets: Vec<(Widget, f64, f64)> = Vec::new();
            let mut elements_with_offset_mut = self_clone.elements_with_offset.borrow_mut();
            for (widget, r_offset, c_offset) in elements_with_offset_mut.iter() {
                let new_r_offset = -(*c_offset) + (cols - 1) as f64;
                let new_c_offset = *r_offset;
                new_offsets.push((widget.clone(), new_r_offset, new_c_offset));
            }

            Self::update_widget_positions(
                &draggable_clone
                    .parent()
                    .unwrap()
                    .downcast::<Fixed>()
                    .unwrap(),
                &mut elements_with_offset_mut,
                new_offsets,
                *self_clone.x.borrow(),
                *self_clone.y.borrow(),
            )
        });

        draggable.add_controller(gesture.clone().upcast::<EventController>());
    }

    fn setup_flip_for_draggable(&self, draggable: &Widget) {
        let gesture = GestureClick::new();
        gesture.set_button(BUTTON_MIDDLE);

        let draggable_clone = draggable.clone();
        let self_clone = self.clone();
        gesture.connect_pressed(move |_, _n_press, _x, _y| {
            let (rows, _) = self_clone.get_rows_cols();
            let mut new_offsets: Vec<(Widget, f64, f64)> = Vec::new();
            let mut elements_with_offset_mut = self_clone.elements_with_offset.borrow_mut();
            for (widget, r_offset, c_offset) in elements_with_offset_mut.iter() {
                let new_r_offset = -(*r_offset) + (rows - 1) as f64;
                let new_c_offset = *c_offset;
                new_offsets.push((widget.clone(), new_r_offset, new_c_offset));
            }

            Self::update_widget_positions(
                &draggable_clone
                    .parent()
                    .unwrap()
                    .downcast::<Fixed>()
                    .unwrap(),
                &mut elements_with_offset_mut,
                new_offsets,
                *self_clone.x.borrow(),
                *self_clone.y.borrow(),
            )
        });

        draggable.add_controller(gesture.clone().upcast::<EventController>());
    }

    fn update_widget_positions(
        fixed: &Fixed,
        original_elements_with_offset: &mut RefMut<Vec<(Widget, f64, f64)>>,
        new_elements_with_offset: Vec<(Widget, f64, f64)>,
        x: f64,
        y: f64,
    ) {
        original_elements_with_offset.clear();
        original_elements_with_offset.extend(new_elements_with_offset.into_iter());

        for (widget, r_offset, c_offset) in original_elements_with_offset.iter() {
            fixed.move_(
                &widget.clone().upcast::<Widget>(),
                x + (*r_offset * GRID_SIZE as f64),
                y + (*c_offset * GRID_SIZE as f64),
            );
        }
    }

    pub fn put(&self, area: &Fixed, x: f64, y: f64) {
        for (widget, r_offset, c_offset) in self.elements_with_offset.borrow().iter() {
            let new_x = x + (r_offset * GRID_SIZE as f64);
            let new_y = y + (c_offset * GRID_SIZE as f64);
            area.put(widget, new_x, new_y);
            self.x.replace(new_x);
            self.y.replace(new_y);
        }
    }

    pub fn move_to(&self, area: &Fixed, x: f64, y: f64) {
        self.x.replace(x);
        self.y.replace(y);
        for (widget, r_offset, c_offset) in self.elements_with_offset.borrow().iter() {
            let new_x = x + (r_offset * GRID_SIZE as f64);
            let new_y = y + (c_offset * GRID_SIZE as f64);
            area.move_(widget, new_x, new_y);
        }
    }

    pub fn get_rows_cols(&self) -> (i32, i32) {
        let rows = self
            .elements_with_offset
            .borrow()
            .iter()
            .map(|(_, r, _)| *r as i32)
            .max()
            .unwrap()
            + 1;
        let cols = self
            .elements_with_offset
            .borrow()
            .iter()
            .map(|(_, _, c)| *c as i32)
            .max()
            .unwrap()
            + 1;
        (rows, cols)
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
