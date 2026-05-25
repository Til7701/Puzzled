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
            let last = Rc::new(Cell::new((self.width(), self.height())));
            move |puzzle_area, _| {
                let (last_width, last_height) = last.get();
                let width = puzzle_area.width();
                let height = puzzle_area.height();
                if width != last_width || height != last_height {
                    last.set((width, height));
                    puzzle_area.update_layout();
                }
                glib::ControlFlow::Continue
            }
        });
    }

    /// Update the layout based on the current state.
    ///
    /// This moves the puzzle area elements according to the current window size.
    pub fn update_layout(&self) {
        let window = self.imp().window.get().unwrap();
        if !window.outer_view().shows_content() {
            return;
        }
        let size = PixelOffset(
            window.width() as f64,
            (window.height() - window.puzzle_area_nav_page().header_bar().height()) as f64,
        );

        let placement_model = { self.imp().placement_model.borrow().clone() };
        if let Some(placement_model) = placement_model {
            let min_cell_size = self.get_min_element_width();
            placement_model.update_pixel_size(size, min_cell_size);
            self.set_min_size();
            self.update_board_layout();
            self.update_tile_layout();
            self.update_hint_tile_layout();
        }
    }

    /// Sets the minimum size of the window based on the current grid configuration.
    ///
    /// This has to be set on the window instead of the Fixed, since the AdwBreakpointBin
    /// that everything is wrapped in, does not work well with changing width requests
    /// of the children.
    fn set_min_size(&self) {
        let window = self.imp().window.get().unwrap();
        if !window.outer_view().shows_content() {
            return;
        }

        let placement_borrow = self.imp().placement_model.borrow();
        let placement_model = placement_borrow.as_ref().unwrap();
        let fixed_min = placement_model.min_area_size();

        window.set_width_request((fixed_min.0 as i32).max(MIN_WINDOW_WIDTH));
        window.set_height_request(
            (fixed_min.1 as i32 + window.puzzle_area_nav_page().header_bar().height())
                .max(MIN_WINDOW_HEIGHT),
        );
    }
}
