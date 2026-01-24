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
        let mut tile_view = TileView::new(tile_id, tile.base().clone());

        let start_position = {
            let data = self.data.borrow();
            let grid_config = &data.grid_config;
            start_position_cell.mul_scalar(grid_config.cell_width_pixel as f64)
        };
        tile_view.position_pixels = start_position.into();
        tile_view.position_cells = Some(*start_position_cell);

        for draggable in tile_view.draggables.iter() {
            self.setup_drag_and_drop(tile_id as usize, &draggable, on_position_changed.clone());
            self.setup_tile_rotation_and_flip(
                tile_id as usize,
                &draggable,
                on_position_changed.clone(),
            );
        }
        let mut data = self.data.borrow_mut();
        tile_view
            .elements_with_offset
            .iter()
            .map(|e| e.0.clone())
            .for_each(|w| {
                data.add_to_fixed(&w, &start_position.into());
            });
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
                    tile_view.position_cells = None;
                }
                on_position_changed();
            }
        });

        drag.connect_drag_update({
            let self_clone = self.clone();
            move |_, dx, dy| {
                let new = {
                    let mut data = self_clone.data.borrow_mut();
                    let tile_view = {
                        match data.tile_views.get_mut(tile_view_index) {
                            Some(tv) => tv,
                            None => return,
                        }
                    };
                    let pos = tile_view.position_pixels;
                    pos.add_tuple((dx, dy))
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
                    let pos = tile_view.position_pixels;
                    let pos = pos
                        .div_scalar(grid_size as f64)
                        .round()
                        .mul_scalar(grid_size as f64);

                    let new_position_cells =
                        self_clone.calculate_cells_from_pixels(&pos, grid_size as f64);
                    tile_view.position_cells = Some(new_position_cells);
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
        self.setup_tile_offset_updating_gesture(
            tile_view_index,
            &gesture,
            on_position_changed.clone(),
            {
                move |offset, tile_view| {
                    let (_, cols) = Self::get_dims(&tile_view.elements_with_offset);
                    PixelOffset(-offset.1 + (cols - 1) as f64, offset.0)
                }
            },
        );
        draggable.add_controller(gesture.clone().upcast::<EventController>());

        // Flip
        let gesture = GestureClick::new();
        gesture.set_button(BUTTON_MIDDLE);
        self.setup_tile_offset_updating_gesture(tile_view_index, &gesture, on_position_changed, {
            move |offset, tile_view| {
                let (rows, _) = Self::get_dims(&tile_view.elements_with_offset);
                PixelOffset(-offset.0 + (rows - 1) as f64, offset.1)
            }
        });
        draggable.add_controller(gesture.clone().upcast::<EventController>());
    }

    pub fn get_dims(elements_with_offset: &[(Widget, PixelOffset)]) -> (i32, i32) {
        if elements_with_offset.is_empty() {
            return (0, 0);
        }
        let max_row = elements_with_offset
            .iter()
            .map(|(_, o)| o.0)
            .fold(f64::NEG_INFINITY, f64::max);
        let max_col = elements_with_offset
            .iter()
            .map(|(_, o)| o.1)
            .fold(f64::NEG_INFINITY, f64::max);
        (max_row as i32 + 1, max_col as i32 + 1)
    }

    fn setup_tile_offset_updating_gesture<
        F: Fn(&PixelOffset, &TileView) -> PixelOffset + 'static,
    >(
        &self,
        tile_view_index: usize,
        gesture: &GestureClick,
        on_position_changed: Rc<dyn Fn()>,
        new_offset_function: F,
    ) {
        gesture.connect_pressed({
            let self_clone = self.clone();
            move |_, _n_press, _x, _y| {
                let mut new_offsets: Vec<(Widget, PixelOffset)> = Vec::new();
                let mut data = self_clone.data.borrow_mut();
                let tile_view = data.tile_views.get_mut(tile_view_index);
                let tile_view = match tile_view {
                    Some(tv) => tv,
                    None => return,
                };
                let elements_with_offset = &tile_view.elements_with_offset;
                for (widget, offset) in elements_with_offset.iter() {
                    let new_offset = new_offset_function(offset, &tile_view);
                    new_offsets.push((widget.clone(), new_offset));
                }

                let elements_with_offset = &mut tile_view.elements_with_offset;
                elements_with_offset.clear();
                elements_with_offset.extend(new_offsets.into_iter());
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

                for (widget, _) in tile_view.elements_with_offset.iter() {
                    widget.set_width_request(grid_size as i32);
                    widget.set_height_request(grid_size as i32);
                }
                if let Some(position_cells) = tile_view.position_cells {
                    position_cells.mul_scalar(grid_size as f64).into()
                } else {
                    tile_view.position_pixels
                }
            };
            self.move_to(i, pos);
        }
    }

    /// Move the tile to the specified (x, y) position in pixels.
    fn move_to(&self, tile_view_index: usize, pos_pixel: PixelOffset) {
        let mut data = self.data.borrow_mut();
        let grid_size = data.grid_config.cell_width_pixel as f64;
        let fixed = {
            match &data.fixed {
                Some(fixed) => fixed.clone(),
                None => return,
            }
        };

        if let Some(tile_view) = data.tile_views.get_mut(tile_view_index) {
            for (widget, offset) in tile_view.elements_with_offset.iter() {
                let new = pos_pixel + offset.mul_scalar(grid_size);
                fixed.move_(widget, new.0, new.1);
                widget.insert_before(&fixed, None::<&Widget>); // Bring to front
            }
            tile_view.position_pixels = pos_pixel;
        }
    }
}
