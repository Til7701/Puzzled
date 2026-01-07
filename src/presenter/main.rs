use crate::presenter::puzzle_area::PuzzleAreaPresenter;
use crate::puzzle;
use crate::state::get_state;
use crate::view::{create_puzzle_info, create_target_selection_dialog};
use crate::window::PuzzlemoredaysWindow;
use adw::prelude::AdwDialogExt;
use gtk::prelude::ButtonExt;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Default, Clone)]
pub struct MainPresenter {
    puzzle_area_presenter: Rc<RefCell<PuzzleAreaPresenter>>,
    window: Rc<RefCell<Option<PuzzlemoredaysWindow>>>,
}

impl MainPresenter {
    pub fn set_puzzle_area_presenter(&self, presenter: &PuzzleAreaPresenter) {
        self.puzzle_area_presenter.replace(presenter.clone());
    }

    pub fn setup(&self, window: &PuzzlemoredaysWindow) {
        self.window.replace(Some(window.clone()));
        let puzzle_area_presenter = self.puzzle_area_presenter.borrow();
        puzzle_area_presenter.set_view(window.grid());
        puzzle_area_presenter.setup_puzzle_config_from_state();

        let puzzle_selection = window.puzzle_selection();
        puzzle_selection.set_selected(0);

        puzzle_selection.connect_selected_notify({
            let puzzle_area_presenter = puzzle_area_presenter.clone();
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

                puzzle_area_presenter.setup_puzzle_config_from_state();
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
                    dialog.connect_closed({
                        let self_clone = self_clone.clone();
                        move |_| {
                            self_clone.update_layout();
                            self_clone
                                .puzzle_area_presenter
                                .borrow()
                                .update_highlights();
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
}
