use crate::application::PuzzledApplication;
use crate::global::puzzle_meta::PuzzleMeta;
use crate::global::state::get_state_mut;
use crate::presenter::puzzle::PuzzlePresenter;
use crate::presenter::puzzle_selection::PuzzleSelectionPresenter;
use crate::window::PuzzledWindow;
use adw::prelude::{ActionMapExtManual, AdwDialogExt, AlertDialogExt};
use adw::{gio, NavigationSplitView, Window};
use log::info;
use std::cell::RefCell;
use std::rc::Rc;

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
        puzzle_selection_presenter: &PuzzleSelectionPresenter,
        puzzle_presenter: &PuzzlePresenter,
    ) {
        *self.presenters.borrow_mut() = Some(Presenters {
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
                }
            }
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
                    presenters.puzzle_selection.show_collection();
                }
            }
        });

        dialog.present(Some(&self.window));
    }
}

struct Presenters {
    puzzle_selection: PuzzleSelectionPresenter,
    puzzle_presenter: PuzzlePresenter,
}
