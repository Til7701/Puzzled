use crate::application::PuzzlemoredaysApplication;
use crate::global::state::get_state;
use crate::window::PuzzlemoredaysWindow;
use adw::prelude::{
    ActionMapExtManual, AdwDialogExt, Cast, PreferencesDialogExt, PreferencesGroupExt,
    PreferencesPageExt,
};
use adw::{gio, ActionRow, Dialog, PreferencesDialog, PreferencesGroup, PreferencesPage};
use puzzle_config::PuzzleConfig;

#[derive(Debug, Clone)]
pub struct PuzzleInfoPresenter {
    window: PuzzlemoredaysWindow,
}

impl PuzzleInfoPresenter {
    pub fn new(window: &PuzzlemoredaysWindow) -> Self {
        PuzzleInfoPresenter {
            window: window.clone(),
        }
    }

    pub fn register_actions(&self, app: &PuzzlemoredaysApplication) {
        let collection_item_activated = gio::ActionEntry::builder("puzzle_info")
            .activate({
                let self_clone = self.clone();
                move |_, _, _| self_clone.show_puzzle_info()
            })
            .build();
        app.add_action_entries([collection_item_activated]);
    }

    pub fn setup(&self) {}

    fn show_puzzle_info(&self) {
        let state = get_state();
        let puzzle_config = &state.puzzle_config;
        if let Some(puzzle_config) = puzzle_config {
            let dialog = self.create_puzzle_info(puzzle_config);
            dialog.present(Some(&self.window));
        }
    }

    fn create_puzzle_info(&self, puzzle_config: &PuzzleConfig) -> Dialog {
        let page = self.create_content_for_puzzle_info(puzzle_config);

        let dialog = PreferencesDialog::builder()
            .title("Puzzle Information")
            .build();
        dialog.add(&page);

        dialog.upcast()
    }

    fn create_content_for_puzzle_info(&self, puzzle_config: &PuzzleConfig) -> PreferencesPage {
        let page = PreferencesPage::builder()
            .title("Puzzle Information")
            .build();

        let general_group = PreferencesGroup::builder()
            .title("General Information")
            .build();

        let name = self.create_row("Puzzle Name", &puzzle_config.name());
        general_group.add(&name);

        let board_dimensions = self.create_row(
            "Board Dimensions",
            &format!(
                "{} x {}",
                puzzle_config.board_config().layout().nrows(),
                puzzle_config.board_config().layout().ncols()
            ),
        );
        general_group.add(&board_dimensions);

        let tile_count = self.create_row(
            "Number of Tiles",
            &format!("{}", puzzle_config.tiles().len()),
        );
        general_group.add(&tile_count);

        page.add(&general_group);

        page
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
