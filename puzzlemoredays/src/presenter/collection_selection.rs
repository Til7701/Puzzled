use crate::application::PuzzlemoredaysApplication;
use crate::global::state::{get_state, get_state_mut};
use crate::presenter::navigation::NavigationPresenter;
use crate::puzzles::get_puzzle_collection_store;
use crate::window::PuzzlemoredaysWindow;
use adw::glib::{Variant, VariantTy};
use adw::prelude::{ActionMapExtManual, ActionRowExt, PreferencesRowExt};
use adw::{gio, ButtonRow};
use gtk::prelude::{ActionableExt, BoxExt};
use gtk::ListBox;
use log::error;
use puzzle_config::PuzzleConfigCollection;

#[derive(Debug, Clone)]
pub struct CollectionSelectionPresenter {
    navigation: NavigationPresenter,
    core_collection_list: ListBox,
    community_collection_list: ListBox,
    load_collection_button_row: ButtonRow,
}

impl CollectionSelectionPresenter {
    pub fn new(window: &PuzzlemoredaysWindow, navigation: NavigationPresenter) -> Self {
        CollectionSelectionPresenter {
            navigation,
            core_collection_list: window.core_collection_list(),
            community_collection_list: window.community_collection_list(),
            load_collection_button_row: window.load_collection_button_row(),
        }
    }

    pub fn register_actions(&self, app: &PuzzlemoredaysApplication) {
        let collection_item_activated = gio::ActionEntry::builder("collection_item_activated")
            .parameter_type(Some(VariantTy::INT32))
            .activate({
                let self_clone = self.clone();
                move |_, _, v: Option<&Variant>| {
                    if let Some(v) = v {
                        let collection_id = CollectionId::from(v.clone());
                        self_clone.activate_collection(collection_id);
                    }
                }
            })
            .build();
        let load_collection_action = gio::ActionEntry::builder("load_collection")
            .activate({
                let self_clone = self.clone();
                move |_, _, _| self_clone.load_collection()
            })
            .build();
        app.add_action_entries([collection_item_activated, load_collection_action]);
    }

    pub fn setup(&self) {
        self.load_core_collections();
        self.update_community_collections();
    }

    fn load_core_collections(&self) {
        self.core_collection_list.remove_all();

        let collection_store = get_puzzle_collection_store();
        for (i, collection) in collection_store
            .core_puzzle_collections()
            .iter()
            .enumerate()
        {
            let row = create_collection_row(CollectionId::Core(i), collection);
            self.core_collection_list.append(&row);
        }
    }

    fn update_community_collections(&self) {
        self.community_collection_list.remove_all();

        let collection_store = get_puzzle_collection_store();
        for (i, collection) in collection_store
            .community_puzzle_collections()
            .iter()
            .enumerate()
        {
            let row = create_collection_row(CollectionId::Community(i), collection);
            self.community_collection_list.append(&row);
        }

        self.community_collection_list
            .append(&self.load_collection_button_row);
    }

    fn load_collection(&self) {
        todo!();
        self.update_community_collections();
    }

    fn activate_collection(&self, collection_id: CollectionId) {
        let collection_store = get_puzzle_collection_store();
        let collection = match collection_id {
            CollectionId::Core(index) => collection_store.core_puzzle_collections().get(index),
            CollectionId::Community(index) => {
                collection_store.community_puzzle_collections().get(index)
            }
        };
        match collection {
            None => {
                error!(
                    "Tried to activate non-existing collection: {:?}",
                    collection_id
                );
            }
            Some(c) => {
                let mut state = get_state_mut();
                state.puzzle_collection = Some(c.clone());
                drop(state);
                self.navigation.show_puzzle_selection();
            }
        };
    }
}

fn create_collection_row(id: CollectionId, collection: &PuzzleConfigCollection) -> adw::ActionRow {
    const RESOURCE_PATH: &str = "/de/til7701/PuzzleMoreDays/puzzle-collection-item.ui";
    let builder = gtk::Builder::from_resource(RESOURCE_PATH);
    let row: adw::ActionRow = builder
        .object("row")
        .expect("Missing `puzzle-collection-item.ui` in resource");

    row.set_title(collection.name());
    if let Some(description) = collection.description() {
        row.set_subtitle(description);
    }

    row.set_action_target_value(Some(&id.into()));

    row
}

#[derive(Debug)]
enum CollectionId {
    Core(usize),
    Community(usize),
}

impl Into<Variant> for CollectionId {
    fn into(self) -> Variant {
        match self {
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
