use crate::application::PuzzledApplication;
use crate::global::puzzle_meta::PuzzleMeta;
use crate::global::state::{get_state, get_state_mut};
use crate::presenter::collection_selection::CollectionSelectionPresenter;
use crate::presenter::puzzle::PuzzlePresenter;
use crate::presenter::puzzle_selection::PuzzleSelectionPresenter;
use crate::solver;
use crate::view::solved_dialog::SolvedDialog;
use crate::window::PuzzledWindow;
use adw::prelude::{ActionMapExtManual, AdwDialogExt, AlertDialogExt};
use adw::{gio, NavigationSplitView};
use log::{debug, error, info};
use std::cell::RefCell;
use std::rc::Rc;

pub const MIN_WINDOW_WIDTH: i32 = 320;
pub const MIN_WINDOW_HEIGHT: i32 = 240;

#[derive(Clone)]
pub struct MainPresenter {
    window: PuzzledWindow,
    outer_view: NavigationSplitView,
    inner_view: NavigationSplitView,
    presenters: Rc<RefCell<Option<Presenters>>>,
}

impl MainPresenter {
    pub fn new(window: &PuzzledWindow) -> Self {
        MainPresenter {
            window: window.clone(),
            outer_view: window.outer_view(),
            inner_view: window.inner_view(),
            presenters: Rc::new(RefCell::new(None)),
        }
    }

    pub fn register_actions(&self, app: &PuzzledApplication) {
        let mark_all_puzzles_unsolved = gio::ActionEntry::builder("mark_all_puzzles_unsolved")
            .activate({
                let self_clone = self.clone();
                move |_, _, _| self_clone.handle_mark_all_puzzles_unsolved()
            })
            .build();
        app.add_action_entries([mark_all_puzzles_unsolved]);
    }

    pub fn setup(
        &mut self,
        collection_selection_presenter: &CollectionSelectionPresenter,
        puzzle_selection_presenter: &PuzzleSelectionPresenter,
        puzzle_presenter: &PuzzlePresenter,
    ) {
        *self.presenters.borrow_mut() = Some(Presenters {
            collection_selection: collection_selection_presenter.clone(),
            puzzle_selection: puzzle_selection_presenter.clone(),
            puzzle_presenter: puzzle_presenter.clone(),
        });
        self.outer_view.connect_show_content_notify({
            let self_clone = self.clone();
            move |_| {
                if !self_clone.outer_view.shows_content()
                    && let Some(presenters) = self_clone.presenters.borrow().as_ref()
                {
                    let mut state = get_state_mut();
                    state.puzzle_config = None;
                    state.puzzle_type_extension = None;
                    drop(state);
                    presenters.puzzle_selection.show_collection();
                    solver::interrupt_solver_call(&get_state());
                }
            }
        });
    }

    pub fn show_puzzle_selection(&self) {
        if let Some(presenters) = &self.presenters.borrow().as_ref() {
            presenters.puzzle_selection.show_collection();
            self.inner_view.set_show_content(true);
            self.outer_view.set_show_content(false);
        }
    }

    pub fn show_puzzle_area(&self) {
        if let Some(presenters) = &self.presenters.borrow().as_ref() {
            presenters.puzzle_presenter.show_puzzle();
            self.outer_view.set_show_content(true);
        }
    }

    pub fn on_solved(&self) {
        if let Some(presenters) = &self.presenters.borrow().as_ref() {
            presenters.collection_selection.on_solved();
        }
        let solved_dialog = SolvedDialog::new();

        let state = get_state();
        let has_next = if let Some(collection) = &state.puzzle_collection
            && let Some(puzzle_config) = &state.puzzle_config
        {
            dbg!(&puzzle_config.index());
            dbg!(&collection.puzzles().len());
            puzzle_config.index() < collection.puzzles().len() - 1
        } else {
            error!(
                "Could not determine if next puzzle exists: missing puzzle collection or puzzle config"
            );
            false
        };

        if !has_next {
            debug!("No next puzzle available, removing 'Next' button");
            solved_dialog.remove_response("next");
            solved_dialog.set_default_response(Some("back"));
            solved_dialog.set_response_appearance("back", adw::ResponseAppearance::Suggested);
        } else {
            solved_dialog.connect_response(Some("next"), {
                let self_clone = self.clone();
                move |_, _| self_clone.show_next_puzzle()
            });
        }
        solved_dialog.connect_response(Some("back"), {
            let self_clone = self.clone();
            move |_, _| self_clone.show_puzzle_selection()
        });
        solved_dialog.present(Some(&self.window));
    }

    fn show_next_puzzle(&self) {
        let mut state = get_state_mut();
        let next_puzzle = if let Some(collection) = &state.puzzle_collection
            && let Some(puzzle_config) = &state.puzzle_config
        {
            let next_index = puzzle_config.index() + 1;
            if next_index < collection.puzzles().len() {
                let next_puzzle_config = collection.puzzles()[next_index].clone();
                Some(next_puzzle_config)
            } else {
                error!("Next puzzle index {} is out of bounds", next_index);
                None
            }
        } else {
            error!("Could not load next puzzle: missing puzzle collection or puzzle config");
            None
        };
        if let Some(next_puzzle) = next_puzzle {
            state.setup_for_puzzle(next_puzzle);
            drop(state);
            if let Some(presenters) = self.presenters.borrow().as_ref() {
                presenters.puzzle_presenter.show_puzzle();
            }
        }
    }

    fn handle_mark_all_puzzles_unsolved(&self) {
        const RESOURCE_PATH: &str = "/de/til7701/Puzzled/ui/dialog/mark-unsolved-dialog.ui";
        let builder = gtk::Builder::from_resource(RESOURCE_PATH);
        let dialog: adw::AlertDialog = builder
            .object("dialog")
            .expect("Missing `dialog` in resource");

        dialog.connect_response(Some("mark"), {
            let self_clone = self.clone();
            move |_, _| {
                info!("Marking all puzzles as unsolved");
                let puzzle_meta = PuzzleMeta::new();
                puzzle_meta.reset_all();
                if let Some(presenters) = self_clone.presenters.borrow().as_ref() {
                    presenters.collection_selection.refresh();
                    presenters.puzzle_selection.show_collection();
                }
            }
        });

        dialog.present(Some(&self.window));
    }
}

struct Presenters {
    collection_selection: CollectionSelectionPresenter,
    puzzle_selection: PuzzleSelectionPresenter,
    puzzle_presenter: PuzzlePresenter,
}
