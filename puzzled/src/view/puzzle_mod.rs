use crate::puzzles::stars::Stars;
use adw::gio;
use adw::glib;
use adw::subclass::prelude::*;
use gtk::prelude::*;
use gtk::Widget;

mod imp {
    use super::*;
    use adw::glib::Properties;
    use std::cell::RefCell;

    #[derive(Debug, Default, gtk::CompositeTemplate, Properties)]
    #[template(resource = "/de/til7701/Puzzled/ui/widget/puzzle-mod.ui")]
    #[properties(wrapper_type = super::PuzzleMod)]
    pub struct PuzzledPuzzleMod {
        #[template_child]
        pub icon: TemplateChild<gtk::Image>,
        #[template_child]
        pub label: TemplateChild<gtk::Label>,
        pub star_widgets: RefCell<Vec<Widget>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PuzzledPuzzleMod {
        const NAME: &'static str = "PuzzledPuzzleMod";
        type Type = PuzzleMod;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for PuzzledPuzzleMod {}
    impl WidgetImpl for PuzzledPuzzleMod {}
    impl BoxImpl for PuzzledPuzzleMod {}
}

glib::wrapper! {
    pub struct PuzzleMod(ObjectSubclass<imp::PuzzledPuzzleMod>)
        @extends Widget, gtk::Box,
         @implements gtk::Buildable, gtk::Accessible, gtk::ConstraintTarget,
                  gtk::Native, gio::ActionGroup, gio::ActionMap;
}

impl PuzzleMod {
    pub fn new<P: IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::builder()
            .property("application", application)
            .build()
    }

    fn set_stars(&self, stars: &Stars) {
        let imp = self.imp();
        imp.icon.set_visible(false);
        imp.label.set_visible(false);
        for star_widget in Self::construct_stars(stars) {
            self.append(&star_widget);
            imp.star_widgets.borrow_mut().push(star_widget);
        }
        let tooltip: Option<String> = stars.max_hints_for_next_star().map(|max| {
            if max == 0 {
                "Get the next star by solving the puzzle".to_string()
            } else {
                format!("Use at most {} hints to get the next star", max)
            }
        });
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
            let star_icon = gtk::Image::builder()
                .icon_name("star-filled-rounded-symbolic")
                .css_classes(reached_css_classes.clone())
                .build();
            star_icon.set_pixel_size(STAR_ICON_SIZE);
            widgets.push(star_icon.upcast());
        }
        for _ in stars.reached()..stars.total() {
            let star_icon = gtk::Image::builder()
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

    fn set_locked(&self) {
        self.clear_stars();
        let imp = self.imp();
        imp.icon.set_icon_name(Some("padlock2-symbolic"));
        imp.icon.set_visible(true);
        imp.label.set_text("Locked");
        self.set_tooltip_text(Some("Solve the previous puzzles to unlock."));
        imp.label.set_visible(true);
    }

    fn set_unsolvable(&self) {
        self.clear_stars();
        let imp = self.imp();
        imp.icon
            .set_icon_name(Some("cross-large-circle-outline-symbolic"));
        imp.icon.set_visible(true);
        imp.label.set_text("Unsolvable");
        self.set_tooltip_text(Some("This puzzle cannot be solved"));
        imp.label.set_visible(true);
    }

    pub fn set_state(&self, state: &PuzzleModState) {
        match state {
            PuzzleModState::Stars(stars) => self.set_stars(stars),
            PuzzleModState::Locked => self.set_locked(),
            PuzzleModState::Unsolvable => self.set_unsolvable(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum PuzzleModState {
    Stars(Stars),
    Locked,
    Unsolvable,
}
