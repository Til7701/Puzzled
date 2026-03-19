use crate::app::collection_selection::collection_selection_page::CollectionSelectionPage;
use crate::config;
use crate::model::store::with_puzzle_collection_store;
use adw::gio::{Cancellable, File};
use adw::prelude::{AdwDialogExt, AlertDialogExt, FileExtManual};
use adw::{AlertDialog, ResponseAppearance};
use gtk::FileFilter;
use log::debug;
use puzzle_config::ReadError;
use puzzle_config::ReadError::FileReadError;

impl CollectionSelectionPage {
    pub(super) fn show_load_collection_dialog(&self) {
        debug!("Showing load collection dialog.");
        let filter = FileFilter::new();
        filter.set_name(Some("Puzzled Collection Files"));
        filter.add_pattern("*.json");
        let dialog = gtk::FileDialog::builder().default_filter(&filter).build();
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
                    with_puzzle_collection_store(|store| {
                        store.add_community_collection_from_string(&content)
                    })?;
                    self.load_community_collections(); // TODO only update necessary rows instead of reloading everything
                    self.select_last_community_collection();
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
}
