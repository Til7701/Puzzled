use crate::app::puzzle_area::puzzle_area::puzzle_area::PuzzleArea;
use crate::components::tile::TileView;
use crate::offset::{CellOffset, PixelOffset};
use adw::gdk::{BUTTON_MIDDLE, BUTTON_SECONDARY};
use adw::subclass::prelude::ObjectSubclassIsExt;
use gtk::prelude::{
    Cast, EventControllerExt, FixedExt, GestureDragExt, GestureSingleExt, WidgetExt,
};
use gtk::{EventController, GestureClick, GestureDrag, PropagationPhase, Widget};
use puzzle_config::TileConfig;

impl PuzzleArea {
    pub fn setup_tile(&self, tile: &TileConfig, tile_id: usize, start_position_cell: &CellOffset) {
        let tile_view = TileView::new(
            tile_id,
            tile.base().clone(),
            tile.color(),
            tile.name().clone(),
        );

        let start_position = {
            let grid_config = self.imp().grid_config.borrow();
            start_position_cell.mul_scalar(grid_config.cell_size_pixel as f64)
        };
        tile_view.set_position_pixels(start_position.into());
        tile_view.set_position_cells(Some(*start_position_cell));

        self.setup_drag_and_drop(tile_id, tile_view.upcast_ref());
        self.setup_tile_rotation_and_flip(tile_id, tile_view.upcast_ref());
        self.add(tile_view.upcast_ref(), &start_position.into());
        self.imp().tiles.borrow_mut().push(tile_view);
    }

    fn setup_drag_and_drop(&self, tile_view_index: usize, draggable: &Widget) {
        let drag = GestureDrag::new();
        drag.set_propagation_phase(PropagationPhase::Capture);

        drag.connect_drag_begin({
            let self_clone = self.clone();
            move |_, _x, _y| {
                {
                    let tiles = self_clone.imp().tiles.borrow();
                    let tile_view = {
                        match tiles.get(tile_view_index) {
                            Some(tv) => tv,
                            None => return,
                        }
                    };
                    tile_view.set_position_cells(None);
                }
                self_clone.run_on_tile_moved();
            }
        });

        drag.connect_drag_update({
            let self_clone = self.clone();
            move |_, dx, dy| {
                let new = {
                    let tiles = self_clone.imp().tiles.borrow();
                    let tile_view = {
                        match tiles.get(tile_view_index) {
                            Some(tv) => tv,
                            None => return,
                        }
                    };
                    let mut pos = tile_view.position_pixels();
                    pos = pos.add_tuple((dx, dy));

                    let max_x = self_clone.width() as f64 - tile_view.width() as f64;
                    let max_y = self_clone.height() as f64 - tile_view.height() as f64;
                    pos.0 = pos.0.clamp(0.0, max_x);
                    pos.1 = pos.1.clamp(0.0, max_y);

                    pos
                };
                self_clone.move_to(tile_view_index, new);
            }
        });

        drag.connect_drag_end({
            let self_clone = self.clone();
            move |_, _, _| {
                let snapped = {
                    let tiles = self_clone.imp().tiles.borrow();
                    let grid_config = self_clone.imp().grid_config.borrow();
                    let grid_size = grid_config.cell_size_pixel;
                    let grid_h_cell_count = grid_config.grid_h_cell_count;
                    let grid_v_cell_count = grid_config.grid_v_cell_count;
                    let tile_view = {
                        match tiles.get(tile_view_index) {
                            Some(tv) => tv,
                            None => return,
                        }
                    };
                    let pos = tile_view.position_pixels();
                    let pos = pos
                        .div_scalar(grid_size as f64)
                        .round()
                        .mul_scalar(grid_size as f64);

                    let max_h_cell_position =
                        grid_h_cell_count as i32 - tile_view.current_rotation().dim().0 as i32;
                    let max_v_cell_position =
                        grid_v_cell_count as i32 - tile_view.current_rotation().dim().1 as i32;
                    let mut new_position_cells =
                        self_clone.calculate_cells_from_pixels(&pos, grid_size as f64);
                    new_position_cells.0 = new_position_cells.0.clamp(0, max_h_cell_position);
                    new_position_cells.1 = new_position_cells.1.clamp(0, max_v_cell_position);

                    tile_view.set_position_cells(Some(new_position_cells));
                    new_position_cells.mul_scalar(grid_size as f64).into()
                };
                self_clone.move_to(tile_view_index, snapped);
                self_clone.run_on_tile_moved();
            }
        });

        draggable.add_controller(drag);
    }

