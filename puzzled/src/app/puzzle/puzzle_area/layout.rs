use crate::app::puzzle::puzzle_area::PuzzleArea;
use crate::offset::PixelOffset;
use crate::window::{MIN_WINDOW_HEIGHT, MIN_WINDOW_WIDTH};
use adw::glib;
use adw::subclass::prelude::ObjectSubclassIsExt;
use gtk::prelude::{WidgetExt, WidgetExtManual};
use std::cell::Cell;
use std::rc::Rc;

impl PuzzleArea {
    pub fn post_construct_setup_layout(&self) {
        // May have to use window here, since the size may change unpredictably
        self.add_tick_callback({
            let self_clone = self.clone();
            let last = Rc::new(Cell::new((self.width(), self.height())));
            move |window, _| {
                let (w, h) = last.get();
                let window_width = window.width();
                let window_height = window.height();
                if window_width != w || window_height != h {
                    last.set((window_width, window_height));
                    self_clone
                        .imp()
                        .placement_model
                        .borrow()
                        .iter()
                        .for_each(|placement_model| {
                            let available_width_pixel =
                                self.imp().window.get().unwrap().width() as f64;
                            let available_height_pixel = {
                                let mut header_height =
                                    self.imp()
                                        .window
                                        .get()
                                        .unwrap()
                                        .puzzle_area_nav_page()
                                        .header_bar()
                                        .height() as f64;
                                if header_height == 0.0 {
                                    header_height = 40.0;
                                }
                                self.imp().window.get().unwrap().height() as f64 - header_height
                            };
                            placement_model.update_pixel_size(PixelOffset(
                                available_width_pixel,
                                available_height_pixel,
                            ));
                        });
                }
                glib::ControlFlow::Continue
            }
        });
    }

    /// Update the layout based on the current state.
    ///
    /// This moves the puzzle area elements according to the current window size.
    pub fn update_layout(&self) {
        self.update_grid_layout();
        self.set_min_size();
        self.update_board_layout();
        self.update_tile_layout();
    }

    /// Sets the minimum size of the window based on the current grid configuration.
    ///
    /// This has to be set on the window instead of the Fixed, since the AdwBreakpointBin
    /// that everything is wrapped in, does not work well with changing width requests
    /// if the children.
    fn set_min_size(&self) {
        let window = self.imp().window.get().unwrap();
        if !window.outer_view().shows_content() {
            return;
        }

        let min_board_elements_width = self.get_min_element_width();
        let grid_config = self.imp().grid_config.borrow();

        let fixed_min_width = grid_config.min_grid_h_cell_count as i32 * min_board_elements_width;
        window.set_width_request(fixed_min_width.max(MIN_WINDOW_WIDTH));

        let fixed_min_height = grid_config.min_grid_v_cell_count as i32 * min_board_elements_width;
        window.set_height_request(
            (fixed_min_height + window.puzzle_area_nav_page().header_bar().height())
                .max(MIN_WINDOW_HEIGHT),
        );
    }
}
