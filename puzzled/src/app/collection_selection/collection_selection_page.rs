use adw::gio;
use adw::subclass::prelude::*;
use gtk::glib;

mod imp {
    use adw::subclass::prelude::*;
    use gtk::glib;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/de/til7701/Puzzled/ui/page/collection-selection-page.ui")]
    pub struct PuzzledCollectionSelectionPage {
        #[template_child]
        pub core_collection_list: TemplateChild<gtk::ListBox>,
        #[template_child]
        pub community_collection_list: TemplateChild<gtk::ListBox>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PuzzledCollectionSelectionPage {
        const NAME: &'static str = "PuzzledCollectionSelectionPage";
        type Type = super::CollectionSelectionPage;
        type ParentType = adw::NavigationPage;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.install_action("app.load_collection", None, move |page, _, _| {
                page.show_load_collection_dialog()
            });
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for PuzzledCollectionSelectionPage {}
    impl WidgetImpl for PuzzledCollectionSelectionPage {}
    impl NavigationPageImpl for PuzzledCollectionSelectionPage {}
}

glib::wrapper! {
    pub struct CollectionSelectionPage(ObjectSubclass<imp::PuzzledCollectionSelectionPage>)
        @extends gtk::Widget, adw::NavigationPage,
         @implements gtk::Buildable, gtk::Accessible, gtk::ConstraintTarget,
                  gtk::Native, gio::ActionGroup, gio::ActionMap;
}

impl CollectionSelectionPage {
    pub fn core_collection_list(&self) -> gtk::ListBox {
        self.imp().core_collection_list.clone()
    }

    pub fn community_collection_list(&self) -> gtk::ListBox {
        self.imp().community_collection_list.clone()
    }
}
