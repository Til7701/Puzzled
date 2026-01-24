use crate::application::PuzzledApplication;
use crate::global::state::get_state_mut;
use crate::presenter::navigation::NavigationPresenter;
use crate::puzzles::{add_community_collection_from_string, get_puzzle_collection_store};
use crate::window::PuzzledWindow;
use adw::gio::{Cancellable, File};
use adw::glib::{Variant, VariantTy};
use adw::prelude::{
    ActionMapExtManual, ActionRowExt, AdwDialogExt, AlertDialogExt, FileExtManual,
    PreferencesRowExt,
};
use adw::{gio, AlertDialog, ButtonRow, ResponseAppearance};
use gtk::prelude::ActionableExt;
use gtk::ListBox;
use log::{debug, error};
use puzzle_config::ReadError::FileReadError;
use puzzle_config::{PuzzleConfigCollection, ReadError};

#[derive(Clone)]
pub struct CollectionSelectionPresenter {
    window: PuzzledWindow,
    navigation: NavigationPresenter,
    core_collection_list: ListBox,
    community_collection_list: ListBox,
    load_collection_button_row: ButtonRow,
}

impl CollectionSelectionPresenter {
    pub fn new(window: &PuzzledWindow, navigation: NavigationPresenter) -> Self {
        CollectionSelectionPresenter {
            window: window.clone(),
            navigation,
            core_collection_list: window.core_collection_list(),
            community_collection_list: window.community_collection_list(),
            load_collection_button_row: window.load_collection_button_row(),
        }
    }

    pub fn register_actions(&self, app: &PuzzledApplication) {
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
                move |_, _, _| self_clone.show_load_collection_dialog()
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

    fn show_load_collection_dialog(&self) {
        let dialog = gtk::FileDialog::builder().build();
        dialog.open(Some(&self.window), None::<&Cancellable>, {
            let self_clone = self.clone();
            move |result| match result {
                Ok(file) => self_clone.load_collection(file),
                Err(error) => {
                    debug!("File dialog error: {:?}", error);
                }
            }
        });
    }

    fn load_collection(&self, file: File) {
        let result = self.try_load_collection(file);
        match result {
            Ok(()) => {
                debug!("Successfully loaded collection.");
            }
            Err(e) => {
                let message: String = match &e {
                    FileReadError(e) => e.clone(),
                    ReadError::MissingVersion => {
                        "The `config_version` field is missing.".to_string()
                    }
                    ReadError::MalformedVersion => {
                        "The `config_version` field is malformed.".to_string()
                    }
                    ReadError::UnsupportedVersion => {
                        "The collection version is not supported. Only version `1` is supported."
                            .to_string()
                    }
                    ReadError::JsonError(e) => {
                        format!("The collection file could not be parsed correctly: {}", e)
                    }
                    ReadError::UnknownPredefinedTile { tile_name, name } => {
                        format!(
                            "The collection file contains an unknown predefined tile '{}' in puzzle '{}'.",
                            tile_name, name
                        )
                    }
                    ReadError::UnknownCustomBoard {
                        puzzle_name,
                        board_name,
                    } => {
                        format!(
                            "The collection file contains an unknown custom board '{}' in puzzle '{}'.",
                            board_name, puzzle_name
                        )
                    }
                    ReadError::TileWidthOrHeightCannotBeZero { tile_name } => {
                        format!(
                            "The collection file contains a tile '{}' with zero width or height.",
                            tile_name
                        )
                    }
                    ReadError::BoardWidthOrHeightCannotBeZero => {
                        "The collection file contains a board with zero width or height."
                            .to_string()
                    }
                };
                self.show_load_collection_error(message);
            }
        }
    }

    fn try_load_collection(&self, file: File) -> Result<(), ReadError> {
        match file.load_contents(None::<&Cancellable>) {
            Ok((bytes, _etag)) => match std::str::from_utf8(bytes.as_ref()) {
                Ok(text) => {
                    let content: String = text.to_owned();
                    add_community_collection_from_string(&content)?;
                    self.update_community_collections();
                    Ok(())
                }
                Err(e) => Err(FileReadError(format!("{}", e))),
            },
            Err(e) => Err(FileReadError(format!("{}", e))),
        }
    }

    fn show_load_collection_error(&self, message: String) {
        let dialog = AlertDialog::builder()
            .heading("Error")
            .body(message)
            .build();

        let ok_id = "ok";
        dialog.add_response(ok_id, "OK");
        dialog.set_default_response(Some(ok_id));
        dialog.set_close_response(ok_id);
        dialog.set_response_appearance(ok_id, ResponseAppearance::Suggested);
        dialog.present(Some(&self.window));
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
    const RESOURCE_PATH: &str = "/de/til7701/Puzzled/puzzle-collection-item.ui";
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
