use crate::app::puzzle_area::puzzle_page::PuzzlePage;
use crate::components::solved_dialog::SolvedDialog;
use adw::prelude::{AdwDialogExt, AlertDialogExt};
use adw::subclass::prelude::ObjectSubclassIsExt;
use log::{debug, error};

impl PuzzlePage {
    pub fn on_solved(&self) {
        let solved_dialog = SolvedDialog::new();
        let extension = self.imp().extension.borrow();
        let puzzle = self.imp().puzzle.borrow();
        let has_next = {
            if let Some(puzzle) = puzzle.as_ref() {
                let stars = puzzle.stars(&*extension);
                solved_dialog.set_stars(&stars);
                puzzle.has_next_puzzle()
            } else {
                false
            }
        };

        if !has_next {
            debug!("No next puzzle available, removing 'Next' button");
            solved_dialog.remove_response("next");
            solved_dialog.set_default_response(Some("back"));
            solved_dialog.set_response_appearance("back", adw::ResponseAppearance::Suggested);
        } else {
            solved_dialog.connect_response(Some("next"), {
                let self_clone = self.clone();
                move |_, _| self_clone.show_next_puzzle()
            });
        }
        solved_dialog.connect_response(Some("back"), { move |_, _| todo!() });
        solved_dialog.present(Some(&self.window));
    }

    fn show_next_puzzle(&self) {
        let puzzle = self.imp().puzzle.borrow();
        let puzzle = match puzzle.as_ref() {
            Some(p) => p,
            None => {
                error!("No current puzzle found when trying to show next puzzle");
                return;
            }
        };
        if let Some(next_puzzle) = puzzle.next_puzzle() {
            self.show_puzzle(&next_puzzle);
        }
    }
}
