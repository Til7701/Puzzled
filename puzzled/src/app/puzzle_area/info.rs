use crate::app::puzzle_area::puzzle_page::PuzzlePage;
use crate::application::PuzzledApplication;
use adw::prelude::{ActionMapExtManual, AdwDialogExt, Cast, PreferencesGroupExt};
use adw::subclass::prelude::ObjectSubclassIsExt;
use adw::{gio, ActionRow, Dialog};
use gtk::prelude::WidgetExt;
use puzzle_config::PuzzleConfig;
use std::ops::Deref;

impl PuzzlePage {
    pub fn register_actions(&self, app: &PuzzledApplication) {
        let collection_item_activated = gio::ActionEntry::builder("puzzle_info")
            .activate({
                let self_clone = self.clone();
                move |_, _, _| self_clone.show_puzzle_info()
            })
            .build();
        app.add_action_entries([collection_item_activated]);
    }

    pub(super) fn show_puzzle_info(&self) {
        if let Some(puzzle) = self.imp().puzzle.borrow().deref() {
            let dialog = self.create_puzzle_info(puzzle.config());
            dialog.present(self.imp().window.get());
        }
    }

    fn create_puzzle_info(&self, puzzle_config: &PuzzleConfig) -> Dialog {
        const RESOURCE_PATH: &str = "/de/til7701/Puzzled/puzzle-info-dialog.ui";
        let builder = gtk::Builder::from_resource(RESOURCE_PATH);
        let dialog: adw::PreferencesDialog = builder
            .object("puzzle_info_dialog")
            .expect("Missing `puzzle_info_dialog` in resource");

        let general_page: adw::PreferencesGroup = builder
            .object("general_info_group")
            .expect("Missing `general_info_group` in resource");
        let general_rows = self.create_general_content_for_puzzle_info(puzzle_config);
        for action_row in general_rows {
            general_page.add(&action_row);
        }

        let additional_info_group: adw::PreferencesGroup = builder
            .object("additional_info_group")
            .expect("Missing `additional_info_group` in resource");
        let additional_info_rows = self.create_additional_content_for_puzzle_info(puzzle_config);
        if additional_info_rows.is_empty() {
            additional_info_group.set_visible(false);
        } else {
            for action_row in additional_info_rows {
                additional_info_group.add(&action_row);
            }
        }

        dialog.upcast()
    }

    fn create_general_content_for_puzzle_info(
        &self,
        puzzle_config: &PuzzleConfig,
    ) -> Vec<ActionRow> {
        let mut action_rows = Vec::new();

        let name = self.create_row("Puzzle Name", puzzle_config.name());
        action_rows.push(name);

        let board_dimensions = self.create_row(
            "Board Dimensions",
            &format!(
                "{} x {}",
                puzzle_config.board_config().layout().nrows(),
                puzzle_config.board_config().layout().ncols()
            ),
        );
        action_rows.push(board_dimensions);

        let tile_count = self.create_row(
            "Number of Tiles",
            &format!("{}", puzzle_config.tiles().len()),
        );
        action_rows.push(tile_count);

        action_rows
    }

    fn create_additional_content_for_puzzle_info(
        &self,
        puzzle_config: &PuzzleConfig,
    ) -> Vec<ActionRow> {
        let mut action_rows = Vec::new();

        if let Some(additional_info) = puzzle_config.additional_info() {
            for (title, value) in additional_info {
                let row = self.create_row(title, value);
                action_rows.push(row);
            }
        }

        action_rows
    }

    fn create_row(&self, title: &str, value: &str) -> ActionRow {
        ActionRow::builder()
            .title(title)
            .subtitle(value)
            .focusable(false)
            .selectable(false)
            .can_focus(false)
            .css_classes(vec!["property".to_string()])
            .build()
    }
}
