use crate::application::PuzzlemoredaysApplication;
use crate::presenter::navigation::NavigationPresenter;
use crate::puzzles::get_puzzle_collection_store;
use crate::state::get_state;
use crate::window::PuzzlemoredaysWindow;
use adw::glib::{Variant, VariantTy};
use adw::prelude::ActionMapExtManual;
use adw::{gio, NavigationView};
use gtk::prelude::{ActionableExt, WidgetExt};
use gtk::ListBox;
use log::error;
use puzzle_config::{PuzzleConfig, PuzzleConfigCollection};

#[derive(Debug, Clone)]
pub struct PuzzleSelectionPresenter {
    navigation: NavigationPresenter,
    puzzle_list: ListBox,
}

impl PuzzleSelectionPresenter {
    pub fn new(window: &PuzzlemoredaysWindow, navigation: NavigationPresenter) -> Self {
        PuzzleSelectionPresenter {
            navigation,
            puzzle_list: window.puzzle_list(),
        }
    }

    pub fn register_actions(&self, app: &PuzzlemoredaysApplication) {
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

    pub fn show_collection(&self, collection: &PuzzleConfigCollection) {
        self.puzzle_list.remove_all();

        for (i, puzzle) in collection.puzzles().iter().enumerate() {
            let row = create_puzzle_row(i as u32, puzzle);
            self.puzzle_list.append(&row);
        }
    }

    fn activate_puzzle(&self, index: u32) {
        let state = get_state();
        let collection = &state.puzzle_collection;
        match collection {
            None => {
                error!("No puzzle collection selected");
            }
            Some(c) => self
                .navigation
                .show_puzzle_area(&c.puzzles()[index as usize]),
        };
    }
}

fn create_puzzle_row(index: u32, collection: &PuzzleConfig) -> gtk::ListBoxRow {
    const RESOURCE_PATH: &str = "/de/til7701/PuzzleMoreDays/puzzle-selection-item.ui";
    let builder = gtk::Builder::from_resource(RESOURCE_PATH);
    let row: gtk::ListBoxRow = builder
        .object("row")
        .expect("Missing `puzzle-selection-item.ui` in resource");

    let puzzle_name_label: gtk::Label = builder
        .object("puzzle_name_label")
        .expect("Missing `puzzle_name_label` in puzzle-selection-item.ui");
    puzzle_name_label.set_text(collection.name());

    let puzzle_description_label: gtk::Label = builder
        .object("puzzle_description_label")
        .expect("Missing `puzzle_description_label` in puzzle-selection-item.ui");
    match collection.description() {
        None => puzzle_description_label.set_visible(false),
        Some(description) => puzzle_description_label.set_text(description),
    }

    row.set_action_target_value(Some(&Variant::from(index)));

    row
}
