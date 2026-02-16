use crate::application::PuzzledApplication;
use crate::global::state::{get_state, get_state_mut, PuzzleTypeExtension, SolverState};
use crate::presenter::puzzle_area::puzzle_state::PuzzleState;
use crate::solver;
use crate::solver::interrupt_solver_call;
use crate::window::PuzzledWindow;
use adw::glib;
use gtk::prelude::{ButtonExt, WidgetExt};
use gtk::Button;
use log::debug;
use puzzle_solver::result::{Solution, UnsolvableReason};
use std::sync::mpsc;
use tokio_util::sync::CancellationToken;

pub type OnComplete = Box<dyn Fn(Result<Solution, UnsolvableReason>)>;

/// Responsible for managing the hint button.
///
/// It calls the solver, when a hint has been requested and updates the icon of the button.
#[derive(Debug, Clone)]
pub struct HintButtonPresenter {
    hint_button: Button,
}

impl HintButtonPresenter {
    pub fn new(window: &PuzzledWindow) -> Self {
        HintButtonPresenter {
            hint_button: window.puzzle_area_nav_page().hint_button().clone(),
        }
    }

    pub fn register_actions(&self, _: &PuzzledApplication) {}

    pub fn setup(&self) {}

    /// Calls the solver and updates the hint button state.
    ///
    /// When the solver is finished, the `on_complete` callback will be called with the result of
    /// the solver.
    ///
    /// # Arguments
    ///
    /// * `puzzle_state`: The current state of the puzzle.
    /// * `on_complete`: Callback to be called when the solver has finished.
    ///
    /// returns: ()
    pub fn calculate_hint(&self, puzzle_state: &mut PuzzleState, on_complete: OnComplete) {
        let state = get_state();
        let calculate_solvability = match &state.puzzle_type_extension {
            None => true,
            Some(PuzzleTypeExtension::Simple) => true,
            Some(PuzzleTypeExtension::Area { target, .. }) => target.is_some(),
        };
        drop(state);
        if calculate_solvability {
            self.calculate_solvability(puzzle_state, on_complete);
        } else {
            self.display_state(&HintButtonState::Bulb);
        }
    }

    fn calculate_solvability(&self, puzzle_state: &mut PuzzleState, on_complete: OnComplete) {
        let mut state = get_state_mut();

        let solver_state = &state.solver_state;
        match solver_state {
            SolverState::Running {
                call_id: _,
                cancel_token: _,
            } => interrupt_solver_call(&state),
            _ => {}
        }

        let (tx, rx) = mpsc::channel::<Result<Solution, UnsolvableReason>>();
        glib::idle_add_local({
            let self_clone = self.clone();
            move || match rx.try_recv() {
                Ok(result) => {
                    self_clone.display_state(&HintButtonState::Bulb);
                    on_complete(result);
                    glib::ControlFlow::Break
                }
                Err(mpsc::TryRecvError::Empty) => glib::ControlFlow::Continue,
                Err(mpsc::TryRecvError::Disconnected) => glib::ControlFlow::Break,
            }
        });

        let call_id = solver::create_solver_call_id();
        debug!("Starting solver call: {:?}", call_id);
        let cancel_token = CancellationToken::new();
        state.solver_state = SolverState::Running {
            call_id: call_id.clone(),
            cancel_token: cancel_token.clone(),
        };
        self.display_state(&HintButtonState::Calculating);
        drop(state);
        solver::solve_for_target(
            &call_id,
            &puzzle_state,
            Box::new(move |result| {
                let _ = tx.send(result);
            }),
            cancel_token,
        );
    }

    pub fn display_state(&self, status: &HintButtonState) {
        match status {
            HintButtonState::Bulb => {
                self.hint_button.set_tooltip_text(Some("Hint"));
                self.hint_button.set_icon_name("lightbulb-symbolic");
            }
            HintButtonState::Calculating { .. } => {
                self.hint_button
                    .set_tooltip_text(Some("Hint: Calculating..."));
                self.hint_button.set_icon_name("timer-sand-symbolic");
            }
        }
    }
}

pub enum HintButtonState {
    Bulb,
    Calculating,
}
