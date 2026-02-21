use crate::application::PuzzledApplication;
use crate::global::puzzle_meta::PuzzleMeta;
use crate::global::state::{get_state, get_state_mut};
use crate::presenter::main::{MainPresenter, MIN_WINDOW_HEIGHT, MIN_WINDOW_WIDTH};
use crate::view::board::BoardView;
use crate::view::info_pill::InfoPill;
use crate::view::puzzle_mod::PuzzleMod;
use crate::view::tile::TileView;
use crate::window::PuzzledWindow;
use adw::glib::{Variant, VariantTy};
use adw::prelude::{ActionMapExtManual, ObjectExt, StaticType};
use adw::{gio, WrapBox};
use gtk::prelude::{ActionableExt, BoxExt, FixedExt, ListBoxRowExt, WidgetExt};
use gtk::{Align, Fixed, Label, ListBox};
use log::error;
use puzzle_config::{
    BoardConfig, ProgressionConfig, PuzzleConfig, PuzzleConfigCollection, TileConfig,
};

const CELL_SIZE: f64 = 20.0;

#[derive(Clone)]
pub struct PuzzleSelectionPresenter {
    window: PuzzledWindow,
    navigation: MainPresenter,
    puzzle_name_label: Label,
    puzzle_description_label: Label,
    collection_info_box: WrapBox,
    puzzle_count_pill: InfoPill,
    author_pill: InfoPill,
    version_pill: InfoPill,
    puzzle_list: ListBox,
    puzzle_meta: PuzzleMeta,
}

