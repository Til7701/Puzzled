use crate::app::collection_selection::collection_selection_item::CollectionSelectionItem;
use crate::app::window::main::MainPresenter;
use crate::application::PuzzledApplication;
use crate::config;
use crate::global::state::get_state_mut;
use crate::model::collection::CollectionModel;
use crate::model::extension::PuzzleTypeExtension;
use crate::model::puzzle_meta::PuzzleMeta;
use crate::model::stars;
use crate::model::store::with_puzzle_collection_store;
use crate::window::PuzzledWindow;
use adw::gio::{Cancellable, File};
use adw::glib::{Variant, VariantTy};
use adw::prelude::{ActionMapExtManual, AdwDialogExt, AlertDialogExt, Cast, FileExtManual};
use adw::{gio, AlertDialog, ResponseAppearance};
use gtk::prelude::{ListBoxRowExt, WidgetExt};
use gtk::{FileFilter, ListBox, ListBoxRow};
use log::{debug, error};
use puzzle_config::ReadError::FileReadError;
use puzzle_config::{PuzzleConfigCollection, ReadError};

#[derive(Clone)]
pub struct CollectionSelectionPresenter {
    window: PuzzledWindow,
    main_presenter: MainPresenter,
    core_collection_list: ListBox,
    community_collection_list: ListBox,
}

impl CollectionSelectionPresenter {
    pub fn new(window: &PuzzledWindow, main_presenter: MainPresenter) -> Self {
        let page = window.collection_selection_nav_page();
        CollectionSelectionPresenter {
            window: window.clone(),
            main_presenter,
            core_collection_list: page.core_collection_list(),
            community_collection_list: page.community_collection_list(),
        }
    }

    pub fn register_actions(&self, app: &PuzzledApplication) {
        let delete_community_collection_action =
            gio::ActionEntry::builder("delete_community_collection")
                .parameter_type(Some(VariantTy::STRING))
                .activate({
                    let self_clone = self.clone();
                    move |_, _, v: Option<&Variant>| {
                        if let Some(v) = v {
                            let collection_id = v
                                .get::<String>()
                                .expect("Expected a string parameter for delete_collection action");
                            debug!("Delete collection with ID: {}", collection_id);
                            with_puzzle_collection_store(|store| {
                                store.remove_community_collection(&collection_id);
                            });
                            self_clone.update_community_collections();
                            self_clone.select_community_or_core_collection();
                        }
                    }
                })
                .build();
        app.add_action_entries([delete_community_collection_action]);
    }

    pub fn setup(&self) {
        self.load_core_collections();
        self.update_community_collections();

        self.core_collection_list.connect_row_selected({
            let self_clone = self.clone();
            move |_, row| {
                if let Some(row) = row {
                    self_clone.community_collection_list.unselect_all();
                    let collection_id = CollectionId::Core(row.index() as usize);
                    debug!("Selected core collection with index: {}", row.index());
                    self_clone.activate_collection(collection_id);
                }
            }
        });
        self.community_collection_list.connect_row_selected({
            let self_clone = self.clone();
            move |_, row| {
                if let Some(row) = row {
                    self_clone.core_collection_list.unselect_all();
                    let collection_id = CollectionId::Community(row.index() as usize);
                    debug!("Selected core collection with index: {}", row.index());
                    self_clone.activate_collection(collection_id);
                }
            }
        });

        self.core_collection_list
            .select_row(self.core_collection_list.row_at_index(0).as_ref());
    }

    fn current_collection_id(&self) -> Option<CollectionId> {
        if let Some(row) = self.core_collection_list.selected_row() {
            return Some(CollectionId::Core(row.index() as usize));
        }
        if let Some(row) = self.community_collection_list.selected_row() {
            return Some(CollectionId::Community(row.index() as usize));
        }
        None
    }

