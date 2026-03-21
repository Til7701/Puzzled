use crate::app::puzzle_area::puzzle_page::PuzzlePage;
use crate::model::extension::PuzzleTypeExtension;
use adw::prelude::{
    AdwDialogExt, AlertDialogExt, AlertDialogExtManual, ComboRowExt, PreferencesGroupExt,
    PreferencesPageExt,
};
use adw::subclass::prelude::ObjectSubclassIsExt;
use adw::{AlertDialog, ComboRow, PreferencesGroup, PreferencesPage, ResponseAppearance};
use gtk::prelude::{ButtonExt, WidgetExt};
use gtk::StringList;
use ndarray::Array2;
use puzzle_config::{AreaConfig, BoardConfig, PuzzleConfig, Target, TargetIndex};

#[derive(Debug, Clone, PartialEq)]
struct TargetIndexListItem {
    display_value: String,
    target_index: TargetIndex,
}

impl PuzzlePage {
    pub fn show_puzzle_extension(&self) {
        let extension = self.imp().extension.borrow();
        match extension.as_ref() {
            None => {
                self.imp().extension_separator.get().set_visible(false);
                self.imp().target_selection_button.set_visible(false);
            }
            Some(PuzzleTypeExtension::Simple) => {
                self.imp().extension_separator.get().set_visible(false);
                self.imp().target_selection_button.set_visible(false);
            }
            Some(PuzzleTypeExtension::Area { .. }) => {
                self.imp().extension_separator.get().set_visible(true);
                self.imp().target_selection_button.set_visible(true);
            }
        }
        self.update_target_selection_button();
    }

    pub(crate) fn show_target_selection_dialog(&self) {
        let dialog = self.create_target_selection_dialog();
        dialog.present(Some(&self.window));
    }

    pub(crate) fn update_target_selection_button(&self) {
        let puzzle = self.imp().puzzle.borrow();
        let puzzle_config = puzzle.as_ref().map(|p| p.config());

        if let Some(puzzle_config) = puzzle_config {
            match self.imp().extension.borrow().as_ref() {
                None => {}
                Some(PuzzleTypeExtension::Simple) => {
                    self.imp().target_selection_button.set_visible(false);
                }
                Some(PuzzleTypeExtension::Area { target }) => {
                    self.imp().target_selection_button.set_visible(true);
                    if let Some(target) = target {
                        let target_string = &*puzzle_config.board_config().format_target(target);
                        self.imp().target_selection_button.set_label(target_string);
                    } else {
                        self.imp()
                            .target_selection_button
                            .set_label("Select Target");
                    }
                }
            }
        }
    }

    pub fn create_target_selection_dialog(&self) -> AlertDialog {
        let dialog = AlertDialog::builder().heading("Select Target").build();

        let content = PreferencesGroup::builder().build();

        let puzzle = self.imp().puzzle.borrow();
        let puzzle = puzzle.as_ref().unwrap();
        let puzzle_config = puzzle.config();
        let extension = self.imp().extension.borrow();
        let current_selection = match extension.as_ref() {
            Some(PuzzleTypeExtension::Area { target }) => target,
            _ => &None,
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
            let (items, dropdown) = Self::create_dropdown_for_area(
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
            let self_clone = self.clone();
            move |_, _| {
                dbg!("Accepted target selection");
                let mut selected_values: Vec<TargetIndex> = Vec::new();
                for (i, dropdown) in dropdowns.iter().enumerate() {
                    let sel = dropdown.selected();
                    if (sel as usize) < area_items[i].len() {
                        selected_values.push(area_items[i][sel as usize].target_index.clone());
                    }
                }
                self_clone.update_extension(&Some(PuzzleTypeExtension::Area {
                    target: Some(Target {
                        indices: selected_values,
                    }),
                }));
            }
        });
        dialog.connect_response(Some(clear_id), {
            let self_clone = self.clone();
            move |_, _| {
                dbg!("Cleared target selection");
                self_clone.update_extension(&None);
            }
        });

        let page = PreferencesPage::builder().build();
        page.add(&content);

        dialog.set_extra_child(Some(&page));
        dialog
    }

    fn create_dropdown_for_area(
        content: &PreferencesGroup,
        puzzle_config: &PuzzleConfig,
        current_selection: &Option<Target>,
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
}
