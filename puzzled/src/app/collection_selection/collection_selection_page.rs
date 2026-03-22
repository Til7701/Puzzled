use crate::app::collection_selection::collection_selection_item::CollectionSelectionItem;
use crate::model::collection::CollectionModel;
use crate::model::store::with_puzzle_collection_store;
use crate::window::PuzzledWindow;
use adw::gio;
use adw::prelude::{Cast, ObjectExt};
use adw::subclass::prelude::*;
use gtk::prelude::WidgetExt;
use gtk::{glib, ListBoxRow};
use log::debug;

const COLLECTION_SELECTED_SIGNAL_NAME: &str = "collection-selected";

mod imp {
    use super::COLLECTION_SELECTED_SIGNAL_NAME;
    use crate::model::collection::CollectionModel;
    use crate::window::PuzzledWindow;
    use adw::glib::subclass::Signal;
    use adw::prelude::StaticType;
    use adw::subclass::prelude::*;
    use gtk::glib;
    use std::cell::OnceCell;
    use std::sync::OnceLock;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/de/til7701/Puzzled/ui/page/collection-selection-page.ui")]
    pub struct PuzzledCollectionSelectionPage {
        #[template_child]
        pub core_collection_list: TemplateChild<gtk::ListBox>,
        #[template_child]
        pub community_collection_list: TemplateChild<gtk::ListBox>,

        pub window: OnceCell<PuzzledWindow>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PuzzledCollectionSelectionPage {
        const NAME: &'static str = "PuzzledCollectionSelectionPage";
        type Type = super::CollectionSelectionPage;
        type ParentType = adw::NavigationPage;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.install_action("app.load_collection", None, |page, _, _| {
                page.show_load_collection_dialog()
            });
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for PuzzledCollectionSelectionPage {
        fn signals() -> &'static [Signal] {
            static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
            SIGNALS.get_or_init(|| {
                vec![
                    Signal::builder(COLLECTION_SELECTED_SIGNAL_NAME)
                        .param_types([CollectionModel::static_type()])
                        .build(),
                ]
            })
        }

        fn constructed(&self) {
            self.parent_constructed();
            self.obj().setup();
        }
    }
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
    /// Must be called to set the main application window this page is shown in.
    /// It is used to set it as the parent of dialogs.
    pub fn set_window(&self, window: &PuzzledWindow) {
        self.imp()
            .window
            .set(window.clone())
            .expect("Failed to set window for CollectionSelectionPage");
    }

    /// Must be called once on initialization to load the collections and connect to changes
    /// in the selection.
    pub(super) fn setup(&self) {
        self.load_core_collections();
        self.load_community_collections();

        self.imp().core_collection_list.connect_row_selected({
            let self_clone = self.clone();
            move |_, row| {
                if let Some(row) = row {
                    self_clone.imp().community_collection_list.unselect_all();
                    let row = row.clone().downcast::<CollectionSelectionItem>().unwrap();
                    let collection = row.collection();
                    self_clone.emit_collection_selected(collection);
                }
            }
        });
        self.imp().community_collection_list.connect_row_selected({
            let self_clone = self.clone();
            move |_, row| {
                if let Some(row) = row {
                    self_clone.imp().core_collection_list.unselect_all();
                    let row = row.clone().downcast::<CollectionSelectionItem>().unwrap();
                    let collection = row.collection();
                    self_clone.emit_collection_selected(collection);
                }
            }
        });

        self.imp()
            .core_collection_list
            .select_row(self.imp().core_collection_list.row_at_index(0).as_ref());
    }

    fn load_core_collections(&self) {
        self.imp().core_collection_list.remove_all();

        with_puzzle_collection_store(|store| {
            for collection in store.core_puzzle_collections().iter() {
                let row = CollectionSelectionItem::new(collection, true);
                self.imp().core_collection_list.append(&row);
            }
        });
    }

    fn load_community_collections(&self) {
        self.imp().community_collection_list.remove_all();

        with_puzzle_collection_store(|store| {
            for collection in store.community_puzzle_collections().iter() {
                self.add_community_collection(collection);
            }
        });
    }

    /// Add a community collection.
    ///
    /// The collection is not selected automatically.
    /// You should use [Self::select_last_community_collection()] after this call, if
    /// necessary.
    pub(super) fn add_community_collection(&self, collection: &CollectionModel) {
        let row = CollectionSelectionItem::new(collection, false);
        self.imp().community_collection_list.append(&row);
        collection.connect_deleted({
            let self_clone = self.clone();
            let row = row.clone();
            move || {
                self_clone.imp().community_collection_list.remove(&row);
                self_clone.select_community_or_core_collection();
            }
        });
    }

    /// The `collection_selected` signal is emitted, if the user selects a collection to see
    /// the puzzles. The [PuzzleSelectionPage] should be shown.
    pub fn connect_collection_selected<F: Fn(&CollectionModel) + 'static>(&self, callback: F) {
        self.connect_local(COLLECTION_SELECTED_SIGNAL_NAME, false, move |values| {
            let model = values[1]
                .get::<CollectionModel>()
                .expect("Failed to get RandomPuzzlePage from signal");
            callback(&model);
            None
        });
    }

    fn emit_collection_selected(&self, collection: &CollectionModel) {
        debug!(
            "Emitting collection selected signal for collection: {}",
            collection.config().id()
        );
        self.emit_by_name::<()>(COLLECTION_SELECTED_SIGNAL_NAME, &[collection]);
    }

    /// Selects the last community collection in the list.
    ///
    /// The callee has to be sure that there is at least one community collection, otherwise this
    /// will panic.
    pub(super) fn select_last_community_collection(&self) {
        let last_index =
            with_puzzle_collection_store(|store| store.community_puzzle_collections().len()) - 1;
        self.imp()
            .community_collection_list
            .row_at_index(last_index as i32)
            .unwrap()
            .activate();
    }

    /// Unselects all collections. Used when a special view should be opened like the random puzzle
    /// generator.
    #[allow(dead_code)]
    fn select_none(&self) {
        self.imp()
            .core_collection_list
            .select_row(None::<&ListBoxRow>);
        self.imp()
            .community_collection_list
            .select_row(None::<&ListBoxRow>);
    }

    /// Selects the last community collection if there are any, otherwise selects the first core collection.
    fn select_community_or_core_collection(&self) {
        let community_count =
            with_puzzle_collection_store(|store| store.community_puzzle_collections().len());
        if community_count > 0 {
            self.select_last_community_collection();
        } else {
            self.imp()
                .core_collection_list
                .row_at_index(0)
                .unwrap()
                .activate();
        }
    }
}
