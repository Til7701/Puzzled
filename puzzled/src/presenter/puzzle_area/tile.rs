use crate::offset::{CellOffset, PixelOffset};
use crate::presenter::puzzle_area::PuzzleAreaData;
use crate::view::tile::TileView;
use adw::gdk::{BUTTON_MIDDLE, BUTTON_SECONDARY};
use gtk::prelude::{
    Cast, EventControllerExt, FixedExt, GestureDragExt, GestureSingleExt, WidgetExt,
};
use gtk::{EventController, GestureClick, GestureDrag, PropagationPhase, Widget};
use puzzle_config::TileConfig;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Default, Clone)]
pub struct TilePresenter {
    data: Rc<RefCell<PuzzleAreaData>>,
}

impl TilePresenter {
    pub fn set_data(&mut self, data: Rc<RefCell<PuzzleAreaData>>) {
        self.data = data;
    }

    pub fn setup(
        &self,
        tile: &TileConfig,
        tile_id: usize,
        start_position_cell: &CellOffset,
        on_position_changed: Rc<dyn Fn()>,
    ) {
        let tile_view = TileView::new(tile_id, tile.base().clone(), tile.color());

        let start_position = {
            let data = self.data.borrow();
            let grid_config = &data.grid_config;
            start_position_cell.mul_scalar(grid_config.cell_width_pixel as f64)
        };
        tile_view.set_position_pixels(start_position.into());
        tile_view.set_position_cells(Some(*start_position_cell));

        self.setup_drag_and_drop(tile_id, tile_view.upcast_ref(), on_position_changed.clone());
        self.setup_tile_rotation_and_flip(
            tile_id,
            tile_view.upcast_ref(),
            on_position_changed.clone(),
        );
        let mut data = self.data.borrow_mut();
        data.add_to_fixed(tile_view.upcast_ref(), &start_position.into());
        data.tile_views.push(tile_view);
    }

    fn setup_drag_and_drop(
        &self,
        tile_view_index: usize,
        draggable: &Widget,
        on_position_changed: Rc<dyn Fn()>,
    ) {
        let drag = GestureDrag::new();
        drag.set_propagation_phase(PropagationPhase::Capture);

        drag.connect_drag_begin({
            let self_clone = self.clone();
            let on_position_changed = on_position_changed.clone();
            move |_, _x, _y| {
                {
                    let mut data = self_clone.data.borrow_mut();
                    let tile_view = {
                        match data.tile_views.get_mut(tile_view_index) {
                            Some(tv) => tv,
                            None => return,
                        }
                    };
                    tile_view.set_position_cells(None);
                }
                on_position_changed();
            }
        });

        drag.connect_drag_update({
            let self_clone = self.clone();
            move |_, dx, dy| {
                let new = {
                    let data = self_clone.data.borrow();
                    let tile_view = {
                        match data.tile_views.get(tile_view_index) {
                            Some(tv) => tv,
                            None => return,
                        }
                    };
                    let mut pos = tile_view.position_pixels();
                    pos = pos.add_tuple((dx, dy));

                    if pos.0 < 0.0 {
                        pos.0 = 0.0;
                    }
                    if pos.1 < 0.0 {
                        pos.1 = 0.0;
                    }

                    let fixed = &data.fixed;
                    if pos.0 + tile_view.width() as f64 > fixed.width() as f64 {
                        pos.0 = fixed.width() as f64 - tile_view.width() as f64;
                    }
                    if pos.1 + tile_view.height() as f64 > fixed.height() as f64 {
                        pos.1 = fixed.height() as f64 - tile_view.height() as f64;
                    }

                    pos
                };
                self_clone.move_to(tile_view_index, new);
            }
        });

        drag.connect_drag_end({
            let self_clone = self.clone();
            move |_, _, _| {
                let snapped = {
                    let mut data = self_clone.data.borrow_mut();
                    let grid_size = data.grid_config.cell_width_pixel;
                    let tile_view = {
                        match data.tile_views.get_mut(tile_view_index) {
                            Some(tv) => tv,
                            None => return,
                        }
                    };
                    let pos = tile_view.position_pixels();
                    let pos = pos
                        .div_scalar(grid_size as f64)
                        .round()
                        .mul_scalar(grid_size as f64);

                    let new_position_cells =
                        self_clone.calculate_cells_from_pixels(&pos, grid_size as f64);
                    tile_view.set_position_cells(Some(new_position_cells));
                    pos
                };
                self_clone.move_to(tile_view_index, snapped);
                on_position_changed();
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

    fn setup_tile_rotation_and_flip(
        &self,
        tile_view_index: usize,
        draggable: &Widget,
        on_position_changed: Rc<dyn Fn()>,
    ) {
        // Rotation
        let gesture = GestureClick::new();
        gesture.set_button(BUTTON_SECONDARY);
        self.setup_tile_updating_gesture(tile_view_index, &gesture, on_position_changed.clone(), {
            move |tile_view| tile_view.rotate_clockwise()
        });
        draggable.add_controller(gesture.clone().upcast::<EventController>());

        // Flip
        let gesture = GestureClick::new();
        gesture.set_button(BUTTON_MIDDLE);
        self.setup_tile_updating_gesture(tile_view_index, &gesture, on_position_changed, {
            move |tile_view| tile_view.flip_horizontal()
        });
        draggable.add_controller(gesture.clone().upcast::<EventController>());
    }

    fn setup_tile_updating_gesture<F: Fn(&TileView) -> () + 'static>(
        &self,
        tile_view_index: usize,
        gesture: &GestureClick,
        on_position_changed: Rc<dyn Fn()>,
        tile_update_function: F,
    ) {
        gesture.connect_pressed({
            let self_clone = self.clone();
            move |_, _n_press, _x, _y| {
                let mut data = self_clone.data.borrow_mut();
                let tile_view = data.tile_views.get_mut(tile_view_index);
                let tile_view = match tile_view {
                    Some(tv) => tv,
                    None => return,
                };

                tile_update_function(&tile_view);

                drop(data);
                on_position_changed();
                self_clone.update_layout();
            }
        });
    }

    pub fn update_layout(&self) {
        let len = {
            let data = self.data.borrow();
            data.tile_views.len()
        };
        for i in 0..len {
            let pos: PixelOffset = {
                let mut data = self.data.borrow_mut();
                let grid_size = data.grid_config.cell_width_pixel;
                let tile_view = &mut data.tile_views[i];

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
        let data = self.data.borrow();
        if let Some(tile_view) = &data.hint_tile_view {
            let grid_size = data.grid_config.cell_width_pixel;
            let dims = tile_view.current_rotation().dim();
            tile_view.set_width_request(dims.0 as i32 * grid_size as i32);
            tile_view.set_height_request(dims.1 as i32 * grid_size as i32);

            if let Some(position_cells) = tile_view.position_cells() {
                let pos: PixelOffset = position_cells.mul_scalar(grid_size as f64).into();
                let fixed = &data.fixed;
                fixed.move_(tile_view, pos.0, pos.1);
                tile_view.set_position_pixels(pos);
            }
        }
    }

    /// Move the tile to the specified (x, y) position in pixels.
    fn move_to(&self, tile_view_index: usize, pos_pixel: PixelOffset) {
        let mut data = self.data.borrow_mut();
        let fixed = &data.fixed.clone();

        if let Some(tile_view) = data.tile_views.get_mut(tile_view_index) {
            fixed.move_(tile_view, pos_pixel.0, pos_pixel.1);
            tile_view.insert_before(fixed, None::<&Widget>); // Bring to front
            tile_view.set_position_pixels(pos_pixel);
        }
    }
}
