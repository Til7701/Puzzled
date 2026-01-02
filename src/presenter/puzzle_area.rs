use crate::offset::Offset;
use crate::puzzle::tile::Tile;
use crate::puzzle::PuzzleConfig;
use crate::state::get_state;
use crate::view::{BoardView, TileView};
use adw::gdk::{BUTTON_MIDDLE, BUTTON_SECONDARY};
use adw::prelude::Cast;
use gtk::prelude::{EventControllerExt, FixedExt, GestureDragExt, GestureSingleExt, WidgetExt};
use gtk::{EventController, Fixed, GestureClick, GestureDrag, PropagationPhase, Widget};
use std::cell::RefCell;
use std::rc::Rc;

pub const WINDOW_TO_BOARD_RATIO: f64 = 2.5;

#[derive(Debug, Default, Clone)]
pub struct PuzzleAreaPresenter {
    fixed: Rc<RefCell<Option<Fixed>>>,
    elements_in_fixed: Rc<RefCell<Vec<Widget>>>,
    board_view: Rc<RefCell<Option<BoardView>>>,
    tile_views: Rc<RefCell<Vec<TileView>>>,
    grid_config: Rc<RefCell<GridConfig>>,
}

impl PuzzleAreaPresenter {
    pub(crate) fn set_view(&self, view: Fixed) {
        self.fixed.replace(Some(view));
        self.clear_elements();
    }

    /// Set up the puzzle configuration from the current state.
    ///
    /// This adds the board and tiles to the puzzle area based on the current puzzle configuration.
    /// Final positions and layout are handled in `update_layout()`. Before that, all elements are
    /// added at position (0, 0) and will be moved later.
    pub fn setup_puzzle_config_from_state(&self) {
        self.clear_elements();

        let state = get_state();
        let puzzle_config = &state.puzzle_config;

        self.setup_board(puzzle_config);
        let mut position_start = Offset::new(1.0, 1.0);
        for tile in puzzle_config.tiles.iter() {
            self.setup_tile(tile, &position_start);

            let (rows, cols) = Self::get_dims(
                &self
                    .tile_views
                    .borrow()
                    .last()
                    .unwrap()
                    .elements_with_offset
                    .borrow(),
            );
            position_start.x += rows + 1.0;
            if position_start.x > 10.0 {
                position_start.x = 1.0;
                position_start.y += cols + 1.0;
            }
        }

        self.update_layout();
    }

    fn setup_board(&self, puzzle_config: &PuzzleConfig) {
        let board_view = BoardView::new(
            puzzle_config.board_layout.clone(),
            puzzle_config.meaning_areas.clone(),
            puzzle_config.meaning_values.clone(),
        );
        let widget = board_view.parent.clone().upcast::<Widget>();
        self.add_to_fixed(&widget, &Offset::default());

        let grid_h_cell_count =
            (puzzle_config.board_layout.dim().1 as f64 * WINDOW_TO_BOARD_RATIO) as u32;
        let board_offset_x_cells =
            ((grid_h_cell_count - puzzle_config.board_layout.dim().1 as u32) / 2) as f64;
        let mut grid_config = self.grid_config.borrow_mut();
        grid_config.grid_h_cell_count = grid_h_cell_count;
        grid_config.board_offset_cells = Offset::new(board_offset_x_cells, 1.0);
        self.elements_in_fixed.borrow_mut().push(widget.clone());
        self.elements_in_fixed.borrow_mut().push(widget);
        self.board_view.replace(Some(board_view));
    }

    fn setup_tile(&self, tile: &Tile, start_position_cell: &Offset) {
        let tile_view = TileView::new(tile.id, tile.base.clone());

        let grid_config = self.grid_config.borrow();
        let start_position = start_position_cell.mul_scalar(grid_config.cell_width_pixel as f64);
        tile_view.position_pixels.replace(start_position);

        for draggable in tile_view.draggables.iter() {
            self.setup_drag_and_drop(&tile_view, &draggable);
            self.setup_tile_rotation_and_flip(&tile_view, &draggable);
        }
        tile_view
            .elements_with_offset
            .borrow()
            .iter()
            .map(|e| e.0.clone())
            .for_each(|w| {
                self.add_to_fixed(&w, &start_position);
            });
        self.tile_views.borrow_mut().push(tile_view);
    }

