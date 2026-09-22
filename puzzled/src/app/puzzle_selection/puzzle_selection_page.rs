use crate::app::puzzle_selection::puzzle_selection_item::PuzzleSelectionItem;
use crate::model::collection::CollectionModel;
use crate::model::puzzle::PuzzleModel;
use adw::gio;
use adw::prelude::NavigationPageExt;
use adw::subclass::prelude::*;
use gtk::glib;
use gtk::prelude::*;
use log::debug;
use puzzle_config::ProgressionConfig;

const PUZZLE_SELECTED_SIGNAL_NAME: &str = "puzzle-selected";

mod imp {
    use super::*;
    use crate::app::components::info_pill::InfoPill;
    use crate::model::puzzle::PuzzleModel;
    use adw::gio::ListStore;
    use adw::glib::subclass::Signal;
    use gtk::{ListItem, NoSelection, SignalListItemFactory};
    use std::cell::RefCell;
    use std::sync::OnceLock;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/de/til7701/Puzzled/ui/page/puzzle-selection-page.ui")]
    pub struct PuzzleSelectionPage {
        #[template_child]
        pub collection_description_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub collection_info_box: TemplateChild<adw::WrapBox>,
        #[template_child]
        pub puzzle_count_pill: TemplateChild<InfoPill>,
        #[template_child]
        pub author_pill: TemplateChild<InfoPill>,
        #[template_child]
        pub version_pill: TemplateChild<InfoPill>,
        #[template_child]
        pub puzzle_list: TemplateChild<gtk::ListView>,
        pub list_store: RefCell<Option<ListStore>>,

        pub collection: RefCell<Option<CollectionModel>>,
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

    impl ObjectImpl for PuzzleSelectionPage {
        fn signals() -> &'static [Signal] {
            static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
            SIGNALS.get_or_init(|| {
                vec![
                    Signal::builder(PUZZLE_SELECTED_SIGNAL_NAME)
                        .param_types([PuzzleModel::static_type()])
                        .build(),
                ]
            })
        }

        fn constructed(&self) {
            let factory = SignalListItemFactory::new();
            factory.connect_setup(|_, list_item| {
                let list_item = list_item.downcast_ref::<ListItem>();
                if let Some(list_item) = list_item {
                    let puzzle_list_item = PuzzleSelectionItem::new();
                    list_item.set_child(Some(&puzzle_list_item));
                    puzzle_list_item.connect_locked_notify({
                        let list_item = list_item.clone();
                        move |item| {
                            let locked = item.locked();
                            list_item.set_activatable(!locked);
                            // if item.locked() {
                            //     list_item.add_css_class("dimmed");
                            // } else {
                            //     list_item.remove_css_class("dimmed");
                            // }
                        }
                    });
                }
            });
            factory.connect_bind(|_, list_item| {
                let list_item = list_item.downcast_ref::<ListItem>();
                if let Some(list_item) = list_item {
                    let item = list_item
                        .item()
                        .and_then(|c| c.downcast::<PuzzleModel>().ok());
                    let child = list_item
                        .child()
                        .and_then(|c| c.downcast::<PuzzleSelectionItem>().ok());
                    if let Some(item) = item
                        && let Some(puzzle_list_item) = child
                    {
                        puzzle_list_item.update(&item);
                    }
                }
            });
            self.puzzle_list.set_factory(Some(&factory));

            let list_store = ListStore::new::<PuzzleModel>();
            self.list_store.replace(Some(list_store.clone()));
            let selection_model = NoSelection::new(Some(list_store));
            self.puzzle_list.set_model(Some(&selection_model));
            selection_model.connect_selection_changed({
                let self_clone = self.obj().clone();
                move |model, i, _| {
                    let selection = model.item(i).and_then(|s| s.downcast::<PuzzleModel>().ok());
                    if let Some(puzzle) = selection {
                        self_clone.emit_puzzle_selected(&puzzle);
                    }
                }
            });
        }
    }
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
    /// Connects to the `puzzle_selected` signal.
    /// This signal is emitted, when the user selects a puzzle.
    /// The puzzle area should be shown when this happens.
    pub fn connect_puzzle_selected<F: Fn(&PuzzleModel) + 'static>(&self, callback: F) {
        self.connect_local(PUZZLE_SELECTED_SIGNAL_NAME, false, move |values| {
            let model = values[1]
                .get::<PuzzleModel>()
                .expect("Failed to get RandomPuzzlePage from signal");
            callback(&model);
            None
        });
    }

    fn emit_puzzle_selected(&self, puzzle: &PuzzleModel) {
        debug!(
            "Emitting puzzle-selected signal for puzzle: {}",
            puzzle.config().id()
        );
        self.emit_by_name::<()>(PUZZLE_SELECTED_SIGNAL_NAME, &[puzzle]);
    }

    /// Shows the given collection in the view. This means, it shows all puzzles of the collection
    /// and displays the collection information and the puzzles which the user can select.
    /// If that is the case, the `puzzle_selected` signal is emitted.
    ///
    /// # Arguments
    ///
    /// * `collection`: the collection to show
    ///
    /// returns: ()
    pub fn show_collection(&self, collection: &CollectionModel) {
        self.imp().collection.replace(Some(collection.clone()));

        self.set_title(collection.config().name());
        if let Some(description) = collection.config().description() {
            self.imp()
                .collection_description_label
                .set_label(description);
            self.imp().collection_description_label.set_visible(true);
        } else {
            self.imp().collection_description_label.set_visible(false);
        }

        let puzzle_count = collection.puzzles().len();
        self.imp()
            .puzzle_count_pill
            .set_label(format!("{}", puzzle_count));
        self.imp()
            .author_pill
            .set_label(collection.config().author().to_string());
        if let Some(version) = collection.config().version() {
            self.imp().version_pill.set_label(version.to_string());
            if self.imp().version_pill.parent().is_none() {
                self.imp()
                    .collection_info_box
                    .append(&self.imp().version_pill.get());
            }
        } else if self.imp().version_pill.parent().is_some() {
            self.imp()
                .collection_info_box
                .remove(&self.imp().version_pill.get());
        }

        if let Some(store) = self.imp().list_store.borrow().as_ref() {
            store.remove_all();
            for puzzle in collection.puzzles().iter() {
                store.append(puzzle);
            }
        };

        match collection.config().progression() {
            ProgressionConfig::Any => {
                self.imp().puzzle_list.add_css_class("boxed-list-separate");
                self.imp().puzzle_list.remove_css_class("boxed-list");
            }
            ProgressionConfig::Sequential => {
                self.imp().puzzle_list.add_css_class("boxed-list");
                self.imp()
                    .puzzle_list
                    .remove_css_class("boxed-list-separate");
            }
        }
    }
}
