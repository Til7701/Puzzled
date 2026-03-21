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
use crate::app::collection_selection::collection_selection_page::CollectionSelectionPage;
use crate::app::puzzle_area::puzzle_page::PuzzlePage;
use crate::app::puzzle_selection::puzzle_selection_page::PuzzleSelectionPage;
use adw::subclass::prelude::*;
use gtk::prelude::*;
use gtk::{gio, glib};

pub const MIN_WINDOW_WIDTH: i32 = 320;
pub const MIN_WINDOW_HEIGHT: i32 = 240;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/de/til7701/Puzzled/window.ui")]
    pub struct PuzzledWindow {
        #[template_child]
        pub outer_view: TemplateChild<adw::NavigationSplitView>,
        #[template_child]
        pub inner_view: TemplateChild<adw::NavigationSplitView>,
        #[template_child]
        pub collection_selection_nav_page: TemplateChild<CollectionSelectionPage>,
        #[template_child]
        pub puzzle_selection_nav_page: TemplateChild<PuzzleSelectionPage>,
        #[template_child]
        pub puzzle_area_nav_page: TemplateChild<PuzzlePage>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PuzzledWindow {
        const NAME: &'static str = "PuzzledWindow";
        type Type = super::PuzzledWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for PuzzledWindow {}
    impl WidgetImpl for PuzzledWindow {}
    impl WindowImpl for PuzzledWindow {}
    impl ApplicationWindowImpl for PuzzledWindow {}
    impl AdwApplicationWindowImpl for PuzzledWindow {}
}

glib::wrapper! {
    pub struct PuzzledWindow(ObjectSubclass<imp::PuzzledWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gtk::Buildable, gtk::Accessible, gtk::ConstraintTarget,
                  gtk::Native, gtk::Root, gtk::ShortcutManager, gio::ActionGroup, gio::ActionMap;
}

impl PuzzledWindow {
    pub fn new<P: IsA<gtk::Application>>(application: &P) -> Self {
        let obj: PuzzledWindow = glib::Object::builder()
            .property("application", application)
            .build();
        obj.imp().puzzle_area_nav_page.set_window(&obj);
        obj.imp().collection_selection_nav_page.set_window(&obj);
        obj.setup_nav_signals();
        obj
    }

    fn setup_nav_signals(&self) {
        self.imp()
            .collection_selection_nav_page
            .connect_collection_selected({
                let self_clone = self.clone();
                move |collection| {
                    self_clone
                        .imp()
                        .puzzle_selection_nav_page
                        .show_collection(collection);
                    self_clone.imp().outer_view.set_show_content(false);
                    self_clone.imp().inner_view.set_show_content(true);
                }
            });
        self.imp()
            .puzzle_selection_nav_page
            .connect_puzzle_selected({
                let self_clone = self.clone();
                move |puzzle| {
                    self_clone.imp().puzzle_area_nav_page.show_puzzle(puzzle);
                    self_clone.imp().outer_view.set_show_content(true);
                }
            })
    }

    pub fn puzzle_area_nav_page(&self) -> &PuzzlePage {
        &self.imp().puzzle_area_nav_page
    }

    pub fn outer_view(&self) -> &TemplateChild<adw::NavigationSplitView> {
        &self.imp().outer_view
    }
}
