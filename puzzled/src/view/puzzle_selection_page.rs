use adw::gio;
use adw::subclass::prelude::*;
use gtk::glib;
use gtk::prelude::*;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/de/til7701/Puzzled/ui/page/puzzle-selection-page.ui")]
    pub struct PuzzleSelectionPage {
        #[template_child]
        pub puzzle_name_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub puzzle_description_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub collection_info_box: TemplateChild<adw::WrapBox>,
        #[template_child]
        pub puzzle_count_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub author_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub version_box: TemplateChild<gtk::Box>,
        #[template_child]
        pub version_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub puzzle_list: TemplateChild<gtk::ListBox>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PuzzleSelectionPage {
        const NAME: &'static str = "PuzzleSelectionPage";
        type Type = super::PuzzleSelectionPage;
        type ParentType = adw::NavigationPage;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for PuzzleSelectionPage {}
    impl WidgetImpl for PuzzleSelectionPage {}
    impl NavigationPageImpl for PuzzleSelectionPage {}
}

glib::wrapper! {
    pub struct PuzzleSelectionPage(ObjectSubclass<imp::PuzzleSelectionPage>)
        @extends gtk::Widget, adw::NavigationPage,
         @implements gtk::Buildable, gtk::Accessible, gtk::ConstraintTarget,
                  gtk::Native, gio::ActionGroup, gio::ActionMap;
}

impl PuzzleSelectionPage {
    pub fn new<P: IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::builder()
            .property("application", application)
            .build()
    }

    pub fn puzzle_name_label(&self) -> gtk::Label {
        self.imp().puzzle_name_label.clone()
    }

    pub fn puzzle_description_label(&self) -> gtk::Label {
        self.imp().puzzle_description_label.clone()
    }

    pub fn collection_info_box(&self) -> adw::WrapBox {
        self.imp().collection_info_box.clone()
    }

    pub fn puzzle_count_label(&self) -> gtk::Label {
        self.imp().puzzle_count_label.clone()
    }

    pub fn author_label(&self) -> gtk::Label {
        self.imp().author_label.clone()
    }

    pub fn version_box(&self) -> gtk::Box {
        self.imp().version_box.clone()
    }

    pub fn version_label(&self) -> gtk::Label {
        self.imp().version_label.clone()
    }

    pub fn puzzle_list(&self) -> gtk::ListBox {
        self.imp().puzzle_list.clone()
    }
}
