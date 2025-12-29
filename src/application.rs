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
use adw::prelude::*;
use adw::subclass::prelude::*;
use gettextrs::gettext;
use gtk::{gio, glib, GestureDrag};

use crate::config::VERSION;
use crate::puzzle::PuzzleConfig;
use crate::{puzzle, PuzzleadayWindow};

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct PuzzleadayApplication {}

    #[glib::object_subclass]
    impl ObjectSubclass for PuzzleadayApplication {
        const NAME: &'static str = "PuzzleadayApplication";
        type Type = super::PuzzleadayApplication;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for PuzzleadayApplication {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.setup_gactions();
            obj.set_accels_for_action("app.quit", &["<control>q"]);
        }
    }

    impl ApplicationImpl for PuzzleadayApplication {
        // We connect to the activate callback to create a window when the application
        // has been launched. Additionally, this callback notifies us when the user
        // tries to launch a "second instance" of the application. When they try
        // to do that, we'll just present any existing window.
        fn activate(&self) {
            let application = self.obj();
            // Get the current window or create one if necessary
            let window = application.active_window().unwrap_or_else(|| {
                let window = PuzzleadayWindow::new(&*application);
                window.upcast()
            });

            application.setup(
                &window
                    .downcast_ref::<PuzzleadayWindow>()
                    .expect("active window is not a PuzzleadayWindow"),
            );

            window.present();
        }
    }

    impl GtkApplicationImpl for PuzzleadayApplication {}
    impl AdwApplicationImpl for PuzzleadayApplication {}
}

glib::wrapper! {
    pub struct PuzzleadayApplication(ObjectSubclass<imp::PuzzleadayApplication>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl PuzzleadayApplication {
    pub fn new(application_id: &str, flags: &gio::ApplicationFlags) -> Self {
        glib::Object::builder()
            .property("application-id", application_id)
            .property("flags", flags)
            .property("resource-base-path", "/de/til7701/PuzzleADay")
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
            .application_name("puzzleaday")
            .application_icon("de.til7701.PuzzleADay")
            .developer_name("Tilman")
            .version(VERSION)
            .developers(vec!["Tilman"])
            // Translators: Replace "translator-credits" with your name/username, and optionally an email or URL.
            .translator_credits(&gettext("translator-credits"))
            .copyright("© 2025 Tilman")
            .build();

        about.present(Some(&window));
    }

    fn setup(&self, window: &PuzzleadayWindow) {
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

            if let (Some(app), Some(window)) = (app_weak.upgrade(), window_weak.upgrade()) {
                app.setup_puzzle_config(&window, puzzle_config);
            }
        });
    }

    fn setup_puzzle_config(&self, window: &PuzzleadayWindow, config: PuzzleConfig) {
        let grid = window.grid();
        let drawing = window.drawing_area();

        // TODO remove existing items from grid

        const GRID_SIZE: i32 = 32;
        let item = gtk::Button::with_label("Drag me");
        grid.put(&item, 0.0, 0.0);

        // create gesture, connect handlers, then add to widget (moved)
        let drag = GestureDrag::new();

        // clones for the drag update closure
        let fixed_clone1 = grid.clone();
        let item_clone1 = item.clone();

        drag.connect_drag_update(move |_, dx, dy| {
            let (x, y) = fixed_clone1.child_position(&item_clone1);
            let new_x = x + dx;
            let new_y = y + dy;
            fixed_clone1.move_(&item_clone1, new_x, new_y);
        });

        // clones for the drag end closure
        let grid_clone2 = grid.clone();
        let item_clone2 = item.clone();

        drag.connect_drag_end(move |_, _, _| {
            let (x, y) = grid_clone2.child_position(&item_clone2);
            let snapped_x = (x as i32 / GRID_SIZE) * GRID_SIZE;
            let snapped_y = (y as i32 / GRID_SIZE) * GRID_SIZE;
            grid_clone2.move_(&item_clone2, snapped_x as f64, snapped_y as f64);
        });

        // move the gesture into the widget (no `&`)
        item.add_controller(drag);

        drawing.set_draw_func(move |_, cr, width, height| {
            cr.set_source_rgba(1.0, 1.0, 1.0, 0.1);
            for x in (0..width).step_by(GRID_SIZE as usize) {
                cr.move_to(x as f64, 0.0);
                cr.line_to(x as f64, height as f64);
            }
            for y in (0..height).step_by(GRID_SIZE as usize) {
                cr.move_to(0.0, y as f64);
                cr.line_to(width as f64, y as f64);
            }
            cr.stroke().unwrap();
        });
    }
}
