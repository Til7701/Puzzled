use adw::gio;
use adw::subclass::prelude::*;
use gtk::glib;
use gtk::prelude::*;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/de/til7701/Puzzled/ui/page/collection-selection-page.ui")]
    pub struct CollectionSelectionPage {
        #[template_child]
        pub core_collection_list: TemplateChild<gtk::ListBox>,
        #[template_child]
        pub community_collection_list: TemplateChild<gtk::ListBox>,
        #[template_child]
        pub load_collection_button_row: TemplateChild<adw::ButtonRow>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for CollectionSelectionPage {
        const NAME: &'static str = "CollectionSelectionPage";
        type Type = super::CollectionSelectionPage;
        type ParentType = adw::NavigationPage;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for CollectionSelectionPage {}
    impl WidgetImpl for CollectionSelectionPage {}
    impl NavigationPageImpl for CollectionSelectionPage {}
}

glib::wrapper! {
    pub struct CollectionSelectionPage(ObjectSubclass<imp::CollectionSelectionPage>)
        @extends gtk::Widget, adw::NavigationPage,
         @implements gtk::Buildable, gtk::Accessible, gtk::ConstraintTarget,
                  gtk::Native, gio::ActionGroup, gio::ActionMap;
}

impl CollectionSelectionPage {
    pub fn new<P: IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::builder()
            .property("application", application)
            .build()
    }

    pub fn core_collection_list(&self) -> gtk::ListBox {
        self.imp().core_collection_list.clone()
    }

    pub fn community_collection_list(&self) -> gtk::ListBox {
        self.imp().community_collection_list.clone()
    }

    pub fn load_collection_button_row(&self) -> adw::ButtonRow {
        self.imp().load_collection_button_row.clone()
    }
}
