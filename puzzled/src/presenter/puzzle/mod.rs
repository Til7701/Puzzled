mod extension;
mod hint;
mod info;

use crate::application::PuzzledApplication;
use crate::global::puzzle_meta::PuzzleMeta;
use crate::global::state::{get_state, get_state_mut, SolverState};
use crate::presenter::puzzle::extension::ExtensionPresenter;
use crate::presenter::puzzle::hint::{HintButtonPresenter, HintButtonState};
use crate::presenter::puzzle::info::PuzzleInfoPresenter;
use crate::presenter::puzzle_area::PuzzleAreaPresenter;
use crate::solver;
use crate::solver::combination_solutions::CombinationsSolver;
use crate::solver::{interrupt_solver_call, is_solved};
use crate::view::puzzle_area_page::PuzzleAreaPage;
use crate::window::PuzzledWindow;
use adw::prelude::{ActionMapExtManual, Cast, NavigationPageExt};
use adw::{gio, Toast, ToastOverlay};
use gtk::prelude::{BoxExt, GtkApplicationExt};
use gtk::{Image, Label, Widget};
use log::{debug, error};
use puzzle_solver::result::UnsolvableReason;
use std::cell::Cell;
use std::rc::Rc;

#[derive(Clone)]
pub struct PuzzlePresenter {
    puzzle_area_nav_page: PuzzleAreaPage,
    toast_overlay: ToastOverlay,
    puzzle_info_presenter: PuzzleInfoPresenter,
    puzzle_area_presenter: PuzzleAreaPresenter,
    hint_button_presenter: HintButtonPresenter,
    extension_presenter: ExtensionPresenter,
    puzzle_meta: PuzzleMeta,
    puzzle_solved_callback: Option<Rc<dyn Fn()>>,
    hint_count: Rc<Cell<u32>>,
    combinations_solver: CombinationsSolver,
}

impl PuzzlePresenter {
    pub fn new(window: &PuzzledWindow) -> Self {
        let puzzle_info_presenter = PuzzleInfoPresenter::new(window);
        let puzzle_area_presenter = PuzzleAreaPresenter::new(window);
        let hint_button_presenter = HintButtonPresenter::new(window);
        let extension_presenter = ExtensionPresenter::new(window);

        PuzzlePresenter {
            puzzle_area_nav_page: window.puzzle_area_nav_page(),
            toast_overlay: window.puzzle_area_nav_page().toast_overlay(),
            puzzle_info_presenter,
            puzzle_area_presenter,
            hint_button_presenter,
            extension_presenter,
            puzzle_meta: PuzzleMeta::new(),
            puzzle_solved_callback: None,
            hint_count: Rc::new(Cell::new(0)),
            combinations_solver: CombinationsSolver::default(),
        }
    }

    pub fn register_actions(&self, app: &PuzzledApplication) {
        self.puzzle_info_presenter.register_actions(app);
        self.hint_button_presenter.register_actions(app);
        self.extension_presenter.register_actions(app);

        let solver_state_action = gio::ActionEntry::builder("hint")
            .activate({
                let self_clone = self.clone();
                move |_, _, _| self_clone.on_hint_requested()
            })
            .build();
        let calculate_tile_combinations_to_solve_action =
            gio::ActionEntry::builder("calculate-tile-combinations-to-solve")
                .activate({
                    let self_clone = self.clone();
                    move |_, _, _| {
                        debug!("Calculating tile combinations to solve...");
                        self_clone.calculate_tile_combinations_to_solve();
                    }
                })
                .build();
        let stop_calculate_tile_combinations_to_solve_action =
            gio::ActionEntry::builder("stop-calculate-tile-combinations-to-solve")
                .activate({
                    let self_clone = self.clone();
                    move |_, _, _| {
                        debug!("Stopping calculation of tile combinations to solve...");
                        self_clone.stop_calculate_tile_combinations_to_solve();
                    }
                })
                .build();

        app.add_action_entries([
            solver_state_action,
            calculate_tile_combinations_to_solve_action,
            stop_calculate_tile_combinations_to_solve_action,
        ]);

        app.set_accels_for_action("app.calculate-tile-combinations-to-solve", &["<control>k"]);
        app.set_accels_for_action(
            "app.stop-calculate-tile-combinations-to-solve",
            &["<control>l"],
        );
    }

