use crate::puzzle::config::Target;
use crate::puzzle_state::PuzzleState;
use crate::state::SolverStatus;
use std::thread;
use std::time::Duration;

mod bitmask;
mod tile;

#[derive(Debug, Clone)]
pub struct SolverCallId(u64);

pub type OnCompleteCallback = Box<dyn FnOnce(SolverStatus) + Send>;

pub fn create_solver_call_id() -> SolverCallId {
    SolverCallId(0)
}

pub fn solve_for_target(
    solver_call_id: &SolverCallId,
    puzzle_state: &PuzzleState,
    target: &Target,
    on_complete: OnCompleteCallback,
) {
    call_on_complete_after_random_delay_with_random_value(on_complete);
}

pub fn interrupt_solver_call(call_id: &SolverCallId) {
    drop(call_id);
}

fn call_on_complete_after_random_delay_with_random_value(on_complete: OnCompleteCallback) {
    let solvable = rand::random();
    let delay_ms = rand::random_range(100..3000);

    thread::spawn(move || {
        thread::sleep(Duration::from_millis(delay_ms));
        on_complete(SolverStatus::Done { solvable });
    });
}
