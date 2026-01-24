mod info;
mod solver;

use crate::application::PuzzlemoredaysApplication;
use crate::global::state::{get_state_mut, SolverState};
use crate::presenter::puzzle::info::PuzzleInfoPresenter;
use crate::presenter::puzzle::solver::SolverStatePresenter;
use crate::presenter::puzzle_area::PuzzleAreaPresenter;
use crate::solver::is_solved;
use crate::view::create_solved_dialog;
use crate::window::PuzzlemoredaysWindow;
use adw::prelude::AdwDialogExt;
use log::{debug, error};
use std::rc::Rc;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct PuzzlePresenter {
    window: PuzzlemoredaysWindow,
    puzzle_info_presenter: PuzzleInfoPresenter,
    puzzle_area_presenter: PuzzleAreaPresenter,
    solver_state_presenter: SolverStatePresenter,
}

impl PuzzlePresenter {
    pub fn new(window: &PuzzlemoredaysWindow) -> Self {
        let puzzle_info_presenter = PuzzleInfoPresenter::new(window);
        let puzzle_area_presenter = PuzzleAreaPresenter::new(window);
        let solver_state_presenter = SolverStatePresenter::new(window);

        PuzzlePresenter {
            window: window.clone(),
            puzzle_info_presenter,
            puzzle_area_presenter,
            solver_state_presenter,
        }
    }

    pub fn register_actions(&self, app: &PuzzlemoredaysApplication) {
        self.puzzle_info_presenter.register_actions(app);
        self.solver_state_presenter.register_actions(app);
    }

    pub fn setup(&self) {
        self.puzzle_info_presenter.setup();
        self.puzzle_area_presenter.setup();
        self.solver_state_presenter.setup();
    }

    pub fn show_puzzle(&self) {
        self.puzzle_area_presenter.show_puzzle(Rc::new({
            let self_clone = self.clone();
            move || self_clone.on_tile_moved()
        }));
    }

    fn on_tile_moved(&self) {
        let puzzle_state = self.puzzle_area_presenter.extract_puzzle_state();

        match puzzle_state {
            Ok(mut puzzle_state) => {
                if is_solved(&puzzle_state) {
                    let mut state = get_state_mut();
                    state.solver_state = SolverState::Done {
                        solvable: true,
                        duration: Duration::ZERO,
                    };
                    drop(state);
                    self.solver_state_presenter
                        .display_solver_state(&SolverState::Done {
                            solvable: true,
                            duration: Duration::ZERO,
                        });
                    self.show_solved_dialog();
                    return;
                }

                self.solver_state_presenter
                    .calculate_solvability_if_enabled(&mut puzzle_state);
            }
            Err(msg) => {
                debug!(
                    "Failed to extract puzzle state: '{}' This is normal at the start of a drag and drop operation of a tile",
                    msg
                );
            }
        }
    }

    fn show_solved_dialog(&self) {
        let dialog = create_solved_dialog();
        dialog.present(Some(&self.window));
    }
}