    fn setup_drag_and_drop(&self, tile_view: &TileView, draggable: &Widget) {
        let drag = GestureDrag::new();
        drag.set_propagation_phase(PropagationPhase::Capture);

        drag.connect_drag_update({
            let tile_view = tile_view.clone();
            let self_clone = self.clone();
            move |_, dx, dy| {
                let new = {
                    let pos = tile_view.position_pixels.borrow();
                    pos.add_tuple((dx, dy))
                };
                self_clone.move_tile_to(&tile_view, new);
            }
        });

        drag.connect_drag_end({
            let tile_view = tile_view.clone();
            let grid_config = self.grid_config.clone();
            let self_clone = self.clone();
            move |_, _, _| {
                let snapped = {
                    let pos = tile_view.position_pixels.borrow();
                    let grid_size = grid_config.borrow().cell_width_pixel;
                    pos.div_scalar(grid_size as f64)
                        .round()
                        .mul_scalar(grid_size as f64)
                };
                self_clone.move_tile_to(&tile_view, snapped);
                let new_position_cells = self_clone.calculate_cells_from_pixels(&snapped);
                tile_view.position_cells.replace(Some(new_position_cells));
            }
        });

        draggable.add_controller(drag);
    }

    fn calculate_cells_from_pixels(&self, pos_pixel: &Offset) -> Offset {
        let grid_size = self.grid_config.borrow().cell_width_pixel as f64;
        pos_pixel.div_scalar(grid_size).round()
    }

    fn setup_tile_rotation_and_flip(&self, tile_view: &TileView, draggable: &Widget) {
        // Rotation
        let gesture = GestureClick::new();
        gesture.set_button(BUTTON_SECONDARY);
        self.setup_tile_offset_updating_gesture(tile_view, &gesture, {
            let tile_view = tile_view.clone();
            move |offset| {
                let (_, cols) = Self::get_dims(tile_view.elements_with_offset.borrow().as_ref());
                Offset::new(-offset.y + (cols - 1.0), offset.x)
            }
        });
        draggable.add_controller(gesture.clone().upcast::<EventController>());

        // Flip
        let gesture = GestureClick::new();
        gesture.set_button(BUTTON_MIDDLE);
        self.setup_tile_offset_updating_gesture(tile_view, &gesture, {
            let tile_view = tile_view.clone();
            move |offset| {
                let (rows, _) = Self::get_dims(tile_view.elements_with_offset.borrow().as_ref());
                Offset::new(-offset.x + (rows - 1.0), offset.y)
            }
        });
        draggable.add_controller(gesture.clone().upcast::<EventController>());
    }

    fn get_dims(elements_with_offset: &Vec<(Widget, Offset)>) -> (f64, f64) {
        let rows = elements_with_offset
            .iter()
            .map(|(_, o)| o.x)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap()
            + 1.0;
        let cols = elements_with_offset
            .iter()
            .map(|(_, o)| o.y)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap()
            + 1.0;
        (rows, cols)
    }

