mod extension;
mod hint;
mod info;

use crate::application::PuzzledApplication;
use crate::global::puzzle_meta::PuzzleMeta;
use crate::global::state::{get_state, get_state_mut, SolverState};
use crate::presenter::puzzle::extension::ExtensionPresenter;
use crate::presenter::puzzle::hint::HintButtonPresenter;
use crate::presenter::puzzle::info::PuzzleInfoPresenter;
use crate::presenter::puzzle_area::PuzzleAreaPresenter;
use crate::solver::is_solved;
use crate::view::puzzle_area_page::PuzzleAreaPage;
use crate::window::PuzzledWindow;
use adw::gio;
use adw::prelude::{ActionMapExtManual, NavigationPageExt};
use log::{debug, error};
use std::rc::Rc;
use std::time::Duration;

#[derive(Clone)]
pub struct PuzzlePresenter {
    puzzle_area_nav_page: PuzzleAreaPage,
    puzzle_info_presenter: PuzzleInfoPresenter,
    puzzle_area_presenter: PuzzleAreaPresenter,
    hint_button_presenter: HintButtonPresenter,
    extension_presenter: ExtensionPresenter,
    puzzle_meta: PuzzleMeta,
    puzzle_solved_callback: Option<Rc<dyn Fn()>>,
}

impl PuzzlePresenter {
    pub fn new(window: &PuzzledWindow) -> Self {
        let puzzle_info_presenter = PuzzleInfoPresenter::new(window);
        let puzzle_area_presenter = PuzzleAreaPresenter::new(window);
        let hint_button_presenter = HintButtonPresenter::new(window);
        let extension_presenter = ExtensionPresenter::new(window);

        PuzzlePresenter {
            puzzle_area_nav_page: window.puzzle_area_nav_page(),
            puzzle_info_presenter,
            puzzle_area_presenter,
            hint_button_presenter,
            extension_presenter,
            puzzle_meta: PuzzleMeta::new(),
            puzzle_solved_callback: None,
        }
    }

    pub fn register_actions(&self, app: &PuzzledApplication) {
        self.puzzle_info_presenter.register_actions(app);
        self.hint_button_presenter.register_actions(app);
        self.extension_presenter.register_actions(app);

        let solver_state_action = gio::ActionEntry::builder("hint")
            .activate({
                let self_clone = self.clone();
                move |_, _, _| self_clone.on_hint_requested()
            })
            .build();
        app.add_action_entries([solver_state_action]);
    }

    pub fn setup(&mut self, puzzle_solved_callback: Rc<dyn Fn()>) {
        self.puzzle_info_presenter.setup();
        self.puzzle_area_presenter.setup();
        self.hint_button_presenter.setup();
        self.extension_presenter.setup();
        self.puzzle_solved_callback = Some(puzzle_solved_callback);
    }

    pub fn show_puzzle(&self) {
        self.puzzle_area_presenter.show_puzzle(Rc::new({
            let self_clone = self.clone();
            move || self_clone.on_tile_moved()
        }));
        self.extension_presenter.show_puzzle(Rc::new({
            let self_clone = self.clone();
            move || {
                self_clone.puzzle_area_presenter.update_layout();
                self_clone.puzzle_area_presenter.update_highlights();
            }
        }));
        self.on_tile_moved();
        let state = get_state();
        if let Some(collection) = &state.puzzle_collection
            && let Some(puzzle_config) = &state.puzzle_config
        {
            let title = format!("{} - {}", collection.name(), puzzle_config.name());
            self.puzzle_area_nav_page.set_title(&title);
        }
    }

    fn on_tile_moved(&self) {
        let puzzle_state = self.puzzle_area_presenter.extract_puzzle_state();

        if let Ok(puzzle_state) = puzzle_state {
            if is_solved(&puzzle_state) {
                let mut state = get_state_mut();
                state.solver_state = SolverState::Done {
                    solvable: true,
                    duration: Duration::ZERO,
                };
                drop(state);
                self.handle_solved();
            }
        }
    }

    fn on_hint_requested(&self) {
        let puzzle_state = self.puzzle_area_presenter.extract_puzzle_state();

        if let Ok(mut puzzle_state) = puzzle_state {
            self.hint_button_presenter
                .calculate_hint(&mut puzzle_state, {
                    let self_clone = self.clone();
                    Box::new(move |result| match result {
                        Ok(solution) => {
                            debug!("Hint calculation succeeded: {:?}", solution);
                            let placement = solution.placements().first().unwrap();
                            self_clone.puzzle_area_presenter.show_hint_tile(placement);
                        }
                        Err(reason) => {
                            debug!("Hint calculation failed: {:?}", reason);
                        }
                    })
                });
        }
    }

    fn handle_solved(&self) {
        let state = get_state();
        if let Some(collection) = &state.puzzle_collection
            && let Some(puzzle_config) = &state.puzzle_config
        {
            self.puzzle_meta.set_solved(
                true,
                collection,
                puzzle_config.index(),
                &state.puzzle_type_extension,
            );
        } else {
            error!("Could not mark puzzle as solved: missing puzzle collection or puzzle config");
        }

        drop(state);
        if let Some(callback) = &self.puzzle_solved_callback {
            callback();
        }
    }
}
