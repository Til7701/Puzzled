pub mod board;
pub mod tile;

use crate::global::state::{get_state, get_state_mut, PuzzleTypeExtension, SolverState};
use adw::prelude::{AlertDialogExt, AlertDialogExtManual, PreferencesGroupExt};
use adw::prelude::{ComboRowExt, PreferencesPageExt};
use adw::{AlertDialog, ComboRow, PreferencesGroup, PreferencesPage, ResponseAppearance};
use gtk::StringList;
use ndarray::Array2;
use puzzle_config::{AreaConfig, BoardConfig, PuzzleConfig, Target, TargetIndex};

#[derive(Debug, Clone, PartialEq)]
struct TargetIndexListItem {
    display_value: String,
    target_index: TargetIndex,
}

pub fn create_target_selection_dialog() -> AlertDialog {
    let dialog = AlertDialog::builder().heading("Select Target Day").build();

    let content = PreferencesGroup::builder().build();

    let state = get_state();
    let puzzle_config = &state.puzzle_config.clone().unwrap();
    let current_selection = match &state.puzzle_type_extension {
        Some(PuzzleTypeExtension::Area { target }) => Some(target),
        _ => None,
    };
    let (area_configs, area_count) = match &puzzle_config.board_config() {
        BoardConfig::Area { area_configs, .. } => {
            (area_configs, puzzle_config.board_config().area_count())
        }
        _ => return dialog,
    };
    let mut area_items: Vec<Vec<TargetIndexListItem>> = Vec::new();
    let mut dropdowns: Vec<ComboRow> = Vec::new();
    for area_index in 0..area_count {
        let (items, dropdown) = create_dropdown_for_area(
            &content,
            puzzle_config,
            current_selection,
            &area_configs,
            area_index,
        );
        area_items.push(items);
        dropdowns.push(dropdown);
    }
    let dropdowns = dropdowns;

    let accept_id = "accept";
    let cancel_id = "cancel";
    let clear_id = "clear";
    dialog.add_responses(
        vec![
            (accept_id, "Accept"),
            (cancel_id, "Cancel"),
            (clear_id, "Clear"),
        ]
        .as_ref(),
    );
    dialog.set_default_response(Some(accept_id));
    dialog.set_close_response(cancel_id);
    dialog.set_response_appearance(accept_id, ResponseAppearance::Suggested);
    dialog.set_response_appearance(clear_id, ResponseAppearance::Destructive);
    dialog.set_prefer_wide_layout(true);
    dialog.connect_response(Some(accept_id), {
        let dropdowns = dropdowns.clone();
        let area_items = area_items.clone();
        move |_, _| {
            dbg!("Accepted target selection");
            let mut selected_values: Vec<TargetIndex> = Vec::new();
            for (i, dropdown) in dropdowns.iter().enumerate() {
                let sel = dropdown.selected();
                if (sel as usize) < area_items[i].len() {
                    selected_values.push(area_items[i][sel as usize].target_index.clone());
                }
            }
            let mut state = get_state_mut();
            if let Some(PuzzleTypeExtension::Area { target }) = &mut state.puzzle_type_extension {
                target.indices = selected_values.clone();
            }
            drop(state);
        }
    });
    dialog.connect_response(Some(clear_id), {
        move |_, _| {
            dbg!("Cleared target selection");
            let mut state = get_state_mut();
            if let Some(PuzzleTypeExtension::Area { .. }) = state.puzzle_type_extension {
                state.puzzle_type_extension = None;
            }
            state.solver_state = SolverState::Initial;
            drop(state);
        }
    });

    let page = PreferencesPage::builder()
        .title("Select Target Day")
        .build();
    page.add(&content);

    drop(state);
    dialog.set_extra_child(Some(&page));
    dialog
}

fn create_dropdown_for_area(
    content: &PreferencesGroup,
    puzzle_config: &PuzzleConfig,
    current_selection: Option<&Target>,
    area_configs: &&Vec<AreaConfig>,
    area_index: usize,
) -> (Vec<TargetIndexListItem>, ComboRow) {
    let mut items: Vec<TargetIndexListItem> = puzzle_config
        .board_config()
        .get_display_values_for_area(area_index as i32)
        .iter()
        .map(|(display_value, target_index)| TargetIndexListItem {
            display_value: display_value.clone(),
            target_index: target_index.clone(),
        })
        .collect();
    let value_order: &Array2<i32> = match puzzle_config.board_config() {
        BoardConfig::Simple { .. } => {
            return (Vec::new(), ComboRow::builder().build());
        }
        BoardConfig::Area { value_order, .. } => value_order,
    };
    items.sort_by(|a, b| {
        let first = value_order
            .get((a.target_index.0, a.target_index.1))
            .cloned()
            .unwrap_or(i32::MAX);
        let second = value_order
            .get((b.target_index.0, b.target_index.1))
            .cloned()
            .unwrap_or(i32::MAX);

        first.cmp(&second)
    });

    let string_list = StringList::new(&[]);
    for it in &items {
        string_list.append(&it.display_value);
    }

    let dropdown = ComboRow::builder()
        .title(area_configs[area_index].name())
        .model(&string_list)
        .build();

    if let Some(idx) = current_selection
        .as_ref()
        .and_then(|sel| sel.indices.get(area_index))
        .and_then(|target_index| items.iter().position(|i| i.target_index == *target_index))
    {
        dropdown.set_selected(idx as u32);
    }

    content.add(&dropdown);
    (items, dropdown)
}

pub fn create_solved_dialog() -> AlertDialog {
    let dialog = AlertDialog::builder().heading("Puzzle Solved!").build();

    let ok_id = "ok";
    dialog.add_response(ok_id, "OK");
    dialog.set_default_response(Some(ok_id));
    dialog.set_close_response(ok_id);
    dialog.set_response_appearance(ok_id, ResponseAppearance::Suggested);

    dialog
}
