use crate::presenter::puzzle_area::PuzzleAreaPresenter;
use crate::puzzle;
use crate::state::get_state;
use crate::window::PuzzlemoredaysWindow;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Default, Clone)]
pub struct MainPresenter {
    puzzle_area_presenter: Rc<RefCell<PuzzleAreaPresenter>>,
}

impl MainPresenter {
    pub fn set_puzzle_area_presenter(&self, presenter: &PuzzleAreaPresenter) {
        self.puzzle_area_presenter.replace(presenter.clone());
    }

    pub fn setup(&self, window: &PuzzlemoredaysWindow) {
        let puzzle_selection = window.puzzle_selection();
        puzzle_selection.set_selected(0);

        puzzle_selection.connect_selected_notify({
            let puzzle_area_presenter = self.puzzle_area_presenter.borrow().clone();
            move |dropdown| {
                let index = dropdown.selected();
                let puzzle_config = match index {
                    0 => puzzle::get_default_config(),
                    1 => puzzle::get_year_config(),
                    _ => panic!("Unknown puzzle selection index: {}", index),
                };
                get_state().puzzle_config = puzzle_config;

                puzzle_area_presenter.setup_puzzle_config_from_state();
            }
        });
    }
}
