use crate::offset::{CellOffset, PixelOffset};
use crate::presenter::puzzle_area::{PuzzleAreaData, WINDOW_TO_BOARD_RATIO};
use crate::puzzle::PuzzleConfig;
use crate::view::BoardView;
use adw::prelude::Cast;
use gtk::prelude::{FixedExt, WidgetExt};
use gtk::Widget;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Default, Clone)]
pub struct BoardPresenter {
    data: Rc<RefCell<PuzzleAreaData>>,
}

impl BoardPresenter {
    pub fn set_data(&mut self, data: Rc<RefCell<PuzzleAreaData>>) {
        self.data = data;
    }

    pub fn setup(&self, puzzle_config: &PuzzleConfig) {
        let board_view = BoardView::new(
            &puzzle_config.board_layout,
            &puzzle_config.meaning_areas,
            &puzzle_config.meaning_values,
            &puzzle_config.display_values,
        )
        .expect("Failed to initialize board view");
        let widget = board_view.parent.clone().upcast::<Widget>();
        let mut data = self.data.borrow_mut();
        data.add_to_fixed(&widget, &PixelOffset::default());

        let grid_h_cell_count =
            (puzzle_config.board_layout.dim().0 as f64 * WINDOW_TO_BOARD_RATIO) as u32;
        let board_offset_horizontal_cells =
            ((grid_h_cell_count - puzzle_config.board_layout.dim().0 as u32) / 2) as i32;

        let grid_config = &mut data.grid_config;
        grid_config.grid_h_cell_count = grid_h_cell_count;
        grid_config.board_offset_cells = CellOffset(board_offset_horizontal_cells, 1);
        data.elements_in_fixed.push(widget.clone());
        data.elements_in_fixed.push(widget);
        data.board_view = Some(board_view);
    }

    pub fn update_layout(&self) {
        let data = self.data.borrow();
        if let Some(board_view) = &data.board_view
            && let Some(fixed) = &data.fixed
        {
            let widget = board_view.parent.clone().upcast::<Widget>();
            let grid_config = &data.grid_config;
            let pos = grid_config
                .board_offset_cells
                .mul_scalar(grid_config.cell_width_pixel as f64);
            fixed.move_(&widget, pos.0 as f64, pos.1 as f64);
            for widget in board_view.elements.iter() {
                widget.set_width_request(grid_config.cell_width_pixel as i32);
                widget.set_height_request(grid_config.cell_width_pixel as i32);
            }
        }
    }
}
