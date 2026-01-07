use crate::presenter::puzzle_area::PuzzleAreaPresenter;
use crate::solver::interrupt_solver_call;
use crate::state::{get_state, SolverStatus};
use crate::view::{create_puzzle_info, create_target_selection_dialog};
use crate::window::PuzzlemoredaysWindow;
use crate::{puzzle, solver};
use adw::glib;
use adw::prelude::{AdwDialogExt, AlertDialogExt};
use gtk::prelude::{ButtonExt, WidgetExt};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{mpsc, Arc};

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
        self.window.replace(Some(window.clone()));
        let puzzle_area_presenter = self.puzzle_area_presenter.borrow();
        puzzle_area_presenter.set_view(window.grid());
        puzzle_area_presenter.setup_puzzle_config_from_state(Rc::new({
            let self_clone = self.clone();
            move || self_clone.calculate_solvability()
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
                state.puzzle_config = puzzle_config;
                state.target_selection = None;
                drop(state);

                puzzle_area_presenter.setup_puzzle_config_from_state(Rc::new({
                    let self_clone = self_clone.clone();
                    move || self_clone.calculate_solvability()
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
                            self_clone.calculate_solvability();
                        }
                    });
                    dialog.present(Some(window));
                }
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

    pub fn calculate_solvability(&self) {
        let puzzle_state = match self.puzzle_area_presenter.borrow().extract_puzzle_state() {
            Ok(state) => state,
            _ => return,
        };
        let mut state = get_state();
        let target = match &state.target_selection {
            Some(target) => target.clone(),
            None => return,
        };
        let solver_status = &state.solver_status;
        match solver_status {
            SolverStatus::Running { call_id } => interrupt_solver_call(&call_id),
            _ => {}
        }
        drop(state);
        let (tx, rx) = mpsc::channel::<SolverStatus>();
        glib::idle_add_local({
            let self_clone = self.clone();
            move || match rx.try_recv() {
                Ok(solver_status) => {
                    dbg!(&solver_status);
                    let mut state = get_state();
                    state.solver_status = solver_status.clone();
                    drop(state);
                    self_clone.set_solver_status(&solver_status);
                    glib::ControlFlow::Break
                }
                Err(mpsc::TryRecvError::Empty) => glib::ControlFlow::Continue,
                Err(mpsc::TryRecvError::Disconnected) => glib::ControlFlow::Break,
            }
        });

        let call_id = solver::create_solver_call_id();
        let mut state = get_state();
        state.solver_status = SolverStatus::Running {
            call_id: call_id.clone(),
        };
        self.set_solver_status(&state.solver_status);
        solver::solve_for_target(
            &call_id,
            &puzzle_state,
            &target,
            Box::new(move |solver_status| {
                let _ = tx.send(solver_status);
            }),
        );
    }

    fn set_solver_status(&self, status: &SolverStatus) {
        if let Some(window) = self.window.borrow().as_ref() {
            let solver_status_button = window.solver_status();
            match status {
                SolverStatus::Disabled => {
                    solver_status_button.set_tooltip_text(Some("Solver: Disabled"));
                    solver_status_button.set_icon_name("process-stop-symbolic");
                }
                SolverStatus::Running { call_id } => {
                    solver_status_button.set_tooltip_text(Some("Solver: Running..."));
                    solver_status_button.set_icon_name("system-run-symbolic");
                }
                SolverStatus::Done { solvable } => {
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
}
