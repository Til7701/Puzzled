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
use crate::window::PuzzlemoredaysWindow;
use adw::gdk::Display;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gettextrs::gettext;
use gtk::{
    gio, glib, CssProvider, InterfaceColorScheme, Settings, STYLE_PROVIDER_PRIORITY_APPLICATION,
};
use log::debug;
use std::fmt::Debug;

mod imp {
    use super::*;
    use crate::presenter::main::MainPresenter;
    use crate::state::take_runtime;
    use crate::window::PuzzlemoredaysWindow;
    use simple_logger::SimpleLogger;

    #[derive(Debug, Default)]
    pub struct PuzzlemoredaysApplication {
        pub main_presenter: MainPresenter,
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
            SimpleLogger::new().init().unwrap();
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

        fn shutdown(&self) {
            self.parent_shutdown();
            let runtime = take_runtime();
            runtime.shutdown_background();
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
        let how_to_play_action = gio::ActionEntry::builder("how_to_play")
            .activate(move |app: &Self, _, _| app.show_how_to_play())
            .build();
        self.add_action_entries([quit_action, about_action, how_to_play_action]);
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
            .copyright("© 2026 Tilman Holube\n\n This application comes with absolutely no warranty. See the GNU General Public Licence, version 3 or later for details.")
            .build();

        about.present(Some(&window));
    }

    fn show_how_to_play(&self) {
        const RESOURCE_PATH: &str = "/de/til7701/PuzzleMoreDays/how-to-play-dialog.ui";
        let builder = gtk::Builder::from_resource(RESOURCE_PATH);
        let dialog: adw::Dialog = builder
            .object("how_to_play_dialog")
            .expect("Missing `how_to_play_dialog` in resource");
        if let Some(window) = self.active_window() {
            dialog.present(Some(&window));
        }
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

        debug!(
            "Initial GTK application prefer dark theme is {:?}",
            provider.prefers_color_scheme()
        );
        // This has to be manually updated when the system theme changes, since the CSSProvider
        // somehow does not do that itself.
        // Remove if this is fixed.
        let settings = Settings::default().unwrap();
        settings.connect_gtk_application_prefer_dark_theme_notify({
            let provider = provider.clone();
            move |s| {
                let theme = s.is_gtk_application_prefer_dark_theme();
                debug!("GTK application prefer dark theme changed to {:?}", theme);
                if theme {
                    provider.set_prefers_color_scheme(InterfaceColorScheme::Dark);
                } else {
                    provider.set_prefers_color_scheme(InterfaceColorScheme::Light);
                }
            }
        });
        if settings.is_gtk_application_prefer_dark_theme() {
            provider.set_prefers_color_scheme(InterfaceColorScheme::Dark);
        } else {
            provider.set_prefers_color_scheme(InterfaceColorScheme::Light);
        }
        debug!(
            "GTK application prefer dark theme is {:?}",
            provider.prefers_color_scheme()
        );
    }

    fn setup(&self, window: &PuzzlemoredaysWindow) {
        window.connect_default_width_notify({
            let main_presenter = self.imp().main_presenter.clone();
            move |_| main_presenter.update_layout()
        });
        window.connect_is_active_notify({
            let main_presenter = self.imp().main_presenter.clone();
            move |_| main_presenter.update_layout()
        });
        // Currently, this does not work, since the width is not updated yet when this signal is emitted.
        window.connect_maximized_notify({
            let main_presenter = self.imp().main_presenter.clone();
            move |_| main_presenter.update_layout()
        });

        self.imp().main_presenter.setup(window);
    }
}
