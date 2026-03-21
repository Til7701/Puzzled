use crate::app::puzzle_area::puzzle_page::PuzzlePage;
use crate::solver::Solver;
use adw::subclass::prelude::ObjectSubclassIsExt;

impl PuzzlePage {
    pub fn calculate_tile_combinations_to_solve<'a>(&self) {
        let solver = Solver::default();
        solver.interrupt_solver_call();

        let puzzle_state = self.imp().grid.extract_puzzle_state();
        if let Ok(puzzle_state) = puzzle_state {
            self.imp()
                .combinations_solver
                .borrow()
                .calculate_tile_combinations_to_solve(puzzle_state)
        }
    }

    pub fn stop_calculate_tile_combinations_to_solve(&self) {
        self.imp()
            .combinations_solver
            .borrow()
            .stop_calculate_tile_combinations_to_solve();
    }
}
