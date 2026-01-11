use crate::presenter::puzzle_area::PuzzleAreaPresenter;
use crate::solver::{interrupt_solver_call, is_solved};
use crate::state::{get_state, SolverState};
use crate::view::{create_puzzle_info, create_solved_dialog, create_target_selection_dialog};
use crate::window::PuzzlemoredaysWindow;
use crate::{puzzle, solver};
use adw::glib;
use adw::prelude::{ActionRowExt, AdwDialogExt, AlertDialogExt};
use gtk::prelude::{ButtonExt, WidgetExt};
use humantime::format_duration;
use log::debug;
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;
use std::sync::{mpsc, Arc};
use std::time::Duration;
use tokio_util::sync::CancellationToken;

#[derive(Debug, Default, Clone)]
pub struct MainPresenter {
    puzzle_area_presenter: Rc<RefCell<PuzzleAreaPresenter>>,
    window: Arc<RefCell<Option<PuzzlemoredaysWindow>>>,
}

impl MainPresenter {
    pub fn set_puzzle_area_presenter(&self, presenter: &PuzzleAreaPresenter) {
        self.puzzle_area_presenter.replace(presenter.clone());
    }

    pub fn setup(&self, window: &PuzzlemoredaysWindow) {
        self.update_target_selection_button();
        self.window.replace(Some(window.clone()));
        let puzzle_area_presenter = self.puzzle_area_presenter.borrow();
        puzzle_area_presenter.set_view(window.grid());
        puzzle_area_presenter.setup_puzzle_config_from_state(Rc::new({
            let self_clone = self.clone();
            move || self_clone.calculate_solvability_if_enabled()
        }));

        let puzzle_selection = window.puzzle_selection();
        puzzle_selection.set_selected(0);

        puzzle_selection.connect_selected_notify({
            let puzzle_area_presenter = puzzle_area_presenter.clone();
            let self_clone = self.clone();
            move |dropdown| {
                let index = dropdown.selected();
                let puzzle_config = match index {
                    0 => puzzle::get_default_config(),
                    1 => puzzle::get_year_config(),
                    _ => panic!("Unknown puzzle selection index: {}", index),
                };
                let mut state = get_state();
                state.target_selection = puzzle_config.default_target.clone();
                state.puzzle_config = puzzle_config;
                drop(state);

                puzzle_area_presenter.setup_puzzle_config_from_state(Rc::new({
                    let self_clone = self_clone.clone();
                    move || self_clone.calculate_solvability_if_enabled()
                }));
            }
        });

        let puzzle_info_button = window.puzzle_info_button();
        puzzle_info_button.connect_clicked({
            let main_presenter = self.clone();
            move |_| {
                main_presenter.show_puzzle_info_dialog();
            }
        });

        window.target_selection_button().connect_clicked({
            let self_clone = self.clone();
            move |_| {
                if let Some(window) = self_clone.window.borrow().as_ref() {
                    let dialog = create_target_selection_dialog();
                    dialog.connect_response(None, {
                        let self_clone = self_clone.clone();
                        move |_, _| {
                            self_clone.update_layout();
                            self_clone
                                .puzzle_area_presenter
                                .borrow()
                                .update_highlights();
                            self_clone.calculate_solvability_if_enabled();
                        }
                    });
                    dialog.present(Some(window));
                }
            }
        });

        window.solver_status().connect_clicked({
            let self_clone = self.clone();
            move |_| {
                self_clone.show_solver_dialog();
            }
        });
    }

    fn show_puzzle_info_dialog(&self) {
        if let Some(window) = self.window.borrow().as_ref() {
            let state = get_state();
            let puzzle_config = &state.puzzle_config;

            let dialog = create_puzzle_info(puzzle_config);
            dialog.present(Some(window));
        }
    }

    pub fn update_layout(&self) {
        self.puzzle_area_presenter.borrow().update_layout();
        self.update_target_selection_button();
    }

    fn update_target_selection_button(&self) {
        if let Some(window) = self.window.borrow().as_ref() {
            let state = get_state();
            let target_selection = &state.target_selection;
            match target_selection {
                Some(target) => {
                    let puzzle_config = &state.puzzle_config;
                    let text = puzzle_config.format_target(target);
                    window.target_selection_button().set_label(&text);
                }
                None => {
                    window
                        .target_selection_button()
                        .set_label("Select Target Day");
                }
            }
        }
    }

    pub fn calculate_solvability_if_enabled(&self) {
        let state = get_state();
        if state.preferences_state.solver_enabled {
            drop(state);
            self.calculate_solvability();
        }
    }

    fn calculate_solvability(&self) {
        let puzzle_state = match self.puzzle_area_presenter.borrow().extract_puzzle_state() {
            Ok(state) => state,
            _ => return,
        };
        let mut state = get_state();
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

        if is_solved(&puzzle_state, &target) {
            state.solver_state = SolverState::Done {
                solvable: true,
                duration: Duration::ZERO,
            };
            drop(state);
            self.set_solver_status(&SolverState::Done {
                solvable: true,
                duration: Duration::ZERO,
            });
            self.show_solved_dialog();
            return;
        }

        let (tx, rx) = mpsc::channel::<SolverState>();
        glib::idle_add_local({
            let self_clone = self.clone();
            move || match rx.try_recv() {
                Ok(solver_status) => {
                    dbg!(&solver_status);
                    self_clone.set_solver_status(&solver_status);
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
        self.set_solver_status(&state.solver_state);
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

    fn set_solver_status(&self, status: &SolverState) {
        if let Some(window) = self.window.borrow().as_ref() {
            let solver_status_button = window.solver_status();
            match status {
                SolverState::Disabled => {
                    solver_status_button.set_tooltip_text(Some("Solver: Disabled"));
                    solver_status_button.set_icon_name("process-stop-symbolic");
                }
                SolverState::Running { .. } => {
                    solver_status_button.set_tooltip_text(Some("Solver: Running..."));
                    solver_status_button.set_icon_name("system-run-symbolic");
                }
                SolverState::Done { solvable, .. } => {
                    if *solvable {
                        solver_status_button.set_tooltip_text(Some("Solver: Solvable!"));
                        solver_status_button.set_icon_name("object-select-symbolic");
                    } else {
                        solver_status_button.set_tooltip_text(Some("Solver: Unsolvable"));
                        solver_status_button.set_icon_name("edit-delete-symbolic");
                    }
                }
            }
        }
    }

    fn show_solved_dialog(&self) {
        if let Some(window) = self.window.borrow().as_ref() {
            let dialog = create_solved_dialog();
            dialog.present(Some(window));
        }
    }

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
                let mut state = get_state();
                let solver_enabled = enable_solver.is_active();
                state.preferences_state.solver_enabled = enable_solver.is_active();
                if solver_enabled {
                    drop(state);
                    self_clone.calculate_solvability();
                } else {
                    self_clone.set_solver_status(&SolverState::Disabled {});
                }
            }
        });

        if let Some(window) = self.window.borrow().as_ref() {
            dialog.present(Some(window));
        }
    }
}
