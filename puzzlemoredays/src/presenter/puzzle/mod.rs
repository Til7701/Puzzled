mod info;

use crate::application::PuzzlemoredaysApplication;
use crate::presenter::puzzle::info::PuzzleInfoPresenter;
use crate::presenter::puzzle_area::PuzzleAreaPresenter;
use crate::window::PuzzlemoredaysWindow;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct PuzzlePresenter {
    puzzle_info_presenter: PuzzleInfoPresenter,
    puzzle_area_presenter: PuzzleAreaPresenter,
}

impl PuzzlePresenter {
    pub fn new(window: &PuzzlemoredaysWindow) -> Self {
        let puzzle_info_presenter = PuzzleInfoPresenter::new(window);
        let puzzle_area_presenter = PuzzleAreaPresenter::new(window);

        PuzzlePresenter {
            puzzle_info_presenter,
            puzzle_area_presenter,
        }
    }

    pub fn register_actions(&self, app: &PuzzlemoredaysApplication) {
        self.puzzle_info_presenter.register_actions(app);
    }

    pub fn setup(&self) {
        self.puzzle_info_presenter.setup();
    }

    pub fn show_puzzle(&self) {
        self.puzzle_area_presenter.show_puzzle(Rc::new({
            let self_clone = self.clone();
            move || self_clone.on_tile_moved()
        }));
    }

    fn on_tile_moved(&self) {}
}
