use crate::presenter::puzzle::PuzzlePresenter;
use crate::presenter::puzzle_selection::PuzzleSelectionPresenter;
use crate::window::PuzzlemoredaysWindow;
use adw::NavigationView;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct NavigationPresenter {
    navigation_view: NavigationView,
    presenters: Rc<RefCell<Option<Presenters>>>,
}

impl NavigationPresenter {
    pub fn new(window: &PuzzlemoredaysWindow) -> Self {
        NavigationPresenter {
            navigation_view: window.navigation_view(),
            presenters: Rc::new(RefCell::new(None)),
        }
    }

    pub fn setup(
        &mut self,
        puzzle_selection_presenter: &PuzzleSelectionPresenter,
        puzzle_presenter: &PuzzlePresenter,
    ) {
        *self.presenters.borrow_mut() = Some(Presenters {
            puzzle_selection: puzzle_selection_presenter.clone(),
            puzzle_presenter: puzzle_presenter.clone(),
        });
    }

    pub fn show_puzzle_selection(&self) {
        if let Some(presenters) = &self.presenters.borrow().as_ref() {
            presenters.puzzle_selection.show_collection();
            self.navigation_view.push_by_tag("puzzle-selection");
        }
    }

    pub fn show_puzzle_area(&self) {
        if let Some(presenters) = &self.presenters.borrow().as_ref() {
            presenters.puzzle_presenter.show_puzzle();
            self.navigation_view.push_by_tag("puzzle-area");
        }
    }
}

struct Presenters {
    puzzle_selection: PuzzleSelectionPresenter,
    puzzle_presenter: PuzzlePresenter,
}
