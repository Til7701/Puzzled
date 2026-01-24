use crate::application::PuzzlemoredaysApplication;
use crate::global::state::{get_state, get_state_mut, SolverState};
use crate::presenter::puzzle_area::puzzle_state::PuzzleState;
use crate::solver;
use crate::solver::{interrupt_solver_call, is_solved};
use crate::window::PuzzlemoredaysWindow;
use adw::prelude::{ActionMapExtManual, ActionRowExt, AdwDialogExt};
use adw::{gio, glib};
use gtk::prelude::{ButtonExt, WidgetExt};
use gtk::Button;
use humantime::format_duration;
use log::debug;
use std::sync::mpsc;
use std::time::Duration;
use tokio_util::sync::CancellationToken;

#[derive(Debug, Clone)]
pub struct SolverStatePresenter {
    window: PuzzlemoredaysWindow,
    solver_status_button: Button,
}

impl SolverStatePresenter {
    pub fn new(window: &PuzzlemoredaysWindow) -> Self {
        SolverStatePresenter {
            window: window.clone(),
            solver_status_button: window.solver_state().clone(),
        }
    }

    pub fn register_actions(&self, app: &PuzzlemoredaysApplication) {
        let solver_state_action = gio::ActionEntry::builder("solver_state")
            .activate({
                let self_clone = self.clone();
                move |_, _, _| self_clone.show_solver_dialog()
            })
            .build();
        app.add_action_entries([solver_state_action]);
    }

    pub fn setup(&self) {}

    fn show_solver_dialog(&self) {
        const RESOURCE_PATH: &str = "/de/til7701/PuzzleMoreDays/solver-dialog.ui";
        let builder = gtk::Builder::from_resource(RESOURCE_PATH);
        let dialog: adw::PreferencesDialog = builder
            .object("solver_dialog")
            .expect("Missing `solver_dialog` in resource");

        let enable_solver = builder
            .object::<adw::SwitchRow>("enable_solver")
            .expect("Missing `enable_solver` in resource");
        let state = get_state();
        enable_solver.set_active(state.preferences_state.solver_enabled);

        if let SolverState::Done {
            solvable: _,
            duration,
        } = state.solver_state
        {
            let last_run_duration = builder
                .object::<adw::ActionRow>("last_run_duration")
                .expect("Missing `last_run_duration` in resource");
            let value = format!("{}", format_duration(duration));
            last_run_duration.set_subtitle(&value);
        }

        drop(state);
        dialog.connect_closed({
            let self_clone = self.clone();
            move |_| {
                let mut state = get_state_mut();
                let solver_enabled = enable_solver.is_active();
                state.preferences_state.solver_enabled = enable_solver.is_active();
                if solver_enabled {
                    drop(state);
                } else {
                    self_clone.display_solver_state(&SolverState::Disabled {});
                }
            }
        });

        dialog.present(Some(&self.window));
    }

    pub fn calculate_solvability_if_enabled(&self, puzzle_state: &PuzzleState) {
        let state = get_state();
        if state.preferences_state.solver_enabled {
            drop(state);
            self.calculate_solvability(puzzle_state);
        }
    }

    fn calculate_solvability(&self, puzzle_state: &PuzzleState) {
        let mut state = get_state_mut();
        let target = match &state.target_selection {
            Some(target) => target.clone(),
            None => return,
        };

        let solver_state = &state.solver_state;
        match solver_state {
            SolverState::Running {
                call_id: _,
                cancel_token: _,
            } => interrupt_solver_call(&state),
            _ => {}
        }

        let (tx, rx) = mpsc::channel::<SolverState>();
        glib::idle_add_local({
            let self_clone = self.clone();
            move || match rx.try_recv() {
                Ok(solver_status) => {
                    dbg!(&solver_status);
                    self_clone.display_solver_state(&solver_status);
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
            &target,
            Box::new(move |solver_status| {
                let _ = tx.send(solver_status);
            }),
            cancel_token,
        );
    }

    pub(crate) fn display_solver_state(&self, status: &SolverState) {
        match status {
            SolverState::Initial => {
                self.solver_status_button.set_tooltip_text(Some("Solver"));
                self.solver_status_button
                    .set_icon_name("circle-outline-thick-symbolic");
            }
            SolverState::NotAvailable => {
                self.solver_status_button
                    .set_tooltip_text(Some("Solver: Not Available without Target Day"));
                self.solver_status_button
                    .set_icon_name("stop-sign-large-outline-symbolic");
            }
            SolverState::Disabled => {
                self.solver_status_button
                    .set_tooltip_text(Some("Solver: Disabled"));
                self.solver_status_button
                    .set_icon_name("stop-sign-large-outline-symbolic");
            }
            SolverState::Running { .. } => {
                self.solver_status_button
                    .set_tooltip_text(Some("Solver: Running..."));
                self.solver_status_button
                    .set_icon_name("timer-sand-symbolic");
            }
            SolverState::Done { solvable, .. } => {
                if *solvable {
                    self.solver_status_button
                        .set_tooltip_text(Some("Solver: Solvable for current Target Day!"));
                    self.solver_status_button
                        .set_icon_name("check-round-outline2-symbolic");
                } else {
                    self.solver_status_button
                        .set_tooltip_text(Some("Solver: Unsolvable for current Target Day!"));
                    self.solver_status_button
                        .set_icon_name("cross-large-circle-outline-symbolic");
                }
            }
        }
    }
}
