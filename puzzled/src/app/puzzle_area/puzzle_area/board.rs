use crate::app::puzzle_area::puzzle_area::puzzle_area::PuzzleArea;
use crate::components::board::BoardView;
use crate::model::extension::PuzzleTypeExtension;
use crate::offset::PixelOffset;
use adw::prelude::Cast;
use gtk::prelude::{FixedExt, GridExt, WidgetExt};
use gtk::Widget;
use puzzle_config::{PuzzleConfig, TargetIndex};

const TARGET_SELECTION_CLASS: &str = "target-selection";

impl PuzzleArea {
    pub fn setup_board(&self, puzzle_config: &PuzzleConfig) {
        let board_view =
            BoardView::new(puzzle_config.board_config()).expect("Failed to initialize board view");
        let widget = board_view.clone().upcast::<Widget>();
        let mut data = self.data.borrow_mut();
        data.add_to_fixed(&widget, &PixelOffset::default());

        data.board_view = Some(board_view);
    }

    pub fn update_board_layout(&self) {
        self.update_target_selection();
        let data = self.data.borrow();
        if let Some(board_view) = &data.board_view {
            let widget = board_view.clone().upcast::<Widget>();
            let grid_config = &data.grid_config;
            let pos = grid_config
                .board_offset_cells
                .mul_scalar(grid_config.cell_size_pixel as f64);
            data.fixed.move_(&widget, pos.0 as f64, pos.1 as f64);
            for widget in board_view.elements().iter() {
                widget.set_width_request(grid_config.cell_size_pixel as i32);
                widget.set_height_request(grid_config.cell_size_pixel as i32);
            }
        }
    }

    fn update_target_selection(&self) {
        self.clear_target_selection();
        let state = get_state();
        let puzzle_type_extension = &state.puzzle_type_extension;
        let data = self.data.borrow();
        if let Some(PuzzleTypeExtension::Area {
            target: Some(target),
        }) = puzzle_type_extension
            && let Some(board_view) = &data.board_view
        {
            target.indices.iter().for_each(|TargetIndex(x, y)| {
                if let Some(widget) = board_view.child_at(*x as i32, *y as i32) {
                    widget.add_css_class(TARGET_SELECTION_CLASS);
                }
            })
        }
    }

    fn clear_target_selection(&self) {
        let data = self.data.borrow();
        if let Some(board_view) = &data.board_view {
            board_view.elements().iter().for_each(|widget| {
                widget.remove_css_class(TARGET_SELECTION_CLASS);
            });
        }
    }

    pub fn get_min_element_width(&self) -> i32 {
        let data = self.data.borrow();
        if let Some(board_view) = &data.board_view {
            board_view.get_min_element_size()
        } else {
            0
        }
    }
}
