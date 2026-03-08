use crate::puzzles::stars::Stars;
use adw::gio;
use adw::glib;
use adw::subclass::prelude::*;
use gtk::prelude::*;
use gtk::{IconSize, Image, Widget};

mod imp {
    use super::*;
    use adw::glib::Properties;
    use gtk::IconSize;
    use std::cell::{Cell, RefCell};

    #[derive(Debug, gtk::CompositeTemplate, Properties)]
    #[template(resource = "/de/til7701/Puzzled/ui/widget/stars-view.ui")]
    #[properties(wrapper_type = super::StarsView)]
    pub struct PuzzledStarsView {
        #[property(name = "icon-size", get, set, builder(IconSize::Inherit))]
        pub icon_size: Cell<IconSize>,
        pub star_widgets: RefCell<Vec<Widget>>,
    }

    impl Default for PuzzledStarsView {
        fn default() -> Self {
            Self {
                icon_size: Cell::new(IconSize::Inherit),
                star_widgets: RefCell::new(Vec::new()),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PuzzledStarsView {
        const NAME: &'static str = "PuzzledStarsView";
        type Type = StarsView;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
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
            self.bind_property("icon-size", &star_widget, "icon-size")
                .sync_create()
                .build();
            imp.star_widgets.borrow_mut().push(star_widget.upcast());
        }
        let tooltip: Option<String> = stars.message();
        self.set_tooltip_text(tooltip.as_deref());
    }

    fn construct_stars(stars: &Stars) -> Vec<Image> {
        let mut widgets = Vec::new();
        let reached_css_classes = if stars.reached() == stars.total() {
            vec!["accent"]
        } else {
            vec![]
        };
        for _ in 0..stars.reached() {
            let star_icon = Image::builder()
                .icon_name("star-large-symbolic")
                .css_classes(reached_css_classes.clone())
                .build();
            widgets.push(star_icon);
        }
        for _ in stars.reached()..stars.total() {
            let star_icon = Image::builder()
                .icon_name("star-outline-rounded-symbolic")
                .build();
            widgets.push(star_icon);
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
