use crate::global::state::get_state;
use crate::presenter::puzzle_area::PuzzleAreaPresenter;
use crate::view::create_target_selection_dialog;
use crate::window::PuzzlemoredaysWindow;
use adw::prelude::{AdwDialogExt, AlertDialogExt};
use gtk::prelude::{ButtonExt, WidgetExt};
use puzzle_config::BoardConfig;
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;
use std::sync::Arc;

#[derive(Debug, Default, Clone)]
pub struct MainPresenter {
    window: Arc<RefCell<Option<PuzzlemoredaysWindow>>>,
}

impl MainPresenter {
    pub fn setup(
        &self,
        window: &PuzzlemoredaysWindow,
        puzzle_area_presenter: Rc<RefCell<PuzzleAreaPresenter>>,
    ) {
        self.setup_puzzle_type();
        self.window.replace(Some(window.clone()));

        window.target_selection_button().connect_clicked({
            let self_clone = self.clone();
            move |_| {
                if let Some(window) = self_clone.window.borrow().as_ref() {
                    let dialog = create_target_selection_dialog();
                    dialog.connect_response(None, {
                        let self_clone = self_clone.clone();
                        move |_, _| {
                            self_clone.update_layout();
                            // self_clone.calculate_solvability_if_enabled();
                            // self_clone.set_solver_status(&get_state().solver_state);
                        }
                    });
                    dialog.present(Some(window));
                }
            }
        });
    }

    pub fn update_layout(&self) {
        self.setup_puzzle_type();
    }

    fn setup_puzzle_type(&self) {
        self.update_target_selection_button();
        let state = get_state();
        let puzzle_config = &state.puzzle_config.clone().unwrap();
        let board_config = puzzle_config.board_config();
        match board_config {
            BoardConfig::Simple { .. } => {
                if let Some(window) = self.window.borrow().as_ref() {
                    window.target_selection_button().set_visible(false);
                    window.solver_state().set_visible(false);
                }
            }
            BoardConfig::Area { .. } => {
                if let Some(window) = self.window.borrow().as_ref() {
                    window.target_selection_button().set_visible(true);
                    window.solver_state().set_visible(true);
                }
            }
        }
    }

    fn update_target_selection_button(&self) {
        if let Some(window) = self.window.borrow().as_ref() {
            let state = get_state();
            let target_selection = &state.target_selection;
            match target_selection {
                Some(target) => {
                    let puzzle_config = &state.puzzle_config.clone().unwrap();
                    let board_config = puzzle_config.board_config();
                    let text = match board_config {
                        BoardConfig::Simple { .. } => "",
                        BoardConfig::Area { .. } => &*board_config.format_target(target),
                    };
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
}
