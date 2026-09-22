use crate::app::components::board::BoardView;
use crate::app::puzzle::puzzle_area::PuzzleArea;
use crate::model::extension::PuzzleTypeExtension;
use crate::model::placement::PixelPosition;
use adw::prelude::Cast;
use adw::subclass::prelude::ObjectSubclassIsExt;
use gtk::Widget;
use gtk::prelude::{FixedExt, WidgetExt};
use puzzle_config::PuzzleConfig;

const TARGET_SELECTION_CLASS: &str = "target-selection";

impl PuzzleArea {
    pub fn setup_board(&self, puzzle_config: &PuzzleConfig) {
        let board_view =
            BoardView::new(puzzle_config.board_config()).expect("Failed to initialize board view");
        let widget = board_view.upcast_ref::<Widget>();
        self.add(widget, &PixelPosition::default());

        self.imp().board.replace(Some(board_view));
    }

    pub fn update_board_layout(&self) {
        self.update_target_selection();
        let board = self.imp().board.borrow();
        let placement_borrow = self.imp().placement_model.borrow();
        let placement_model = placement_borrow.as_ref().unwrap();
        if let Some(board_view) = board.as_ref() {
            let widget = board_view.upcast_ref::<Widget>();
            let pos = placement_model.board_pixel_position();
            let size = placement_model.board_size();
            self.move_(widget, pos.0, pos.1);
            board_view.set_width_request(size.0 as i32);
            board_view.set_height_request(size.1 as i32);
        }
    }

    fn update_target_selection(&self) {
        self.clear_target_selection();
        let puzzle_type_extension = self.imp().puzzle_type_extension.borrow();
        let board = self.imp().board.borrow();
        if let Some(PuzzleTypeExtension::Area {
            target: Some(target),
        }) = puzzle_type_extension.as_ref()
            && let Some(board_view) = board.as_ref()
        {
            target.indices.iter().for_each(|target_index| {
                let coord = target_index.coord();
                board_view.highlight(coord);
            })
        }
    }

    fn clear_target_selection(&self) {
        let board = self.imp().board.borrow();
        if let Some(board_view) = board.as_ref() {
            board_view.remove_highlights();
        }
    }

    pub fn get_min_element_width(&self) -> u32 {
        let board = self.imp().board.borrow();
        if let Some(board_view) = board.as_ref() {
            board_view.get_min_element_size()
        } else {
            0
        }
    }
}
