use crate::presenter::puzzle::PuzzlePresenter;
use crate::presenter::puzzle_selection::PuzzleSelectionPresenter;
use crate::window::PuzzledWindow;
use adw::NavigationSplitView;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct NavigationPresenter {
    outer_view: NavigationSplitView,
    inner_view: NavigationSplitView,
    presenters: Rc<RefCell<Option<Presenters>>>,
}

impl NavigationPresenter {
    pub fn new(window: &PuzzledWindow) -> Self {
        NavigationPresenter {
            outer_view: window.outer_view(),
            inner_view: window.inner_view(),
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
            self.inner_view.set_show_content(true);
        }
    }

    pub fn show_puzzle_area(&self) {
        if let Some(presenters) = &self.presenters.borrow().as_ref() {
            presenters.puzzle_presenter.show_puzzle();
            self.outer_view.set_show_content(true);
        }
    }
}

struct Presenters {
    puzzle_selection: PuzzleSelectionPresenter,
    puzzle_presenter: PuzzlePresenter,
}