    fn calculate_cells_from_pixels(
        &self,
        pos_pixel: &PixelOffset,
        grid_cell_width_pixel: f64,
    ) -> CellOffset {
        pos_pixel.div_scalar(grid_cell_width_pixel).round().into()
    }

    fn setup_tile_rotation_and_flip(&self, tile_view_index: usize, draggable: &Widget) {
        // Rotation
        let gesture = GestureClick::new();
        gesture.set_button(BUTTON_SECONDARY);
        self.setup_tile_updating_gesture(tile_view_index, &gesture, {
            move |tile_view| tile_view.rotate_clockwise()
        });
        draggable.add_controller(gesture.upcast::<EventController>());

        // Flip
        let gesture = GestureClick::new();
        gesture.set_button(BUTTON_MIDDLE);
        self.setup_tile_updating_gesture(tile_view_index, &gesture, {
            move |tile_view| tile_view.flip_horizontal()
        });
        draggable.add_controller(gesture.upcast::<EventController>());
    }

    fn setup_tile_updating_gesture<F: Fn(&TileView) + 'static>(
        &self,
        tile_view_index: usize,
        gesture: &GestureClick,
        tile_update_function: F,
    ) {
        gesture.connect_pressed({
            let self_clone = self.clone();
            move |_, _n_press, _x, _y| {
                let tiles = self_clone.imp().tiles.borrow();
                let tile_view = tiles.get(tile_view_index);
                let tile_view = match tile_view {
                    Some(tv) => tv,
                    None => return,
                };

                tile_update_function(tile_view);

                self_clone.run_on_tile_moved();
                self_clone.update_tile_layout();
            }
        });
    }

    pub fn update_tile_layout(&self) {
        let len = self.imp().tiles.borrow().len();
        for i in 0..len {
            let pos: PixelOffset = {
                let tiles = self.imp().tiles.borrow();
                let grid_config = self.imp().grid_config.borrow();
                let grid_size = grid_config.cell_size_pixel;
                let tile_view = &tiles[i];

                let dims = tile_view.current_rotation().dim();
                tile_view.set_width_request(dims.0 as i32 * grid_size as i32);
                tile_view.set_height_request(dims.1 as i32 * grid_size as i32);

                if let Some(position_cells) = tile_view.position_cells() {
                    position_cells.mul_scalar(grid_size as f64).into()
                } else {
                    tile_view.position_pixels()
                }
            };
            self.move_to(i, pos);
        }
        let hint_tile = self.imp().hint_tile.borrow();
        let grid_config = self.imp().grid_config.borrow();
        if let Some(tile_view) = hint_tile.as_ref() {
            let grid_size = grid_config.cell_size_pixel;
            let dims = tile_view.current_rotation().dim();
            tile_view.set_width_request(dims.0 as i32 * grid_size as i32);
            tile_view.set_height_request(dims.1 as i32 * grid_size as i32);

            if let Some(position_cells) = tile_view.position_cells() {
                let pos: PixelOffset = position_cells.mul_scalar(grid_size as f64).into();
                self.move_(tile_view, pos.0, pos.1);
                tile_view.set_position_pixels(pos);
            }
        }
    }

    /// Move the tile to the specified (x, y) position in pixels.
    fn move_to(&self, tile_view_index: usize, pos_pixel: PixelOffset) {
        let tiles = self.imp().tiles.borrow();
        if let Some(tile_view) = tiles.get(tile_view_index) {
            self.move_(tile_view, pos_pixel.0, pos_pixel.1);
            tile_view.insert_before(self, None::<&Widget>); // Bring to front
            tile_view.set_position_pixels(pos_pixel);
        }
    }
}
