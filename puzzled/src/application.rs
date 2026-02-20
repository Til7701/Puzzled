/* application.rs
 *
 * Copyright 2026 Tilman
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
use crate::global::settings::{Preferences, ShowBoardGridLines};
use crate::global::state::get_state_mut;
use crate::presenter::collection_selection::CollectionSelectionPresenter;
use crate::presenter::main::MainPresenter;
use crate::presenter::puzzle::PuzzlePresenter;
use crate::presenter::puzzle_selection::PuzzleSelectionPresenter;
use crate::puzzles;
use crate::view::tile::{DrawingMode, TileView};
use crate::window::PuzzledWindow;
use adw::gdk::Display;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gettextrs::gettext;
use gtk::{gio, glib, CssProvider, License, Settings, STYLE_PROVIDER_PRIORITY_APPLICATION};
use ndarray::array;
use puzzle_config::ColorConfig;
use std::fmt::Debug;
use std::rc::Rc;

mod imp {
    use super::*;
    use crate::global::runtime::take_runtime;
    use crate::window::PuzzledWindow;

    #[derive(Debug, Default)]
    pub struct PuzzledApplication {}

    #[glib::object_subclass]
    impl ObjectSubclass for PuzzledApplication {
        const NAME: &'static str = "PuzzledApplication";
        type Type = super::PuzzledApplication;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for PuzzledApplication {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.setup_gactions();
            obj.set_accels_for_action("app.quit", &["<control>q"]);
        }
    }

    impl ApplicationImpl for PuzzledApplication {
        // We connect to the activate callback to create a window when the application
        // has been launched. Additionally, this callback notifies us when the user
        // tries to launch a "second instance" of the application. When they try
        // to do that, we'll just present any existing window.
        fn activate(&self) {
            simple_logger::init_with_env().unwrap();
            let application = self.obj();
            // Get the current window or create one if necessary
            let window = application.active_window().unwrap_or_else(|| {
                let window = PuzzledWindow::new(&*application);
                window.upcast()
            });

            application.load_css();
            application.setup(
                &window
                    .downcast_ref::<PuzzledWindow>()
                    .expect("active window is not a PuzzledWindow"),
            );

            window.present();
        }

        fn shutdown(&self) {
            self.parent_shutdown();
            let runtime = take_runtime();
            runtime.shutdown_background();
        }
    }

    impl GtkApplicationImpl for PuzzledApplication {}
    impl AdwApplicationImpl for PuzzledApplication {}
}

glib::wrapper! {
    pub struct PuzzledApplication(ObjectSubclass<imp::PuzzledApplication>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl PuzzledApplication {
    pub fn new(application_id: &str, flags: &gio::ApplicationFlags) -> Self {
        glib::Object::builder()
            .property("application-id", application_id)
            .property("flags", flags)
            .property("resource-base-path", "/de/til7701/Puzzled")
            .build()
    }

    fn setup_gactions(&self) {
        let quit_action = gio::ActionEntry::builder("quit")
            .activate(move |app: &Self, _, _| app.quit())
            .build();
        let about_action = gio::ActionEntry::builder("about")
            .activate(move |app: &Self, _, _| app.show_about())
            .build();
        let how_to_play_action = gio::ActionEntry::builder("how_to_play")
            .activate(move |app: &Self, _, _| app.show_how_to_play())
            .build();
        let preferences = gio::ActionEntry::builder("preferences")
            .activate(move |app: &Self, _, _| app.show_preferences())
            .build();
        self.add_action_entries([quit_action, about_action, how_to_play_action, preferences]);
    }

    fn show_about(&self) {
        let window = self.active_window().unwrap();
        let about = adw::AboutDialog::builder()
            .application_name("Puzzled")
            .application_icon("de.til7701.Puzzled")
            .developer_name("Tilman Holube")
            .version(VERSION)
            .developers(vec!["Tilman Holube", "Jonas Pohl"])
            // Translators: Replace "translator-credits" with your name/username, and optionally an email or URL.
            .translator_credits(&gettext("translator-credits"))
            .copyright("© 2026 Tilman Holube and contributors")
            .license_type(License::Gpl30)
            .website("https://til7701.de/projects/puzzled")
            .issue_url("https://github.com/til7701/til7701.de")
            .build();

        about.present(Some(&window));
    }

    fn show_preferences(&self) {
        const RESOURCE_PATH: &str = "/de/til7701/Puzzled/preferences-dialog.ui";
        let builder = gtk::Builder::from_resource(RESOURCE_PATH);
        let dialog: adw::PreferencesDialog = builder
            .object("preferences_dialog")
            .expect("Missing `preferences_dialog` in resource");

        let show_board_grid_lines: adw::SwitchRow = builder
            .object("show_board_grid_lines")
            .expect("Missing `show_board_grid_lines` in resource");
        let preferences = Preferences::default();
        preferences.bind(ShowBoardGridLines, &show_board_grid_lines, "active");

        if let Some(window) = self.active_window() {
            dialog.present(Some(&window));
        }
    }

    fn show_how_to_play(&self) {
        const RESOURCE_PATH: &str = "/de/til7701/Puzzled/how-to-play-dialog.ui";
        let builder = gtk::Builder::from_resource(RESOURCE_PATH);
        let dialog: adw::Window = builder
            .object("how_to_play_dialog")
            .expect("Missing `how_to_play_dialog` in resource");

        const CELL_SIZE: i32 = 30;

        let overlapping_fixed: gtk::Fixed = builder
            .object("overlapping_fixed")
            .expect("Missing `overlapping_fixed` in resource");
        let left_tile = TileView::new(
            0,
            array![[true, false], [true, true]],
            ColorConfig::default_with_index(0),
        );
        left_tile.set_drawing_mode_at(1, 1, DrawingMode::Overlapping);
        left_tile.set_width_request(CELL_SIZE * 2);
        left_tile.set_height_request(CELL_SIZE * 2);

        let right_tile = TileView::new(
            0,
            array![[true, true], [false, true]],
            ColorConfig::default_with_index(5),
        );
        right_tile.set_width_request(CELL_SIZE * 2);
        right_tile.set_height_request(CELL_SIZE * 2);
        right_tile.set_drawing_mode_at(0, 0, DrawingMode::Overlapping);

        overlapping_fixed.put(&left_tile, 0.0, 0.0);
        overlapping_fixed.put(&right_tile, CELL_SIZE as f64, CELL_SIZE as f64);

        let outside_fixed: gtk::Fixed = builder
            .object("outside_fixed")
            .expect("Missing `outside_fixed` in resource");
        let tile = TileView::new(
            0,
            array![[true, true], [false, true]],
            ColorConfig::default_with_index(0),
        );
        tile.set_drawing_mode_at(1, 1, DrawingMode::OutOfBounds);
        tile.set_width_request(CELL_SIZE * 2);
        tile.set_height_request(CELL_SIZE * 2);
        outside_fixed.put(&tile, 0.0, 0.0);

        let hint_fixed: gtk::Fixed = builder
            .object("hint_fixed")
            .expect("Missing `hint_fixed` in resource");
        let mut color_config = ColorConfig::default_with_index(0);
        color_config = ColorConfig::new(
            color_config.red(),
            color_config.green(),
            color_config.blue(),
            128,
        );
        let tile = TileView::new(0, array![[true, true], [false, true]], color_config);
        tile.set_width_request(CELL_SIZE * 2);
        tile.set_height_request(CELL_SIZE * 2);

        hint_fixed.put(&tile, 0.0, 0.0);

        dialog.present();
    }

    fn load_css(&self) {
        let provider = CssProvider::new();
        provider.load_from_resource("/de/til7701/Puzzled/style.css");

        if let Some(display) = Display::default() {
            gtk::style_context_add_provider_for_display(
                &display,
                &provider,
                STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        } else {
            eprintln!("No default adw::Display available to add CSS provider");
        }

        let settings = Settings::default().unwrap();
        settings.connect_gtk_interface_color_scheme_notify({
            let provider = provider.clone();
            move |s| {
                let theme = s.gtk_interface_color_scheme();
                provider.set_prefers_color_scheme(theme);
            }
        });
        let theme = settings.gtk_interface_color_scheme();
        provider.set_prefers_color_scheme(theme);
    }

    fn setup(&self, window: &PuzzledWindow) {
        puzzles::init();
        let collection_store = puzzles::get_puzzle_collection_store();
        let mut state = get_state_mut();
        state.puzzle_collection = Some(
            collection_store
                .core_puzzle_collections()
                .first()
                .unwrap()
                .clone(),
        );
        drop(collection_store);
        drop(state);

        let mut main_presenter = MainPresenter::new(window);
        main_presenter.register_actions(self);

        let mut puzzle_presenter = PuzzlePresenter::new(window);
        puzzle_presenter.register_actions(self);
        puzzle_presenter.setup(Rc::new({
            let main_presenter = main_presenter.clone();
            move || {
                main_presenter.on_solved();
            }
        }));

        let puzzle_selection_presenter = Rc::new(PuzzleSelectionPresenter::new(
            &window,
            main_presenter.clone(),
        ));
        puzzle_selection_presenter.register_actions(self);
        puzzle_selection_presenter.setup();

        let collection_selection_presenter = Rc::new(CollectionSelectionPresenter::new(
            &window,
            main_presenter.clone(),
        ));
        collection_selection_presenter.register_actions(self);
        collection_selection_presenter.setup();

        main_presenter.setup(
            &collection_selection_presenter,
            &puzzle_selection_presenter,
            &puzzle_presenter,
        );

        if cfg!(debug_assertions) {
            window.add_css_class("devel");
        }
    }
}
