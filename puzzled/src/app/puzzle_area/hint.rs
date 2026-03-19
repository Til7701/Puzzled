use crate::app::puzzle_area::puzzle_area::puzzle_state::PuzzleState;
use crate::app::puzzle_area::puzzle_area_page::PuzzleAreaPage;
use crate::model::extension::PuzzleTypeExtension;
use crate::solver::Solver;
use adw::glib;
use adw::subclass::prelude::ObjectSubclassIsExt;
use gtk::prelude::{ButtonExt, WidgetExt};
use puzzle_solver::result::{Solution, UnsolvableReason};
use std::sync::mpsc;
use tokio_util::sync::CancellationToken;

pub type OnComplete = Box<dyn Fn(Result<Solution, UnsolvableReason>)>;

impl PuzzleAreaPage {
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
    pub(self) fn calculate_hint(&self, puzzle_state: &mut PuzzleState, on_complete: OnComplete) {
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

    fn calculate_solvability(&self, puzzle_state: &mut PuzzleState, on_complete: OnComplete) {
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
        solver.solve_for_target(
            puzzle_state,
            Box::new(move |result| {
                let _ = tx.send(result);
            }),
            cancel_token,
        );
    }

    pub fn display_state(&self, status: &HintButtonState) {
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
}

enum HintButtonState {
    Bulb,
    Calculating,
}
