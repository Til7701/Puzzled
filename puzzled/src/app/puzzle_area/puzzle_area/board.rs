use crate::app::puzzle_area::puzzle_area::puzzle_area::PuzzleArea;
use crate::components::board::BoardView;
use crate::model::extension::PuzzleTypeExtension;
use crate::offset::PixelOffset;
use adw::prelude::Cast;
use adw::subclass::prelude::ObjectSubclassIsExt;
use gtk::prelude::{FixedExt, GridExt, WidgetExt};
use gtk::Widget;
use puzzle_config::{PuzzleConfig, TargetIndex};

const TARGET_SELECTION_CLASS: &str = "target-selection";

impl PuzzleArea {
    pub fn setup_board(&self, puzzle_config: &PuzzleConfig) {
        let board_view =
            BoardView::new(puzzle_config.board_config()).expect("Failed to initialize board view");
        let widget = board_view.clone().upcast::<Widget>();
        self.add(&widget, &PixelOffset::default());

        self.imp().board.replace(Some(board_view));
    }

    pub fn update_board_layout(&self) {
        self.update_target_selection();
        let board = self.imp().board.borrow();
        let grid_config = self.imp().grid_config.borrow();
        if let Some(board_view) = board.as_ref() {
            let widget = board_view.clone().upcast::<Widget>();
            let pos = grid_config
                .board_offset_cells
                .mul_scalar(grid_config.cell_size_pixel as f64);
            self.move_(&widget, pos.0 as f64, pos.1 as f64);
            for widget in board_view.elements().iter() {
                widget.set_width_request(grid_config.cell_size_pixel as i32);
                widget.set_height_request(grid_config.cell_size_pixel as i32);
            }
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
            target.indices.iter().for_each(|TargetIndex(x, y)| {
                if let Some(widget) = board_view.child_at(*x as i32, *y as i32) {
                    widget.add_css_class(TARGET_SELECTION_CLASS);
                }
            })
        }
    }

    fn clear_target_selection(&self) {
        let board = self.imp().board.borrow();
        if let Some(board_view) = board.as_ref() {
            board_view.elements().iter().for_each(|widget| {
                widget.remove_css_class(TARGET_SELECTION_CLASS);
            });
        }
    }

    pub fn get_min_element_width(&self) -> i32 {
        let board = self.imp().board.borrow();
        if let Some(board_view) = board.as_ref() {
            board_view.get_min_element_size()
        } else {
            0
        }
    }
}
