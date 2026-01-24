use crate::application::PuzzledApplication;
use crate::global::state::{get_state, get_state_mut, PuzzleTypeExtension};
use crate::presenter::navigation::NavigationPresenter;
use crate::window::PuzzledWindow;
use adw::gio;
use adw::glib::{Variant, VariantTy};
use adw::prelude::{ActionMapExtManual, ActionRowExt, PreferencesRowExt};
use gtk::prelude::ActionableExt;
use gtk::ListBox;
use log::error;
use puzzle_config::{BoardConfig, PuzzleConfig};

#[derive(Clone)]
pub struct PuzzleSelectionPresenter {
    navigation: NavigationPresenter,
    puzzle_list: ListBox,
}

impl PuzzleSelectionPresenter {
    pub fn new(window: &PuzzledWindow, navigation: NavigationPresenter) -> Self {
        PuzzleSelectionPresenter {
            navigation,
            puzzle_list: window.puzzle_list(),
        }
    }

    pub fn register_actions(&self, app: &PuzzledApplication) {
        let collection_item_activated = gio::ActionEntry::builder("puzzle_activated")
            .parameter_type(Some(VariantTy::UINT32))
            .activate({
                let self_clone = self.clone();
                move |_, _, v: Option<&Variant>| {
                    if let Some(v) = v {
                        let puzzle_index = v.get::<u32>().unwrap();
                        self_clone.activate_puzzle(puzzle_index);
                    }
                }
            })
            .build();
        app.add_action_entries([collection_item_activated]);
    }

    pub fn setup(&self) {}

    pub fn show_collection(&self) {
        self.puzzle_list.remove_all();

        let state = get_state();
        if let Some(collection) = &state.puzzle_collection {
            for (i, puzzle) in collection.puzzles().iter().enumerate() {
                let row = create_puzzle_row(i as u32, puzzle);
                self.puzzle_list.append(&row);
            }
        }
    }

    fn activate_puzzle(&self, index: u32) {
        let state = get_state();
        let collection = &state.puzzle_collection;
        match collection {
            None => {
                error!("No puzzle collection selected");
            }
            Some(c) => {
                let puzzle_config = c.puzzles()[index as usize].clone();
                drop(state);
                self.setup_state_for_puzzle(puzzle_config);
                self.navigation.show_puzzle_area();
            }
        };
    }

    fn setup_state_for_puzzle(&self, puzzle_config: PuzzleConfig) {
        let mut state = get_state_mut();

        match &puzzle_config.board_config() {
            BoardConfig::Simple { .. } => {
                state.puzzle_type_extension = Some(PuzzleTypeExtension::Simple);
            }
            BoardConfig::Area { .. } => {
                let default_target = puzzle_config.board_config().default_target();
                state.puzzle_type_extension = Some(PuzzleTypeExtension::Area {
                    target: default_target,
                });
            }
        }

        state.puzzle_config = Some(puzzle_config);
    }
}

fn create_puzzle_row(index: u32, collection: &PuzzleConfig) -> adw::ActionRow {
    const RESOURCE_PATH: &str = "/de/til7701/Puzzled/puzzle-selection-item.ui";
    let builder = gtk::Builder::from_resource(RESOURCE_PATH);
    let row: adw::ActionRow = builder
        .object("row")
        .expect("Missing `puzzle-selection-item.ui` in resource");

    row.set_title(collection.name());
    if let Some(description) = collection.description() {
        row.set_subtitle(description);
    }

    row.set_action_target_value(Some(&Variant::from(index)));

    row
}
