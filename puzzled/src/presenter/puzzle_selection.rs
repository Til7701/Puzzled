use crate::application::PuzzledApplication;
use crate::global::state::{get_state, get_state_mut, PuzzleTypeExtension};
use crate::presenter::navigation::NavigationPresenter;
use crate::view::board::BoardView;
use crate::view::tile::TileView;
use crate::window::PuzzledWindow;
use adw::gio;
use adw::glib::{Variant, VariantTy};
use adw::prelude::{ActionMapExtManual, ObjectExt};
use gtk::prelude::{ActionableExt, BoxExt, FixedExt, WidgetExt};
use gtk::{Align, Fixed, ListBox};
use log::error;
use puzzle_config::{BoardConfig, PuzzleConfig, PuzzleDifficultyConfig, TileConfig};

const CELL_SIZE: f64 = 20.0;

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

fn create_puzzle_row(index: u32, puzzle: &PuzzleConfig) -> gtk::ListBoxRow {
    const RESOURCE_PATH: &str = "/de/til7701/Puzzled/puzzle-selection-item.ui";
    let builder = gtk::Builder::from_resource(RESOURCE_PATH);
    let row: gtk::ListBoxRow = builder
        .object("row")
        .expect("Missing `puzzle-selection-item.ui` in resource");

    let name_label: gtk::Label = builder.object("name").expect("Missing `name` in resource");
    name_label.set_label(puzzle.name());

    let description_label: gtk::Label = builder
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

    let board_size_label: gtk::Label = builder
        .object("board_size")
        .expect("Missing `board_size` in resource");
    let (width, height) = puzzle.board_config().layout().dim();
    board_size_label.set_label(&format!("{} x {}", width, height));

    let tile_count_label: gtk::Label = builder
        .object("tile_count")
        .expect("Missing `tile_count` in resource");
    let tile_count = puzzle.tiles().len();
    tile_count_label.set_label(&format!("{}", tile_count));

    if let Some(difficulty) = puzzle.difficulty() {
        let text = match difficulty {
            PuzzleDifficultyConfig::Easy => "Easy",
            PuzzleDifficultyConfig::Medium => "Medium",
            PuzzleDifficultyConfig::Hard => "Hard",
            PuzzleDifficultyConfig::Expert => "Expert",
        };
        let difficulty_label: gtk::Label = builder
            .object("difficulty")
            .expect("Missing `difficulty` in resource");
        difficulty_label.set_label(text);
    } else {
        let info_box: adw::WrapBox = builder
            .object("info_box")
            .expect("Missing `info_box` in resource");
        let difficulty_box: gtk::Box = builder
            .object("difficulty_box")
            .expect("Missing `difficulty_box` in resource");
        info_box.remove(&difficulty_box);
    }

    let fixed: Fixed = builder
        .object("tile_preview_fixed")
        .expect("Missing `tile_preview_fixed` in resource");
    create_tiles_preview(puzzle.tiles(), fixed);

    let preview_box: gtk::Box = builder
        .object("board_preview_box")
        .expect("Missing `board_preview_box` in resource");
    create_board_preview(puzzle.board_config(), preview_box);

    row.set_action_target_value(Some(&Variant::from(index)));

    row
}

fn create_tiles_preview(tiles: &[TileConfig], fixed: Fixed) {
    let max_tile_cell_height = tiles
        .iter()
        .map(|tile| tile.base().dim().1)
        .max()
        .unwrap_or(1) as i32;
    let mut current_x_offset_cells = 0;

    for (i, tile) in tiles.iter().enumerate() {
        let tile_view = TileView::new(i, tile.base().clone());

        let tile_height = tile.base().dim().1 as i32;
        let y_offset = (max_tile_cell_height - tile_height) as f64 / 2.0;

        for (widget, offset) in tile_view.elements_with_offset {
            fixed.put(
                &widget,
                (current_x_offset_cells as f64 + offset.0) * CELL_SIZE,
                (y_offset + offset.1) * CELL_SIZE,
            );
            widget.set_width_request(CELL_SIZE as i32);
            widget.set_height_request(CELL_SIZE as i32);
        }
        let tile_width = tile.base().dim().0;
        let next_x_offset = current_x_offset_cells + tile_width + 1;
        current_x_offset_cells = next_x_offset;
    }
}

fn create_board_preview(board: &BoardConfig, preview_box: gtk::Box) {
    let board_view = BoardView::new(board);

    match board_view {
        Ok(bv) => {
            bv.parent.set_property("halign", Align::Center);
            preview_box.append(&bv.parent);

            let min_element_width = bv.get_min_element_size();
            let size_per_cell = CELL_SIZE.max(min_element_width as f64);

            bv.parent
                .set_width_request(size_per_cell as i32 * board.layout().dim().0 as i32);
            bv.parent
                .set_height_request(size_per_cell as i32 * board.layout().dim().1 as i32);
        }
        Err(e) => {
            error!("Failed to create board preview: {}", e);
        }
    }
}
