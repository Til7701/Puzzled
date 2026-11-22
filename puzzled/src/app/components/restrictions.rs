use adw::gio;
use adw::glib;
use adw::subclass::prelude::*;
use gtk::prelude::WidgetExt;
use puzzled_config::RestrictionsConfig;

mod imp {
    use super::*;
    use adw::glib::Properties;

    #[derive(Debug, Default, gtk::CompositeTemplate, Properties)]
    #[template(resource = "/de/til7701/Puzzled/ui/widget/restrictions.ui")]
    #[properties(wrapper_type = super::RestrictionsView)]
    pub struct PuzzledRestrictionsView {
        #[template_child]
        pub rotate: TemplateChild<gtk::MenuButton>,
        #[template_child]
        pub rotate_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub flip: TemplateChild<gtk::MenuButton>,
        #[template_child]
        pub flip_label: TemplateChild<gtk::Label>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PuzzledRestrictionsView {
        const NAME: &'static str = "PuzzledRestrictionsView";
        type Type = RestrictionsView;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for PuzzledRestrictionsView {}
    impl WidgetImpl for PuzzledRestrictionsView {}
    impl BoxImpl for PuzzledRestrictionsView {}
}

glib::wrapper! {
    pub struct RestrictionsView(ObjectSubclass<imp::PuzzledRestrictionsView>)
        @extends gtk::Widget, gtk::Box,
         @implements gtk::Buildable, gtk::Accessible, gtk::ConstraintTarget,
                  gtk::Native, gio::ActionGroup, gio::ActionMap;
}

impl RestrictionsView {
    pub fn set(&self, restrictions: &RestrictionsConfig) {
        let rotate = &self.imp().rotate;
        if restrictions.allow_rotation() {
            rotate.add_css_class("success");
            rotate.remove_css_class("error");
            self.imp()
                .rotate_label
                .set_label("Rotating tiles is allowed.");
        } else {
            rotate.remove_css_class("success");
            rotate.add_css_class("error");
            self.imp()
                .rotate_label
                .set_label("Rotating tiles not allowed!");
        }

        let flip = &self.imp().flip;
        if restrictions.allow_flip() {
            flip.add_css_class("success");
            flip.remove_css_class("error");
            self.imp()
                .flip_label
                .set_label("Flipping tiles is allowed.");
        } else {
            flip.remove_css_class("success");
            flip.add_css_class("error");
            self.imp()
                .flip_label
                .set_label("Flipping tiles not allowed!");
        }
    }
}
