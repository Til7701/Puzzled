/* window.rs
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

use adw::subclass::prelude::*;
use gtk::prelude::*;
use gtk::{gio, glib};

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/de/til7701/PuzzleADay/window.ui")]
    pub struct PuzzleadayWindow {
        #[template_child]
        pub grid: TemplateChild<gtk::Fixed>,
        #[template_child]
        pub drawing: TemplateChild<gtk::DrawingArea>,
        #[template_child]
        pub puzzle_selection: TemplateChild<gtk::DropDown>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PuzzleadayWindow {
        const NAME: &'static str = "PuzzleadayWindow";
        type Type = super::PuzzleadayWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for PuzzleadayWindow {}
    impl WidgetImpl for PuzzleadayWindow {}
    impl WindowImpl for PuzzleadayWindow {}
    impl ApplicationWindowImpl for PuzzleadayWindow {}
    impl AdwApplicationWindowImpl for PuzzleadayWindow {}
}

glib::wrapper! {
    pub struct PuzzleadayWindow(ObjectSubclass<imp::PuzzleadayWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,        @implements gio::ActionGroup, gio::ActionMap;
}

impl PuzzleadayWindow {
    pub fn new<P: IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::builder()
            .property("application", application)
            .build()
    }

    pub fn grid(&self) -> gtk::Fixed {
        self.imp().grid.clone()
    }

    pub fn puzzle_selection(&self) -> gtk::DropDown {
        self.imp().puzzle_selection.clone()
    }

    pub fn drawing_area(&self) -> gtk::DrawingArea {
        self.imp().drawing.clone()
    }
}
