use crate::app::puzzle::puzzle_area::puzzle_state::PuzzleState;
use crate::app::puzzle::puzzle_page::PuzzlePage;
use crate::model::extension::PuzzleTypeExtension;
use crate::solver::Solver;
use adw::prelude::Cast;
use adw::subclass::prelude::ObjectSubclassIsExt;
use adw::{glib, Toast};
use gtk::prelude::{BoxExt, ButtonExt, WidgetExt};
use gtk::{Image, Label, Widget};
use puzzle_solver::result::{Solution, UnsolvableReason};
use std::sync::mpsc;
use tokio_util::sync::CancellationToken;

pub type OnComplete = Box<dyn Fn(Result<Solution, UnsolvableReason>)>;

impl PuzzlePage {
    pub fn on_hint_requested(&self) {
        let puzzle_state = self.imp().grid.extract_puzzle_state();

        if let Ok(puzzle_state) = puzzle_state {
            self.imp().grid.remove_hint_tile();
            self.calculate_hint(&puzzle_state, {
                let self_clone = self.clone();
                Box::new(move |result| {
                    self_clone.imp().toast_overlay.dismiss_all();
                    let hint_count = self_clone.imp().hint_count.get();
                    self_clone.imp().hint_count.replace(hint_count + 1);
                    match result {
                        Ok(solution) => {
                            if let Some(placement) = solution.placements().last() {
                                self_clone.imp().grid.show_hint_tile(placement)
                            }
                        }
                        Err(unsolvable_reason) => {
                            self_clone.show_unsolvable_toast(unsolvable_reason);
                        }
                    }
                })
            });
        }
    }

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
    fn calculate_hint(&self, puzzle_state: &PuzzleState, on_complete: OnComplete) {
        let extension = self.imp().extension.borrow();
        let calculate_solvability = match extension.as_ref() {
            None => true,
            Some(PuzzleTypeExtension::Simple) => true,
            Some(PuzzleTypeExtension::Area { target, .. }) => target.is_some(),
        };
        drop(extension);
        if calculate_solvability {
            self.calculate_solvability(puzzle_state, on_complete);
        } else {
            self.display_state(&HintButtonState::Bulb);
        }
    }

    fn calculate_solvability(&self, puzzle_state: &PuzzleState, on_complete: OnComplete) {
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

        let cancel_token = CancellationToken::new();
        let solver = Solver::default();
        self.display_state(&HintButtonState::Calculating);
        solver.interrupt_solver_call();
        solver.solve_for_target(
            puzzle_state,
            Box::new(move |result| {
                let _ = tx.send(result);
            }),
            cancel_token,
        );
    }

    fn display_state(&self, status: &HintButtonState) {
        match status {
            HintButtonState::Bulb => {
                self.imp().hint_button.set_tooltip_text(Some("Hint"));
                self.imp().hint_button.set_icon_name("lightbulb-symbolic");
            }
            HintButtonState::Calculating => {
                self.imp()
                    .hint_button
                    .set_tooltip_text(Some("Hint: Calculating..."));
                self.imp().hint_button.set_icon_name("timer-sand-symbolic");
            }
        }
    }

    fn show_unsolvable_toast(&self, unsolvable_reason: UnsolvableReason) {
        fn build_label(content: &str) -> Widget {
            Label::builder().label(content).build().upcast()
        }

        let icon = Image::builder()
            .icon_name("cross-large-circle-outline-symbolic")
            .css_classes(vec!["error"])
            .build()
            .upcast();

        let widgets: Vec<Widget> = match unsolvable_reason {
            UnsolvableReason::NoFit => {
                vec![
                    icon,
                    build_label("The remaining tiles do not fit on the board!"),
                ]
            }
            UnsolvableReason::BoardTooLarge => {
                vec![
                    icon,
                    build_label("The board of this puzzle is too large for the solver!"),
                ]
            }
            UnsolvableReason::TileCannotBePlaced { .. } => {
                vec![
                    icon,
                    build_label(
                        "At least one of the remaining tiles does not fit in the remaining space!",
                    ),
                ]
            }
            UnsolvableReason::PlausibilityCheckFailed => {
                vec![
                    icon,
                    build_label("Some tiles are overlapping or are out of bounds!"),
                ]
            }
            UnsolvableReason::Cancelled => {
                return;
            }
        };

        let content = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(5)
            .build();
        for widget in widgets {
            content.append(&widget);
        }

        self.imp()
            .toast_overlay
            .add_toast(Toast::builder().custom_title(&content).build());
    }
}

enum HintButtonState {
    Bulb,
    Calculating,
}
