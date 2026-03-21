use crate::app::puzzle_selection::puzzle_mod::PuzzleModState;
use crate::components::board::BoardView;
use crate::components::tile::TileView;
use crate::model::collection::CollectionModel;
use crate::model::puzzle::PuzzleModel;
use adw::gio;
use adw::glib;
use adw::prelude::{Cast, ObjectExt};
use adw::subclass::prelude::*;
use gtk::prelude::{ActionableExt, BoxExt, FixedExt, ListBoxRowExt, WidgetExt};
use gtk::{Align, Fixed, Widget};
use log::error;
use puzzle_config::{BoardConfig, ProgressionConfig, TileConfig};

const PREVIEW_CELL_SIZE: f64 = 20.0;

mod imp {
    use super::*;
    use crate::app::puzzle_selection::puzzle_mod::PuzzleMod;
    use crate::components::info_pill::InfoPill;
    use std::cell::OnceCell;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/de/til7701/Puzzled/ui/widget/puzzle-selection-item.ui")]
    pub struct PuzzledPuzzleSelectionItem {
        #[template_child]
        pub outer_box: TemplateChild<gtk::Box>,

        #[template_child]
        pub name: TemplateChild<gtk::Label>,
        #[template_child]
        pub puzzle_mod: TemplateChild<PuzzleMod>,
        #[template_child]
        pub description: TemplateChild<gtk::Label>,
        #[template_child]
        pub info_box: TemplateChild<adw::WrapBox>,
        #[template_child]
        pub board_size_pill: TemplateChild<InfoPill>,
        #[template_child]
        pub cell_count_pill: TemplateChild<InfoPill>,
        #[template_child]
        pub tile_count_pill: TemplateChild<InfoPill>,
        #[template_child]
        pub difficulty_pill: TemplateChild<InfoPill>,

        #[template_child]
        pub tile_preview_fixed: TemplateChild<Fixed>,
        #[template_child]
        pub board_preview_box: TemplateChild<gtk::Box>,

