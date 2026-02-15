use crate::application::PuzzledApplication;
use crate::global::settings::Preferences;
use crate::global::settings::SolverEnabled;
use crate::global::state::{get_state, get_state_mut, PuzzleTypeExtension, SolverState};
use crate::offset::CellOffset;
use crate::presenter::puzzle_area::puzzle_state::PuzzleState;
use crate::solver;
use crate::solver::{interrupt_solver_call, OnCompleteCallback};
use crate::window::PuzzledWindow;
use adw::prelude::{ActionMapExtManual, ActionRowExt, AdwDialogExt};
use adw::{gio, glib};
use gtk::prelude::{ButtonExt, WidgetExt};
use gtk::Button;
use humantime::format_duration;
use log::debug;
use ndarray::Array2;
use puzzle_solver::result::{Solution, UnsolvableReason};
use std::sync::mpsc;
use tokio_util::sync::CancellationToken;

pub type OnComplete = Box<dyn Fn(Result<Solution, UnsolvableReason>)>;

#[derive(Debug, Clone)]
pub struct HintButtonPresenter {
    window: PuzzledWindow,
    hint_button: Button,
    preferences: Preferences,
}

impl HintButtonPresenter {
    pub fn new(window: &PuzzledWindow) -> Self {
        HintButtonPresenter {
            preferences: Preferences::default(),
            window: window.clone(),
            hint_button: window.puzzle_area_nav_page().hint_button().clone(),
        }
    }

    pub fn register_actions(&self, app: &PuzzledApplication) {}

    pub fn setup(&self) {}

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
            self.display_solver_state(&SolverState::Initial {});
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
            move || match rx.try_recv() {
                Ok(result) => {
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
        self.display_solver_state(&state.solver_state);
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

    fn display_solver_state(&self, status: &SolverState) {
        match status {
            SolverState::Initial => {
                self.hint_button.set_tooltip_text(Some("Solver"));
                self.hint_button
                    .set_icon_name("circle-outline-thick-symbolic");
            }
            SolverState::Running { .. } => {
                self.hint_button
                    .set_tooltip_text(Some("Solver: Running..."));
                self.hint_button.set_icon_name("timer-sand-symbolic");
            }
            SolverState::Done { solvable, .. } => {
                if *solvable {
                    self.hint_button
                        .set_tooltip_text(Some("Solver: Solvable for current Target!"));
                    self.hint_button
                        .set_icon_name("check-round-outline2-symbolic");
                } else {
                    self.hint_button
                        .set_tooltip_text(Some("Solver: Unsolvable for current Target!"));
                    self.hint_button
                        .set_icon_name("cross-large-circle-outline-symbolic");
                }
            }
        }
    }
}

enum HintButtonState {
    Bulb,
    Calculating,
    Solvable,
    Unsolvable,
}

pub struct Hint {
    tile_base: Array2<bool>,
    position: CellOffset,
}
