use crate::model::stars::Stars;
use adw::gio;
use adw::glib;
use adw::subclass::prelude::*;
use gtk::prelude::*;
use gtk::Widget;

mod imp {
    use super::*;
    use crate::app::components::stars::StarsView;
    use adw::glib::Properties;

    #[derive(Debug, Default, gtk::CompositeTemplate, Properties)]
    #[template(resource = "/de/til7701/Puzzled/ui/widget/puzzle-mod.ui")]
    #[properties(wrapper_type = super::PuzzleMod)]
    pub struct PuzzledPuzzleMod {
        #[template_child]
        pub icon: TemplateChild<gtk::Image>,
        #[template_child]
        pub label: TemplateChild<gtk::Label>,
        #[template_child]
        pub stars: TemplateChild<StarsView>,
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
    /// Set the state to display.
    pub fn set_state(&self, state: &PuzzleModState) {
        match state {
            PuzzleModState::Stars(stars) => self.set_stars(stars),
            PuzzleModState::Locked => self.set_locked(),
            PuzzleModState::Unsolvable => self.set_unsolvable(),
        }
    }

    fn set_stars(&self, stars: &Stars) {
        let imp = self.imp();
        imp.icon.set_visible(false);
        imp.label.set_visible(false);
        self.set_tooltip_text(None);
        imp.stars.set_visible(true);
        imp.stars.set_stars(stars);
    }

    fn set_locked(&self) {
        let imp = self.imp();
        imp.icon.set_icon_name(Some("padlock2-symbolic"));
        imp.icon.set_visible(true);
        imp.label.set_text("Locked");
        self.set_tooltip_text(Some("Solve the previous puzzles to unlock."));
        imp.label.set_visible(true);
        imp.stars.set_visible(false);
    }

    fn set_unsolvable(&self) {
        let imp = self.imp();
        imp.icon
            .set_icon_name(Some("cross-large-circle-outline-symbolic"));
        imp.icon.set_visible(true);
        imp.label.set_text("Unsolvable");
        self.set_tooltip_text(Some("This puzzle cannot be solved"));
        imp.label.set_visible(true);
        imp.stars.set_visible(false);
    }
}

/// Used to set what the [PuzzleMod] should display.
#[derive(Debug, PartialEq, Eq)]
pub enum PuzzleModState {
    /// Displays only the stars using [StarsView].
    Stars(Stars),
    /// Displays a lock and shows the text `Locked`.
    Locked,
    /// Displays a cross and the test `Unsolvable`.
    Unsolvable,
}