        pub(super) puzzle: OnceCell<PuzzleModel>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PuzzledPuzzleSelectionItem {
        const NAME: &'static str = "PuzzledPuzzleSelectionItem";
        type Type = PuzzleSelectionItem;
        type ParentType = gtk::ListBoxRow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for PuzzledPuzzleSelectionItem {}
    impl WidgetImpl for PuzzledPuzzleSelectionItem {}
    impl ListBoxRowImpl for PuzzledPuzzleSelectionItem {}
}

glib::wrapper! {
    pub struct PuzzleSelectionItem(ObjectSubclass<imp::PuzzledPuzzleSelectionItem>)
        @extends Widget, gtk::ListBoxRow,
         @implements gtk::Buildable, gtk::Accessible, gtk::ConstraintTarget,
                  gtk::Native, gio::ActionGroup, gio::ActionMap, gtk::Actionable;
}

impl PuzzleSelectionItem {
    pub fn new(collection: &CollectionModel, puzzle: &PuzzleModel) -> Self {
        let obj: PuzzleSelectionItem = glib::Object::builder().build();
        let imp = obj.imp();
        let stars = puzzle.stars_default();
        let solved = puzzle.is_solved_default();

        imp.name.set_text(puzzle.config().name());

        let state = {
            let state = match &collection.config().progression() {
                ProgressionConfig::Any => PuzzleModState::Stars(stars),
                ProgressionConfig::Sequential => {
                    let previous_solved = puzzle.is_previous_solved_default().unwrap_or(true);
                    if solved || previous_solved {
                        PuzzleModState::Stars(stars)
                    } else {
                        PuzzleModState::Locked
                    }
                }
            };
            if let PuzzleModState::Stars(_) = state
                && puzzle.config().is_unsolvable()
            {
                PuzzleModState::Unsolvable
            } else {
                state
            }
        };
        imp.puzzle_mod.set_state(&state);
        match state {
            PuzzleModState::Stars(_) => {
                obj.set_activatable(true);
                obj.remove_css_class("dimmed");
            }
            PuzzleModState::Locked => {
                obj.set_activatable(false);
                obj.add_css_class("dimmed");
            }
            PuzzleModState::Unsolvable => {
                obj.set_activatable(true);
                obj.remove_css_class("dimmed");
            }
        }

        if let Some(description) = puzzle.config().description() {
            imp.description.set_text(description);
        } else {
            imp.outer_box.remove(&imp.description.get());
        }

        if state != PuzzleModState::Locked || collection.config().preview().show_board_size() {
            let (width, height) = puzzle.config().board_config().layout().dim();
            imp.board_size_pill
                .set_label(format!("{} x {}", width, height));
            let cell_count = puzzle
                .config()
                .board_config()
                .layout()
                .iter()
                .filter(|c| **c)
                .count();
            imp.cell_count_pill.set_label(format!("{}", cell_count));
        } else {
            imp.info_box.remove(&imp.board_size_pill.get());
            imp.info_box.remove(&imp.cell_count_pill.get());
        }

        if state != PuzzleModState::Locked || collection.config().preview().show_tile_count() {
            let tile_count = puzzle.config().tiles().len();
            imp.tile_count_pill.set_label(format!("{}", tile_count));
        } else {
            imp.info_box.remove(&imp.tile_count_pill.get());
        }

        if let Some(difficulty) = puzzle.config().difficulty() {
            let label: String = (*difficulty).into();
            imp.difficulty_pill.set_label(label);
        } else {
            imp.info_box.remove(&imp.difficulty_pill.get());
        }

        if state != PuzzleModState::Locked || collection.config().preview().show_tiles() {
            Self::create_tiles_preview(puzzle.config().tiles(), &imp.tile_preview_fixed.get());
        }

        if state != PuzzleModState::Locked || collection.config().preview().show_board() {
            Self::create_board_preview(
                puzzle.config().board_config(),
                &imp.board_preview_box.get(),
            );
        }

        obj
    }

    fn create_tiles_preview(tiles: &[TileConfig], fixed: &Fixed) {
        let max_tile_cell_height = tiles
            .iter()
            .map(|tile| tile.base().dim().1)
            .max()
            .unwrap_or(1) as i32;
        let mut current_x_offset_cells = 0;

        for (i, tile) in tiles.iter().enumerate() {
            let tile_view = TileView::new(i, tile.base().clone(), tile.color(), None);

            let tile_height = tile.base().dim().1 as i32;
            let y_offset = (max_tile_cell_height - tile_height) as f64 / 2.0;

            fixed.put(
                &tile_view,
                current_x_offset_cells as f64 * PREVIEW_CELL_SIZE,
                y_offset * PREVIEW_CELL_SIZE,
            );
            tile_view.set_width_request((PREVIEW_CELL_SIZE * tile.base().dim().0 as f64) as i32);
            tile_view.set_height_request((PREVIEW_CELL_SIZE * tile.base().dim().1 as f64) as i32);
            let tile_width = tile.base().dim().0;
            let next_x_offset = current_x_offset_cells + tile_width + 1;
            current_x_offset_cells = next_x_offset;
        }
    }

    fn create_board_preview(board: &BoardConfig, preview_box: &gtk::Box) {
        let board_view = BoardView::new(board);

        match board_view {
            Ok(bv) => {
                bv.set_property("halign", Align::Center);
                preview_box.append(&bv);

                let min_element_width = bv.get_min_element_size();
                let size_per_cell = PREVIEW_CELL_SIZE.max(min_element_width as f64);

                bv.set_width_request(size_per_cell as i32 * board.layout().dim().0 as i32);
                bv.set_height_request(size_per_cell as i32 * board.layout().dim().1 as i32);
            }
            Err(e) => {
                error!("Failed to create board preview: {}", e);
            }
        }
    }

    pub fn puzzle(&self) -> &PuzzleModel {
        self.imp().puzzle.get().unwrap()
    }
}
