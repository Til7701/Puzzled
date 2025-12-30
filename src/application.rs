/* application.rs
 *
 * Copyright 2025 Tilman
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
use crate::config::VERSION;
use crate::puzzle;
use crate::puzzle::tile::Tile;
use crate::puzzle::PuzzleConfig;
use crate::state::get_state;
use crate::view::TileView;
use crate::window::PuzzlemoredaysWindow;
use adw::gdk::Display;
use adw::glib::property::PropertyGet;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gettextrs::gettext;
use gtk::{
    gio, glib, CssProvider, Fixed, GestureDrag, PropagationPhase,
    Widget, STYLE_PROVIDER_PRIORITY_APPLICATION,
};
use std::cell::RefCell;

pub const GRID_SIZE: i32 = 32;

mod imp {
    use super::*;
    use crate::window::PuzzlemoredaysWindow;
    use std::cell::RefCell;

    #[derive(Debug, Default)]
    pub struct PuzzlemoredaysApplication {
        pub widgets_in_grid: RefCell<Vec<Widget>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PuzzlemoredaysApplication {
        const NAME: &'static str = "PuzzlemoredaysApplication";
        type Type = super::PuzzlemoredaysApplication;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for PuzzlemoredaysApplication {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.setup_gactions();
            obj.set_accels_for_action("app.quit", &["<control>q"]);
        }
    }

    impl ApplicationImpl for PuzzlemoredaysApplication {
        // We connect to the activate callback to create a window when the application
        // has been launched. Additionally, this callback notifies us when the user
        // tries to launch a "second instance" of the application. When they try
        // to do that, we'll just present any existing window.
        fn activate(&self) {
            let application = self.obj();
            // Get the current window or create one if necessary
            let window = application.active_window().unwrap_or_else(|| {
                let window = PuzzlemoredaysWindow::new(&*application);
                window.upcast()
            });

            application.load_css();
            application.setup(
                &window
                    .downcast_ref::<PuzzlemoredaysWindow>()
                    .expect("active window is not a PuzzlemoredaysWindow"),
            );

            window.present();
        }
    }

    impl GtkApplicationImpl for PuzzlemoredaysApplication {}
    impl AdwApplicationImpl for PuzzlemoredaysApplication {}
}

glib::wrapper! {
    pub struct PuzzlemoredaysApplication(ObjectSubclass<imp::PuzzlemoredaysApplication>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl PuzzlemoredaysApplication {
    pub fn new(application_id: &str, flags: &gio::ApplicationFlags) -> Self {
        glib::Object::builder()
            .property("application-id", application_id)
            .property("flags", flags)
            .property("resource-base-path", "/de/til7701/PuzzleMoreDays")
            .build()
    }

    fn setup_gactions(&self) {
        let quit_action = gio::ActionEntry::builder("quit")
            .activate(move |app: &Self, _, _| app.quit())
            .build();
        let about_action = gio::ActionEntry::builder("about")
            .activate(move |app: &Self, _, _| app.show_about())
            .build();
        self.add_action_entries([quit_action, about_action]);
    }

    fn show_about(&self) {
        let window = self.active_window().unwrap();
        let about = adw::AboutDialog::builder()
            .application_name("Puzzle More Days")
            .application_icon("de.til7701.PuzzleMoreDays")
            .developer_name("Tilman Holube")
            .version(VERSION)
            .developers(vec!["Tilman Holube"])
            // Translators: Replace "translator-credits" with your name/username, and optionally an email or URL.
            .translator_credits(&gettext("translator-credits"))
            .copyright("© 2025 Tilman Holube")
            .build();

        about.present(Some(&window));
    }

    fn load_css(&self) {
        let provider = CssProvider::new();
        provider.load_from_resource("/de/til7701/PuzzleMoreDays/style.css");

        if let Some(display) = Display::default() {
            gtk::style_context_add_provider_for_display(
                &display,
                &provider,
                STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        } else {
            eprintln!("No default adw::Display available to add CSS provider");
        }
    }

    fn setup(&self, window: &PuzzlemoredaysWindow) {
        let puzzle_selection = window.puzzle_selection();
        puzzle_selection.set_selected(0);
        let app_weak = self.downgrade();
        let window_weak = window.downgrade();

        puzzle_selection.connect_selected_notify(move |dropdown| {
            let index = dropdown.selected();
            let puzzle_config = match index {
                0 => puzzle::get_default_config(),
                1 => puzzle::get_year_config(),
                _ => panic!("Unknown puzzle selection index: {}", index),
            };
            get_state().puzzle_config = puzzle_config;

            if let (Some(app), Some(window)) = (app_weak.upgrade(), window_weak.upgrade()) {
                app.setup_puzzle_config(&window);
            }
        });

        self.setup_puzzle_config(window);
    }

    fn setup_puzzle_config(&self, window: &PuzzlemoredaysWindow) {
        let grid = window.grid();
        let drawing = window.drawing_area();
        let mut widgets_in_grid = self.imp().widgets_in_grid.borrow_mut();

        widgets_in_grid
            .iter()
            .for_each(|widget: &Widget| grid.remove(widget));
        widgets_in_grid.clear();

        self.setup_board(&grid, &get_state().puzzle_config, &mut widgets_in_grid);

        let puzzle_config = &get_state().puzzle_config;
        for tile in &puzzle_config.tiles {
            self.setup_tile(&grid, tile, &mut widgets_in_grid);
        }

        // drawing.set_draw_func(move |_, cr, width, height| {
        //     cr.set_source_rgba(1.0, 1.0, 1.0, 0.1);
        //     for x in (0..width).step_by(GRID_SIZE as usize) {
        //         cr.move_to(x as f64, 0.0);
        //         cr.line_to(x as f64, height as f64);
        //     }
        //     for y in (0..height).step_by(GRID_SIZE as usize) {
        //         cr.move_to(0.0, y as f64);
        //         cr.line_to(width as f64, y as f64);
        //     }
        //     cr.stroke().unwrap();
        // });
    }

    fn setup_tile(&self, grid: &Fixed, tile: &Tile, widgets_in_grid: &mut Vec<Widget>) {
        let tile_view = TileView::new(tile.id, tile.base.clone());
        tile_view.put(grid, 0.0, 0.0);
        tile_view.move_to(grid, 0.0, 0.0);
        for draggable in tile_view.draggables.iter() {
            self.setup_drag_and_drop(&tile_view, &draggable, grid);
        }
        tile_view
            .elements_with_offset
            .borrow()
            .iter()
            .map(|e| e.0.clone())
            .for_each(|w| {
                widgets_in_grid.push(w);
            });
    }

    fn setup_drag_and_drop(&self, tile_view: &TileView, draggable: &Widget, fixed: &Fixed) {
        let drag = GestureDrag::new();
        drag.set_propagation_phase(PropagationPhase::Capture);

        let fixed_clone1 = fixed.clone();
        let tile_view_clone = tile_view.clone();
        drag.connect_drag_update(move |_, dx, dy| {
            let (new_x, new_y) = {
                let (x, y) = (*tile_view_clone.x.borrow(), *tile_view_clone.y.borrow());
                let new_x = x + dx;
                let new_y = y + dy;
                (new_x, new_y)
            };
            tile_view_clone.move_to(&fixed_clone1, new_x, new_y);
        });

        let fixed_clone2 = fixed.clone();
        let tile_view_clone = tile_view.clone();
        drag.connect_drag_end(move |_, _, _| {
            let (snapped_x, snapped_y) = {
                let (x, y) = (*tile_view_clone.x.borrow(), *tile_view_clone.y.borrow());
                let snapped_x = (x / GRID_SIZE as f64).round() * GRID_SIZE as f64;
                let snapped_y = (y / GRID_SIZE as f64).round() * GRID_SIZE as f64;
                (snapped_x, snapped_y)
            };
            tile_view_clone.move_to(&fixed_clone2, snapped_x, snapped_y);
        });

        draggable.add_controller(drag);
    }

    fn setup_board(
        &self,
        grid: &Fixed,
        puzzle_config: &PuzzleConfig,
        widgets_in_grid: &mut Vec<Widget>,
    ) {
        let board_view = crate::view::BoardView::new(
            puzzle_config.board_layout.clone(),
            puzzle_config.meaning_areas.clone(),
            puzzle_config.meaning_values.clone(),
        );
        let widget = board_view.parent.upcast::<Widget>();
        grid.put(&widget, 0.0, 0.0);
        widgets_in_grid.push(widget);
    }
}