    pub fn on_solved(&self) {
        let current_collection_id = self.current_collection_id();
        if let Some(collection_id) = current_collection_id {
            match collection_id {
                CollectionId::Core(index) => {
                    let row = self.core_collection_list.row_at_index(index as i32);
                    if let Some(row) = row {
                        let collection_item: CollectionSelectionItem = row.downcast().unwrap();
                        let (stars_reached, stars_total) = with_puzzle_collection_store(|store| {
                            store.core_puzzle_collections().get(index).unwrap().stars()
                        });
                        collection_item
                            .set_star_counts(stars_reached as usize, stars_total as usize);
                    }
                }
                CollectionId::Community(index) => {
                    let row = self.community_collection_list.row_at_index(index as i32);
                    if let Some(row) = row {
                        let collection_item: CollectionSelectionItem = row.downcast().unwrap();
                        let (stars_reached, stars_total) = with_puzzle_collection_store(|store| {
                            store
                                .community_puzzle_collections()
                                .get(index)
                                .unwrap()
                                .stars()
                        });
                        collection_item
                            .set_star_counts(stars_reached as usize, stars_total as usize);
                    }
                }
            }
        }
    }

    pub fn refresh(&self) {
        let current_collection_id = self.current_collection_id();
        self.load_core_collections();
        self.update_community_collections();
        if let Some(collection_id) = current_collection_id {
            match collection_id {
                CollectionId::Core(index) => {
                    let row = self.core_collection_list.row_at_index(index as i32);
                    if let Some(row) = row {
                        row.activate();
                    }
                }
                CollectionId::Community(index) => {
                    let row = self.community_collection_list.row_at_index(index as i32);
                    if let Some(row) = row {
                        row.activate();
                    }
                }
            }
        }
    }

    fn load_core_collections(&self) {
        self.core_collection_list.remove_all();

        with_puzzle_collection_store(|store| {
            for collection in store.core_puzzle_collections().iter() {
                let row = CollectionSelectionItem::new(collection, true);
                self.core_collection_list.append(&row);
            }
        });
    }

    fn update_community_collections(&self) {
        self.community_collection_list.remove_all();

        with_puzzle_collection_store(|store| {
            for collection in store.community_puzzle_collections().iter() {
                let row = CollectionSelectionItem::new(collection, false);
                self.community_collection_list.append(&row);
            }
        });
    }

    pub fn select_none(&self) {
        self.core_collection_list.select_row(None::<&ListBoxRow>);
        self.community_collection_list
            .select_row(None::<&ListBoxRow>);
    }

    /// Selects the last community collection in the list.
    ///
    /// The callee has to be sure that there is at least one community collection, otherwise this
    /// will panic.
    fn select_last_community_collection(&self) {
        let last_index =
            with_puzzle_collection_store(|store| store.community_puzzle_collections().len()) - 1;
        self.community_collection_list
            .row_at_index(last_index as i32)
            .unwrap()
            .activate();
    }

    /// Selects the last community collection if there are any, otherwise selects the first core collection.
    fn select_community_or_core_collection(&self) {
        let community_count =
            with_puzzle_collection_store(|store| store.community_puzzle_collections().len());
        if community_count > 0 {
            self.select_last_community_collection();
        } else {
            self.core_collection_list
                .row_at_index(0)
                .unwrap()
                .activate();
        }
    }

    fn activate_collection(&self, collection_id: CollectionId) {
        let collection: Option<CollectionModel> =
            with_puzzle_collection_store(|store| match collection_id {
                CollectionId::Core(index) => store.core_puzzle_collections().get(index).cloned(),
                CollectionId::Community(index) => {
                    store.community_puzzle_collections().get(index).cloned()
                }
            });
        match collection {
            None => {
                error!(
                    "Tried to activate non-existing collection: {:?}",
                    collection_id
                );
            }
            Some(c) => {
                let mut state = get_state_mut();
                state.puzzle_collection = Some(c);
                drop(state);
                self.main_presenter.show_puzzle_selection();
            }
        };
    }
}

#[derive(Debug)]
enum CollectionId {
    Core(usize),
    Community(usize),
}

impl From<CollectionId> for Variant {
    fn from(val: CollectionId) -> Self {
        match val {
            CollectionId::Core(index) => Variant::from(index as i32),
            CollectionId::Community(index) => Variant::from(-(index as i32) - 1),
        }
    }
}

impl From<Variant> for CollectionId {
    fn from(value: Variant) -> Self {
        let id = value.get::<i32>().unwrap();
        if id >= 0 {
            CollectionId::Core(id as usize)
        } else {
            CollectionId::Community((-id - 1) as usize)
        }
    }
}
