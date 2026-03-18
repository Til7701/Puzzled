use adw::gio;
use adw::glib;
use adw::subclass::prelude::*;
use gtk::prelude::*;

mod imp {
    use super::*;
    use adw::glib::Properties;
    use std::cell::RefCell;

    #[derive(Debug, Default, gtk::CompositeTemplate, Properties)]
    #[template(resource = "/de/til7701/Puzzled/ui/widget/info-pill.ui")]
    #[properties(wrapper_type = super::InfoPill)]
    pub struct PuzzledInfoPill {
        #[property(name = "label", get, set)]
        pub label_text: RefCell<String>,
        #[property(name = "icon-name", get, set)]
        pub icon_name: RefCell<Option<String>>,

        #[template_child]
        pub icon: TemplateChild<gtk::Image>,
        #[template_child]
        pub label: TemplateChild<gtk::Label>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PuzzledInfoPill {
        const NAME: &'static str = "PuzzledInfoPill";
        type Type = InfoPill;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for PuzzledInfoPill {}
    impl WidgetImpl for PuzzledInfoPill {}
    impl BoxImpl for PuzzledInfoPill {}
}

glib::wrapper! {
    pub struct InfoPill(ObjectSubclass<imp::PuzzledInfoPill>)
        @extends gtk::Widget, gtk::Box,
         @implements gtk::Buildable, gtk::Accessible, gtk::ConstraintTarget,
                  gtk::Native, gio::ActionGroup, gio::ActionMap;
}

impl InfoPill {
    pub fn new<P: IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::builder()
            .property("application", application)
            .build()
    }

    pub fn highlight(&self, highlight: bool) {
        if highlight {
            self.imp().icon.add_css_class("accent");
        } else {
            self.imp().icon.remove_css_class("accent");
        }
    }
}