    fn setup_tile_offset_updating_gesture<F: Fn(&Offset) -> Offset + 'static>(
        &self,
        tile_view: &TileView,
        gesture: &GestureClick,
        new_offset_function: F,
    ) {
        gesture.connect_pressed({
            let tile_view = tile_view.clone();
            let self_clone = self.clone();
            move |_, _n_press, _x, _y| {
                let mut new_offsets: Vec<(Widget, Offset)> = Vec::new();
                let elements_with_offset = tile_view.elements_with_offset.borrow();
                for (widget, offset) in elements_with_offset.iter() {
                    let new_offset = new_offset_function(offset);
                    new_offsets.push((widget.clone(), new_offset));
                }

                drop(elements_with_offset);
                let mut elements_with_offset = tile_view.elements_with_offset.borrow_mut();
                elements_with_offset.clear();
                elements_with_offset.extend(new_offsets.into_iter());
                drop(elements_with_offset);
                self_clone.update_layout();
            }
        });
    }

    /// Update the layout based on the current state.
    ///
    /// This moves the puzzle area elements according to the current window size.
    pub fn update_layout(&self) {
        self.update_cell_width();
        self.update_board_layout();
        self.update_tile_layout();
    }

    fn update_cell_width(&self) {
        let width = {
            let fixed_borrow = self.fixed.borrow();
            let fixed = fixed_borrow.as_ref().unwrap();
            fixed.width()
        };

        let mut grid_config = self.grid_config.borrow_mut();
        grid_config.cell_width_pixel = width as u32 / grid_config.grid_h_cell_count;
    }

    fn update_board_layout(&self) {
        let fixed_borrow = self.fixed.borrow();
        let fixed = fixed_borrow.as_ref().unwrap();
        if let Some(board_view) = self.board_view.borrow().as_ref() {
            let widget = board_view.parent.clone().upcast::<Widget>();
            let grid_config = self.grid_config.borrow();
            let pos = grid_config
                .board_offset_cells
                .mul_scalar(grid_config.cell_width_pixel as f64);
            fixed.move_(&widget, pos.x, pos.y);
            for widget in board_view.elements.iter() {
                widget.set_width_request(grid_config.cell_width_pixel as i32);
                widget.set_height_request(grid_config.cell_width_pixel as i32);
            }
        }
    }

    fn update_tile_layout(&self) {
        let grid_config = self.grid_config.borrow();
        for tile_view in self.tile_views.borrow().iter() {
            for widget in tile_view.elements_with_offset.borrow().iter() {
                widget
                    .0
                    .set_width_request(grid_config.cell_width_pixel as i32);
                widget
                    .0
                    .set_height_request(grid_config.cell_width_pixel as i32);
            }
            let pos = {
                if let Some(position_cells) = tile_view.position_cells.borrow().as_ref() {
                    position_cells.mul_scalar(self.grid_config.borrow().cell_width_pixel as f64)
                } else {
                    tile_view.position_pixels.clone().borrow().clone()
                }
            };
            self.move_tile_to(tile_view, pos);
        }
    }

    /// Move the tile to the specified (x, y) position in pixels.
    fn move_tile_to(&self, tile_view: &TileView, pos_pixel: Offset) {
        let fixed_borrow = self.fixed.borrow();
        let fixed = fixed_borrow.as_ref().unwrap();

        let grid_size = self.grid_config.borrow().cell_width_pixel as f64;
        for (widget, offset) in tile_view.elements_with_offset.borrow().iter() {
            let new = pos_pixel + offset.mul_scalar(grid_size);
            fixed.move_(widget, new.x, new.y);
        }
        tile_view.position_pixels.replace(pos_pixel);
    }

    fn add_to_fixed(&self, widget: &Widget, pos: &Offset) {
        let fixed_borrow = self.fixed.borrow();
        let fixed = fixed_borrow.as_ref().unwrap();
        fixed.put(widget, pos.x, pos.y);
        self.elements_in_fixed.borrow_mut().push(widget.clone());
    }

    fn clear_elements(&self) {
        let fixed_borrow = self.fixed.borrow();
        let fixed = fixed_borrow.as_ref().unwrap();
        self.elements_in_fixed
            .borrow_mut()
            .drain(..)
            .for_each(|e| fixed.remove(&e));
        self.tile_views.borrow_mut().clear();
        self.board_view.replace(None);
    }
}

/// Configuration for the puzzle grid layout.
#[derive(Debug, Default, Clone)]
struct GridConfig {
    grid_h_cell_count: u32,
    cell_width_pixel: u32,
    board_offset_cells: Offset,
}