    pub fn setup(&mut self, puzzle_solved_callback: Rc<dyn Fn()>) {
        self.puzzle_info_presenter.setup();
        self.puzzle_area_presenter.setup();
        self.hint_button_presenter.setup();
        self.extension_presenter.setup();
        self.puzzle_solved_callback = Some(puzzle_solved_callback);
    }

    pub fn show_puzzle(&self) {
        self.puzzle_area_presenter.show_puzzle(Rc::new({
            let self_clone = self.clone();
            move || self_clone.on_tile_moved()
        }));
        self.extension_presenter.show_puzzle(Rc::new({
            let self_clone = self.clone();
            move || {
                self_clone.puzzle_area_presenter.update_layout();
                self_clone.puzzle_area_presenter.update_highlights();
            }
        }));
        self.on_tile_moved();
        let state = get_state();
        if let Some(collection) = &state.puzzle_collection
            && let Some(puzzle_config) = &state.puzzle_config
        {
            let title = format!("{} - {}", collection.name(), puzzle_config.name());
            self.puzzle_area_nav_page.set_title(&title);
        }
        self.hint_count.replace(0);
    }

    fn on_tile_moved(&self) {
        let puzzle_state = self.puzzle_area_presenter.extract_puzzle_state();

        if let Ok(puzzle_state) = puzzle_state {
            let mut state = get_state_mut();
            interrupt_solver_call(&state);
            self.stop_calculate_tile_combinations_to_solve();
            state.solver_state = SolverState::Done;
            self.hint_button_presenter
                .display_state(&HintButtonState::Bulb);
            drop(state);
            if is_solved(&puzzle_state) {
                self.handle_solved();
            }
        }
    }

    fn on_hint_requested(&self) {
        let puzzle_state = self.puzzle_area_presenter.extract_puzzle_state();

        if let Ok(mut puzzle_state) = puzzle_state {
            self.puzzle_area_presenter.remove_hint_tile();
            self.hint_button_presenter
                .calculate_hint(&mut puzzle_state, {
                    let self_clone = self.clone();
                    Box::new(move |result| {
                        self_clone.toast_overlay.dismiss_all();
                        let hint_count = self_clone.hint_count.get();
                        self_clone.hint_count.replace(hint_count + 1);
                        match result {
                            Ok(solution) => {
                                if let Some(placement) = solution.placements().last() {
                                    self_clone.puzzle_area_presenter.show_hint_tile(placement)
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

        self.toast_overlay
            .add_toast(Toast::builder().custom_title(&content).build());
    }

    fn handle_solved(&self) {
        let state = get_state();
        if let Some(collection) = &state.puzzle_collection
            && let Some(puzzle_config) = &state.puzzle_config
        {
            self.puzzle_meta.set_solved(
                true,
                collection,
                puzzle_config.index(),
                &state.puzzle_type_extension,
            );
            let hint_count = self.hint_count.get();
            let previous_hint_count = self
                .puzzle_meta
                .hints(
                    collection,
                    puzzle_config.index(),
                    &state.puzzle_type_extension,
                )
                .unwrap_or(u32::MAX);
            if hint_count < previous_hint_count {
                self.puzzle_meta.set_hints(
                    hint_count,
                    collection,
                    puzzle_config.index(),
                    &state.puzzle_type_extension,
                )
            }
        } else {
            error!("Could not mark puzzle as solved: missing puzzle collection or puzzle config");
        }

        drop(state);
        if let Some(callback) = &self.puzzle_solved_callback {
            callback();
        }
    }

    fn calculate_tile_combinations_to_solve<'a>(&self) {
        let state = get_state();
        interrupt_solver_call(&state);
        drop(state);

        let puzzle_state = self.puzzle_area_presenter.extract_puzzle_state();
        if let Ok(puzzle_state) = puzzle_state {
            self.combinations_solver
                .calculate_tile_combinations_to_solve(puzzle_state)
        }
    }

    fn stop_calculate_tile_combinations_to_solve(&self) {
        self.combinations_solver
            .stop_calculate_tile_combinations_to_solve();
    }
}
