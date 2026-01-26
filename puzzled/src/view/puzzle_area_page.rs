use adw::gio;
use adw::subclass::prelude::*;
use gtk::glib;
use gtk::prelude::*;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/de/til7701/Puzzled/ui/page/puzzle-area-page.ui")]
    pub struct PuzzleAreaPage {
        #[template_child]
        pub grid: TemplateChild<gtk::Fixed>,
        #[template_child]
        pub puzzle_info_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub extension_separator: TemplateChild<gtk::Separator>,
        #[template_child]
        pub target_selection_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub solver_state: TemplateChild<gtk::Button>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PuzzleAreaPage {
        const NAME: &'static str = "PuzzleAreaPage";
        type Type = super::PuzzleAreaPage;
        type ParentType = adw::NavigationPage;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for PuzzleAreaPage {}
    impl WidgetImpl for PuzzleAreaPage {}
    impl NavigationPageImpl for PuzzleAreaPage {}
}

glib::wrapper! {
    pub struct PuzzleAreaPage(ObjectSubclass<imp::PuzzleAreaPage>)
        @extends gtk::Widget, adw::NavigationPage,
         @implements gtk::Buildable, gtk::Accessible, gtk::ConstraintTarget,
                  gtk::Native, gio::ActionGroup, gio::ActionMap;
}

impl PuzzleAreaPage {
    pub fn new<P: IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::builder()
            .property("application", application)
            .build()
    }

    pub fn grid(&self) -> gtk::Fixed {
        self.imp().grid.clone()
    }

    pub fn puzzle_info_button(&self) -> gtk::Button {
        self.imp().puzzle_info_button.clone()
    }

    pub fn extension_separator(&self) -> gtk::Separator {
        self.imp().extension_separator.clone()
    }

    pub fn target_selection_button(&self) -> gtk::Button {
        self.imp().target_selection_button.clone()
    }

    pub fn solver_state(&self) -> gtk::Button {
        self.imp().solver_state.clone()
    }
}