impl PuzzleSelectionPresenter {
    pub fn new(window: &PuzzledWindow, navigation: MainPresenter) -> Self {
        let page = window.puzzle_selection_nav_page();
        PuzzleSelectionPresenter {
            window: window.clone(),
            navigation,
            puzzle_name_label: page.puzzle_name_label(),
            puzzle_description_label: page.puzzle_description_label(),
            collection_info_box: page.collection_info_box(),
            puzzle_count_pill: page.puzzle_count_pill(),
            author_pill: page.author_pill(),
            version_pill: page.version_pill(),
            puzzle_list: page.puzzle_list(),
            puzzle_meta: PuzzleMeta::new(),
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

    pub fn setup(&self) {
        self.show_collection();
    }

    pub fn show_collection(&self) {
        self.window.set_width_request(MIN_WINDOW_WIDTH);
        self.window.set_height_request(MIN_WINDOW_HEIGHT);
        self.puzzle_list.remove_all();

        let state = get_state();
        if let Some(collection) = &state.puzzle_collection {
            self.puzzle_name_label.set_label(collection.name());
            if let Some(description) = collection.description() {
                self.puzzle_description_label.set_label(description);
                self.puzzle_description_label.set_visible(true);
            } else {
                self.puzzle_description_label.set_visible(false);
            }

            let puzzle_count = collection.puzzles().len();
            self.puzzle_count_pill
                .set_label(format!("{}", puzzle_count));
            self.author_pill
                .set_label(format!("{}", collection.author()));
            if let Some(version) = collection.version() {
                self.version_pill.set_label(format!("{}", version));
                if self.version_pill.parent().is_none() {
                    self.collection_info_box.append(&self.version_pill);
                }
            } else {
                if self.version_pill.parent().is_some() {
                    self.collection_info_box.remove(&self.version_pill);
                }
            }

            for puzzle in collection.puzzles().iter() {
                let row = self.create_puzzle_row(puzzle, collection);
                self.puzzle_list.append(&row);
            }

            match collection.progression() {
                ProgressionConfig::Any => {
                    self.puzzle_list.add_css_class("boxed-list-separate");
                    self.puzzle_list.remove_css_class("boxed-list");
                }
                ProgressionConfig::Sequential => {
                    self.puzzle_list.add_css_class("boxed-list");
                    self.puzzle_list.remove_css_class("boxed-list-separate");
                }
            }
        }
    }

    fn activate_puzzle(&self, index: u32) {
        let mut state = get_state_mut();
        let collection = &state.puzzle_collection;
        match collection {
            None => {
                error!("No puzzle collection selected");
            }
            Some(c) => {
                let puzzle_config = c.puzzles()[index as usize].clone();
                state.setup_for_puzzle(puzzle_config);
                drop(state);
                self.navigation.show_puzzle_area();
            }
        };
    }

    fn create_puzzle_row(
        &self,
        puzzle: &PuzzleConfig,
        collection: &PuzzleConfigCollection,
    ) -> gtk::ListBoxRow {
        PuzzleMod::static_type();
        const RESOURCE_PATH: &str = "/de/til7701/Puzzled/puzzle-selection-item.ui";
        let builder = gtk::Builder::from_resource(RESOURCE_PATH);
        let row: gtk::ListBoxRow = builder
            .object("row")
            .expect("Missing `puzzle-selection-item.ui` in resource");
        row.set_action_target_value(Some(&Variant::from(puzzle.index() as u32)));

        let name_label: Label = builder.object("name").expect("Missing `name` in resource");
        name_label.set_label(puzzle.name());

        let solved = self.puzzle_meta.is_solved(
            collection,
            puzzle.index(),
            &get_state().puzzle_type_extension,
        );

        #[derive(Debug, PartialEq)]
        enum State {
            Solved,
            Unlocked,
            Locked,
        }

        let state = match &collection.progression() {
            ProgressionConfig::Any => {
                if solved {
                    State::Solved
                } else {
                    State::Unlocked
                }
            }
            ProgressionConfig::Sequential => {
                let previous_solved = if puzzle.index() == 0 {
                    true
                } else {
                    self.puzzle_meta
                        .is_solved(collection, puzzle.index() - 1, &None)
                };

                if solved {
                    State::Solved
                } else if previous_solved {
                    State::Unlocked
                } else {
                    State::Locked
                }
            }
        };

        let puzzle_mod: PuzzleMod = builder
            .object("puzzle_mod")
            .expect("Missing `puzzle_mod` in resource");
        match state {
            State::Solved => {
                puzzle_mod.set_solved(self.puzzle_meta.hints(
                    collection,
                    puzzle.index(),
                    &get_state().puzzle_type_extension,
                ));
                row.set_activatable(true);
                row.remove_css_class("dimmed");
            }
            State::Unlocked => {
                puzzle_mod.set_off();
                row.set_activatable(true);
                row.remove_css_class("dimmed");
            }
            State::Locked => {
                puzzle_mod.set_locked();
                row.set_activatable(false);
                row.add_css_class("dimmed");
            }
        }

        let description_label: Label = builder
            .object("description")
            .expect("Missing `description` in resource");
        if let Some(description) = puzzle.description() {
            description_label.set_label(description);
        } else {
            let outer_box: gtk::Box = builder
                .object("outer_box")
                .expect("Missing `outer_box` in resource");
            outer_box.remove(&description_label);
        }

        let info_box: WrapBox = builder
            .object("info_box")
            .expect("Missing `info_box` in resource");

        let board_size_pill: InfoPill = builder
            .object("board_size_pill")
            .expect("Missing `board_size_pill` in resource");
        let cell_count_pill: InfoPill = builder
            .object("cell_count_pill")
            .expect("Missing `cell_count_pill` in resource");
        if state != State::Locked || collection.preview().show_board_size() {
            let (width, height) = puzzle.board_config().layout().dim();
            board_size_pill.set_label(format!("{} x {}", width, height));
            let cell_count = puzzle
                .board_config()
                .layout()
                .iter()
                .filter(|c| **c)
                .count();
            cell_count_pill.set_label(format!("{}", cell_count));
        } else {
            info_box.remove(&board_size_pill);
            info_box.remove(&cell_count_pill);
        }

        let tile_count_pill: InfoPill = builder
            .object("tile_count_pill")
            .expect("Missing `tile_count_pill` in resource");
        if state != State::Locked || collection.preview().show_tile_count() {
            let tile_count = puzzle.tiles().len();
            tile_count_pill.set_label(format!("{}", tile_count));
        } else {
            info_box.remove(&tile_count_pill);
        }

        let difficulty_pill: InfoPill = builder
            .object("difficulty_pill")
            .expect("Missing `difficulty_pill` in resource");
        if let Some(difficulty) = puzzle.difficulty() {
            let label: String = (*difficulty).into();
            difficulty_pill.set_label(label);
        } else {
            info_box.remove(&difficulty_pill);
        }

        if state != State::Locked || collection.preview().show_tiles() {
            let fixed: Fixed = builder
                .object("tile_preview_fixed")
                .expect("Missing `tile_preview_fixed` in resource");
            create_tiles_preview(puzzle.tiles(), fixed);
        }

        if state != State::Locked || collection.preview().show_board() {
            let preview_box: gtk::Box = builder
                .object("board_preview_box")
                .expect("Missing `board_preview_box` in resource");
            create_board_preview(puzzle.board_config(), preview_box);
        }

        row
    }
}

fn create_tiles_preview(tiles: &[TileConfig], fixed: Fixed) {
    let max_tile_cell_height = tiles
        .iter()
        .map(|tile| tile.base().dim().1)
        .max()
        .unwrap_or(1) as i32;
    let mut current_x_offset_cells = 0;

    for (i, tile) in tiles.iter().enumerate() {
        let tile_view = TileView::new(i, tile.base().clone(), tile.color());

        let tile_height = tile.base().dim().1 as i32;
        let y_offset = (max_tile_cell_height - tile_height) as f64 / 2.0;

        fixed.put(
            &tile_view,
            current_x_offset_cells as f64 * CELL_SIZE,
            y_offset * CELL_SIZE,
        );
        tile_view.set_width_request((CELL_SIZE * tile.base().dim().0 as f64) as i32);
        tile_view.set_height_request((CELL_SIZE * tile.base().dim().1 as f64) as i32);
        let tile_width = tile.base().dim().0;
        let next_x_offset = current_x_offset_cells + tile_width + 1;
        current_x_offset_cells = next_x_offset;
    }
}

fn create_board_preview(board: &BoardConfig, preview_box: gtk::Box) {
    let board_view = BoardView::new(board);

    match board_view {
        Ok(bv) => {
            bv.set_property("halign", Align::Center);
            preview_box.append(&bv);

            let min_element_width = bv.get_min_element_size();
            let size_per_cell = CELL_SIZE.max(min_element_width as f64);

            bv.set_width_request(size_per_cell as i32 * board.layout().dim().0 as i32);
            bv.set_height_request(size_per_cell as i32 * board.layout().dim().1 as i32);
        }
        Err(e) => {
            error!("Failed to create board preview: {}", e);
        }
    }
}
