use crate::application::PuzzledApplication;
use crate::global::state::{get_state, PuzzleTypeExtension};
use crate::view::create_target_selection_dialog;
use crate::window::PuzzledWindow;
use adw::gio;
use adw::prelude::{ActionMapExtManual, AdwDialogExt, AlertDialogExt};
use gtk::prelude::{ButtonExt, WidgetExt};
use gtk::{Button, Separator};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct ExtensionPresenter {
    window: PuzzledWindow,
    separator: Separator,
    target_selection_button: Button,
    target_changed_callback: Rc<RefCell<Option<Rc<dyn Fn()>>>>,
}

impl ExtensionPresenter {
    pub fn new(window: &PuzzledWindow) -> Self {
        ExtensionPresenter {
            window: window.clone(),
            separator: window.extension_separator(),
            target_selection_button: window.target_selection_button(),
            target_changed_callback: Rc::new(RefCell::new(None)),
        }
    }

    pub fn register_actions(&self, app: &PuzzledApplication) {
        let solver_state_action = gio::ActionEntry::builder("select_target")
            .activate({
                let self_clone = self.clone();
                move |_, _, _| self_clone.show_target_selection_dialog()
            })
            .build();
        app.add_action_entries([solver_state_action]);
    }

    pub fn setup(&self) {}

    pub fn show_puzzle(&self, on_changed: Rc<dyn Fn()>) {
        self.target_changed_callback.replace(Some(on_changed));
        let state = get_state();
        match &state.puzzle_type_extension {
            None => {
                self.separator.set_visible(false);
                self.target_selection_button.set_visible(false);
            }
            Some(PuzzleTypeExtension::Simple) => {
                self.separator.set_visible(false);
                self.target_selection_button.set_visible(false);
            }
            Some(PuzzleTypeExtension::Area { .. }) => {
                self.separator.set_visible(true);
                self.target_selection_button.set_visible(true);
            }
        }
        self.update_target_selection_button();
    }

    fn show_target_selection_dialog(&self) {
        let dialog = create_target_selection_dialog();
        dialog.connect_response(None, {
            let self_clone = self.clone();
            move |_, _| {
                self_clone.update_target_selection_button();
                if let Some(callback) = &*self_clone.target_changed_callback.borrow() {
                    callback();
                }
            }
        });
        dialog.present(Some(&self.window));
    }

    fn update_target_selection_button(&self) {
        let state = get_state();
        let puzzle_config = &state.puzzle_config;

        if let Some(puzzle_config) = puzzle_config {
            match &state.puzzle_type_extension {
                None => {}
                Some(PuzzleTypeExtension::Simple) => {
                    self.target_selection_button.set_visible(false);
                }
                Some(PuzzleTypeExtension::Area { target }) => {
                    self.target_selection_button.set_visible(true);
                    if let Some(target) = target {
                        let target_string = &*puzzle_config.board_config().format_target(target);
                        self.target_selection_button.set_label(target_string);
                    } else {
                        self.target_selection_button.set_label("Select Target");
                    }
                }
            }
        }
    }
}
