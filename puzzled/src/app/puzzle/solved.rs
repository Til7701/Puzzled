use crate::app::puzzle::puzzle_page::PuzzlePage;
use crate::components::solved_dialog::SolvedDialog;
use adw::prelude::{AdwDialogExt, AlertDialogExt};
use adw::subclass::prelude::ObjectSubclassIsExt;
use gtk::prelude::WidgetExt;
use log::{debug, error};

impl PuzzlePage {
    fn handle_solved(&self) {
        let puzzle = self.imp().puzzle.borrow();
        if let Some(puzzle) = puzzle.as_ref() {
            let hint_count = self.imp().hint_count.get();
            let previous_hint_count = puzzle.best_hint_count(&*self.imp().extension.borrow());
            let best_hint_count = hint_count.min(previous_hint_count.unwrap_or(u32::MAX));

            puzzle.set_solved(best_hint_count, &*self.imp().extension.borrow());
        } else {
            error!("Could not mark puzzle as solved: missing puzzle collection or puzzle config");
        }
    }

    pub fn on_solved(&self) {
        self.handle_solved();
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
        solved_dialog.connect_response(Some("back"), {
            let self_clone = self.clone();
            move |_, _| {
                self_clone
                    .imp()
                    .window
                    .get()
                    .unwrap()
                    .outer_view()
                    .get()
                    .set_show_content(false);
            }
        });
        solved_dialog.present(self.imp().window.get());
    }

    fn show_next_puzzle(&self) {
        let opt_puzzle = self.imp().puzzle.borrow();
        let puzzle = match opt_puzzle.as_ref() {
            Some(p) => p,
            None => {
                error!("No current puzzle found when trying to show next puzzle");
                return;
            }
        };
        if let Some(next_puzzle) = puzzle.next_puzzle() {
            let next_puzzle = next_puzzle.clone();
            drop(opt_puzzle);
            self.show_puzzle(&next_puzzle);
        }
    }
}
