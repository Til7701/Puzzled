/* window.rs
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
use adw::subclass::prelude::*;
use gtk::prelude::*;
use gtk::{gio, glib};

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/de/til7701/PuzzleMoreDays/window.ui")]
    pub struct PuzzlemoredaysWindow {
        #[template_child]
        pub navigation_view: TemplateChild<adw::NavigationView>,
        #[template_child]
        pub core_collection_list: TemplateChild<gtk::ListBox>,
        #[template_child]
        pub community_collection_list: TemplateChild<gtk::ListBox>,
        #[template_child]
        pub load_collection_button_row: TemplateChild<adw::ButtonRow>,
        #[template_child]
        pub puzzle_list: TemplateChild<gtk::ListBox>,
        #[template_child]
        pub grid: TemplateChild<gtk::Fixed>,
        #[template_child]
        pub puzzle_info_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub target_selection_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub solver_state: TemplateChild<gtk::Button>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PuzzlemoredaysWindow {
        const NAME: &'static str = "PuzzlemoredaysWindow";
        type Type = super::PuzzlemoredaysWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for PuzzlemoredaysWindow {}
    impl WidgetImpl for PuzzlemoredaysWindow {}
    impl WindowImpl for PuzzlemoredaysWindow {}
    impl ApplicationWindowImpl for PuzzlemoredaysWindow {}
    impl AdwApplicationWindowImpl for PuzzlemoredaysWindow {}
}

glib::wrapper! {
    pub struct PuzzlemoredaysWindow(ObjectSubclass<imp::PuzzlemoredaysWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gtk::Buildable, gtk::Accessible, gtk::ConstraintTarget,
                  gtk::Native, gtk::Root, gtk::ShortcutManager, gio::ActionGroup, gio::ActionMap;
}

impl PuzzlemoredaysWindow {
    pub fn new<P: IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::builder()
            .property("application", application)
            .build()
    }

    pub fn navigation_view(&self) -> adw::NavigationView {
        self.imp().navigation_view.clone()
    }

    pub fn core_collection_list(&self) -> gtk::ListBox {
        self.imp().core_collection_list.clone()
    }

    pub fn community_collection_list(&self) -> gtk::ListBox {
        self.imp().community_collection_list.clone()
    }

    pub fn load_collection_button_row(&self) -> adw::ButtonRow {
        self.imp().load_collection_button_row.clone()
    }

    pub fn puzzle_list(&self) -> gtk::ListBox {
        self.imp().puzzle_list.clone()
    }

    pub fn grid(&self) -> gtk::Fixed {
        self.imp().grid.clone()
    }

    pub fn puzzle_info_button(&self) -> gtk::Button {
        self.imp().puzzle_info_button.clone()
    }

    pub fn target_selection_button(&self) -> gtk::Button {
        self.imp().target_selection_button.clone()
    }

    pub fn solver_state(&self) -> gtk::Button {
        self.imp().solver_state.clone()
    }
}
