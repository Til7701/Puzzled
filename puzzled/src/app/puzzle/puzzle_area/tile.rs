use crate::app::components::tile::TileView;
use crate::app::puzzle::puzzle_area::PuzzleArea;
use crate::offset::{CellOffset, PixelOffset};
use adw::gdk::{BUTTON_MIDDLE, BUTTON_SECONDARY};
use adw::subclass::prelude::ObjectSubclassIsExt;
use gtk::prelude::{
    Cast, EventControllerExt, FixedExt, GestureDragExt, GestureSingleExt, WidgetExt,
};
use gtk::{EventController, GestureClick, GestureDrag, PropagationPhase, Widget};
use puzzle_config::TileConfig;

impl PuzzleArea {
    pub fn setup_tile(&self, tile: &TileConfig, tile_id: usize) {
        let tile_view = TileView::new(
            tile_id,
            tile.base().clone(),
            tile.color(),
            tile.name().clone(),
        );

        self.setup_drag_and_drop(tile_id, tile_view.upcast_ref());
        self.setup_tile_rotation_and_flip(tile_id, tile_view.upcast_ref());
        self.add(tile_view.upcast_ref(), &PixelOffset(0.0, 0.0));
        self.imp().tiles.borrow_mut().push(tile_view);
    }

    fn setup_drag_and_drop(&self, tile_view_index: usize, draggable: &Widget) {
        let drag = GestureDrag::new();
        drag.set_propagation_phase(PropagationPhase::Capture);

        drag.connect_drag_begin({
            let self_clone = self.clone();
            move |_, _x, _y| {
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
                    let mut pos: PixelOffset = self_clone.child_position(tile_view).into();
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
                let tiles = self_clone.imp().tiles.borrow();
                let tile_view = {
                    match tiles.get(tile_view_index) {
                        Some(tv) => tv,
                        None => return,
                    }
                };
                let pos: PixelOffset = self_clone.child_position(tile_view).into();
                self_clone
                    .imp()
                    .placement_model
                    .borrow()
                    .as_ref()
                    .unwrap()
                    .update_tile_pixel_position(tile_view_index, pos);
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
            let self_clone = self.clone();
            move |tile_view| {
                tile_view.rotate_clockwise();
                self_clone
                    .imp()
                    .placement_model
                    .borrow()
                    .as_ref()
                    .unwrap()
                    .update_tile_shape(tile_view.id(), tile_view.current_rotation().clone());
            }
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
            }
        });
    }

    pub fn update_tile_layout(&self) {
        let len = self.imp().tiles.borrow().len();
        for i in 0..len {
            let pos: Option<PixelOffset> = {
                let tiles = self.imp().tiles.borrow();
                let placement_borrow = self.imp().placement_model.borrow();
                let placement_model = placement_borrow.as_ref().unwrap();
                let tile_view = &tiles[i];
                let size = placement_model.tile_size(i);

                tile_view.set_width_request(size.0 as i32);
                tile_view.set_height_request(size.1 as i32);

                placement_model.tile_pixel_position(i)
            };
            if let Some(pos) = pos {
                self.move_to(i, pos);
            }
        }
    }

    /// Move the tile to the specified (x, y) position in pixels.
    fn move_to(&self, tile_view_index: usize, pos_pixel: PixelOffset) {
        let tiles = self.imp().tiles.borrow();
        if let Some(tile_view) = tiles.get(tile_view_index) {
            self.move_(tile_view, pos_pixel.0, pos_pixel.1);
            tile_view.insert_before(self, None::<&Widget>); // Bring to front
        }
    }
}
