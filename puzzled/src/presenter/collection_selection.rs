use crate::application::PuzzledApplication;
use crate::config;
use crate::global::state::get_state_mut;
use crate::presenter::main::MainPresenter;
use crate::puzzles::get_puzzle_collection_store;
use crate::view::info_pill::InfoPill;
use crate::window::PuzzledWindow;
use adw::gio::{Cancellable, File};
use adw::glib::{Variant, VariantTy};
use adw::prelude::{ActionMapExtManual, AdwDialogExt, AlertDialogExt, FileExtManual};
use adw::{gio, AlertDialog, ResponseAppearance};
use gtk::prelude::{ActionableExt, BoxExt, ListBoxRowExt, WidgetExt};
use gtk::ListBox;
use log::{debug, error};
use puzzle_config::ReadError::FileReadError;
use puzzle_config::{PuzzleConfigCollection, ReadError};

#[derive(Clone)]
pub struct CollectionSelectionPresenter {
    window: PuzzledWindow,
    navigation: MainPresenter,
    core_collection_list: ListBox,
    community_collection_list: ListBox,
}

impl CollectionSelectionPresenter {
    pub fn new(window: &PuzzledWindow, navigation: MainPresenter) -> Self {
        let page = window.collection_selection_nav_page();
        CollectionSelectionPresenter {
            window: window.clone(),
            navigation,
            core_collection_list: page.core_collection_list(),
            community_collection_list: page.community_collection_list(),
        }
    }

    pub fn register_actions(&self, app: &PuzzledApplication) {
        let load_collection_action = gio::ActionEntry::builder("load_collection")
            .activate({
                let self_clone = self.clone();
                move |_, _, _| self_clone.show_load_collection_dialog()
            })
            .build();
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
                            let mut store = get_puzzle_collection_store();
                            store.remove_community_collection(&collection_id);
                            drop(store);
                            self_clone.update_community_collections();
                            self_clone.select_community_or_core_collection();
                        }
                    }
                })
                .build();
        app.add_action_entries([load_collection_action, delete_community_collection_action]);
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
                    self_clone.activate_collection(collection_id);
                }
            }
        });
    }

    fn load_core_collections(&self) {
        self.core_collection_list.remove_all();

        let collection_store = get_puzzle_collection_store();
        for collection in collection_store.core_puzzle_collections().iter() {
            let row = create_collection_row(collection, true);
            self.core_collection_list.append(&row);
        }
    }

    fn update_community_collections(&self) {
        self.community_collection_list.remove_all();

        let collection_store = get_puzzle_collection_store();
        for collection in collection_store.community_puzzle_collections().iter() {
            let row = create_collection_row(collection, false);
            self.community_collection_list.append(&row);
        }
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
                    ReadError::MissingVersion => "The `puzzled` field is missing.".to_string(),
                    ReadError::MalformedVersion => "The `puzzled` field is malformed.".to_string(),
                    ReadError::UnsupportedVersion => {
                        format!(
                            "The collection is requiring a higher version of Puzzled. Only version {} or lower is supported.",
                            config::VERSION
                        )
                    }
                    ReadError::JsonError(e) => {
                        format!("The collection file could not be parsed correctly: {}", e)
                    }
                    ReadError::UnknownPredefinedTile { name } => {
                        format!(
                            "The collection file contains an unknown predefined tile in puzzle '{}'.",
                            name
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
                    ReadError::TileWidthOrHeightCannotBeZero => {
                        "The collection file contains a tile with zero width or height.".to_string()
                    }
                    ReadError::BoardWidthOrHeightCannotBeZero => {
                        "The collection file contains a board with zero width or height."
                            .to_string()
                    }
                    ReadError::InvalidVersion(_) => {
                        "The version in the `puzzled` field is invalid.".to_string()
                    }
                    ReadError::InvalidCollectionId(_) => {
                        "The collection file contains an invalid collection ID.".to_string()
                    }
                    ReadError::InvalidColor { message } => {
                        format!("The collection file contains an invalid color: {}", message)
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
                    let mut store = get_puzzle_collection_store();
                    store.add_community_collection_from_string(&content)?;
                    drop(store);
                    self.update_community_collections();
                    self.select_last_community_collection();
                    Ok(())
                }
                Err(e) => Err(FileReadError(format!("{}", e))),
            },
            Err(e) => Err(FileReadError(format!("{}", e))),
        }
    }

    /// Selects the last community collection in the list.
    ///
    /// The callee has to be sure that there is at least one community collection, otherwise this
    /// will panic.
    fn select_last_community_collection(&self) {
        let last_index = {
            get_puzzle_collection_store()
                .community_puzzle_collections()
                .len()
                - 1
        };
        self.community_collection_list
            .row_at_index(last_index as i32)
            .unwrap()
            .activate();
    }

    /// Selects the last community collection if there are any, otherwise selects the first core collection.
    fn select_community_or_core_collection(&self) {
        let community_count = get_puzzle_collection_store()
            .community_puzzle_collections()
            .len();
        if community_count > 0 {
            self.select_last_community_collection();
        } else {
            self.core_collection_list
                .row_at_index(0)
                .unwrap()
                .activate();
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

fn create_collection_row(collection: &PuzzleConfigCollection, core: bool) -> gtk::ListBoxRow {
    const RESOURCE_PATH: &str = "/de/til7701/Puzzled/puzzle-collection-item.ui";
    let builder = gtk::Builder::from_resource(RESOURCE_PATH);
    let row: gtk::ListBoxRow = builder
        .object("row")
        .expect("Missing `puzzle-collection-item.ui` in resource");
    let info_box: adw::WrapBox = builder
        .object("info_box")
        .expect("Missing `info_box` in resource");

    let name_label: gtk::Label = builder.object("name").expect("Missing `name` in resource");
    name_label.set_label(collection.name());

    let puzzle_count_pill: InfoPill = builder
        .object("puzzle_count_pill")
        .expect("Missing `puzzle_count_pill` in resource");
    let puzzle_count_text = format!("{}", collection.puzzles().len());
    puzzle_count_pill.set_label(puzzle_count_text);

    let difficulty_pill: InfoPill = builder
        .object("difficulty_pill")
        .expect("Missing `difficulty_pill` in resource");
    if let Some(difficulty) = collection.average_difficulty() {
        let label: String = difficulty.into();
        difficulty_pill.set_label(label);
    } else {
        info_box.remove(&difficulty_pill);
    }

    let author_pill: InfoPill = builder
        .object("author_pill")
        .expect("Missing `author_pill` in resource");
    if core {
        info_box.remove(&author_pill);
    } else {
        author_pill.set_label(collection.author());
    }

    let version_pill: InfoPill = builder
        .object("version_pill")
        .expect("Missing `version_pill` in resource");
    if let Some(version) = collection.version() {
        version_pill.set_label(version.as_str());
    } else {
        info_box.remove(&version_pill);
    }

    let delete_button: gtk::Button = builder
        .object("delete_button")
        .expect("Missing `delete_button` in resource");
    if !core {
        delete_button.set_action_target_value(Some(&collection.id().to_string().into()));
    } else {
        let main_box: gtk::Box = builder
            .object("main_box")
            .expect("Missing `main_box` in resource");
        main_box.remove(&delete_button);
    }

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
