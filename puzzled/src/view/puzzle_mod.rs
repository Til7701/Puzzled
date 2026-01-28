use adw::gio;
use adw::glib;
use adw::subclass::prelude::*;
use gtk::prelude::*;

mod imp {
    use super::*;
    use adw::glib::Properties;

    #[derive(Debug, Default, gtk::CompositeTemplate, Properties)]
    #[template(resource = "/de/til7701/Puzzled/ui/widget/puzzle-mod.ui")]
    #[properties(wrapper_type = super::PuzzleMod)]
    pub struct PuzzledPuzzleMod {
        #[template_child]
        pub icon: TemplateChild<gtk::Image>,
        #[template_child]
        pub label: TemplateChild<gtk::Label>,
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
        @extends gtk::Widget, gtk::Box,
         @implements gtk::Buildable, gtk::Accessible, gtk::ConstraintTarget,
                  gtk::Native, gio::ActionGroup, gio::ActionMap;
}

impl PuzzleMod {
    pub fn new<P: IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::builder()
            .property("application", application)
            .build()
    }

    pub fn set_off(&self) {
        let imp = self.imp();
        imp.icon.set_visible(false);
        imp.label.set_visible(false);
    }

    pub fn set_solved(&self) {
        let imp = self.imp();
        imp.icon
            .set_icon_name(Some("check-round-outline2-symbolic"));
        imp.icon.set_visible(true);
        imp.label.set_text("Solved");
        imp.label.set_visible(true);
    }
}
