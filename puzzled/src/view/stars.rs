use crate::puzzles::stars::Stars;
use adw::gio;
use adw::glib;
use adw::subclass::prelude::*;
use gtk::prelude::*;
use gtk::{Image, Widget};

mod imp {
    use super::*;
    use std::cell::RefCell;

    #[derive(Debug, Default)]
    pub struct PuzzledStarsView {
        pub star_widgets: RefCell<Vec<Widget>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PuzzledStarsView {
        const NAME: &'static str = "PuzzledStarsView";
        type Type = StarsView;
        type ParentType = gtk::Box;

        fn class_init(_: &mut Self::Class) {}

        fn instance_init(_: &glib::subclass::InitializingObject<Self>) {}
    }

    impl ObjectImpl for PuzzledStarsView {}
    impl WidgetImpl for PuzzledStarsView {}
    impl BoxImpl for PuzzledStarsView {}
}

glib::wrapper! {
    pub struct StarsView(ObjectSubclass<imp::PuzzledStarsView>)
        @extends Widget, gtk::Box,
         @implements gtk::Buildable, gtk::Accessible, gtk::ConstraintTarget,
                  gtk::Native, gio::ActionGroup, gio::ActionMap;
}

impl StarsView {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    pub fn set_stars(&self, stars: &Stars) {
        self.clear_stars();
        let imp = self.imp();
        for star_widget in Self::construct_stars(stars) {
            self.append(&star_widget);
            imp.star_widgets.borrow_mut().push(star_widget);
        }
        let tooltip: Option<String> = stars.message();
        self.set_tooltip_text(tooltip.as_deref());
    }

    fn construct_stars(stars: &Stars) -> Vec<Widget> {
        let mut widgets = Vec::new();
        const STAR_ICON_SIZE: i32 = 12;
        let reached_css_classes = if stars.reached() == stars.total() {
            vec!["accent"]
        } else {
            vec![]
        };
        for _ in 0..stars.reached() {
            let star_icon = Image::builder()
                .icon_name("star-filled-rounded-symbolic")
                .css_classes(reached_css_classes.clone())
                .build();
            star_icon.set_pixel_size(STAR_ICON_SIZE);
            widgets.push(star_icon.upcast());
        }
        for _ in stars.reached()..stars.total() {
            let star_icon = Image::builder()
                .icon_name("star-outline-rounded-symbolic")
                .build();
            star_icon.set_pixel_size(STAR_ICON_SIZE);
            widgets.push(star_icon.upcast());
        }

        widgets
    }

    fn clear_stars(&self) {
        let imp = self.imp();
        for star_widget in imp.star_widgets.borrow_mut().drain(..) {
            self.remove(&star_widget);
        }
    }
}
