use adw::prelude::Cast;
use gtk::prelude::{FrameExt, GridExt, WidgetExt};
use gtk::{Frame, Grid, Label, Widget};
use puzzle_config::BoardConfig;

#[derive(Debug, Clone)]
pub struct BoardView {
    pub parent: Grid,
    pub elements: Vec<Widget>,
}

impl BoardView {
    pub fn new(board_config: &BoardConfig) -> Result<BoardView, String> {
        let board_layout = &board_config.layout();

        let grid = Grid::builder()
            .css_classes(vec!["board-grid".to_string()])
            .build();
        grid.set_row_homogeneous(true);
        grid.set_column_homogeneous(true);

        let mut elements: Vec<Widget> = Vec::new();

        for ((x, y), value) in board_layout.indexed_iter() {
            let cell = if *value {
                match board_config {
                    BoardConfig::Simple { .. } => {
                        let css_classes: Vec<String> =
                            vec!["board-cell".to_string(), "board-cell-simple".to_string()];
                        let cell = Frame::builder().css_classes(css_classes).build();

                        cell
                    }
                    BoardConfig::Area {
                        area_indices,
                        display_values,
                        ..
                    } => {
                        let css_classes: Vec<String> = vec![
                            "board-cell".to_string(),
                            format!("board-cell-{}", area_indices[[x, y]]),
                        ];
                        let cell = Frame::builder().css_classes(css_classes).build();

                        let label = Label::new(Some(&display_values[[x, y]]));
                        cell.set_child(Some(&label));
                        cell
                    }
                }
            } else {
                let css_classes: Vec<String> =
                    vec!["board-cell".to_string(), "board-cell-outside".to_string()];
                Frame::builder().css_classes(css_classes).build()
            };
            grid.attach(&cell, x as i32, y as i32, 1, 1);
            elements.push(cell.upcast::<Widget>());
        }

        Ok(BoardView {
            parent: grid,
            elements,
        })
    }

    pub fn get_min_element_size(&self) -> i32 {
        let max_elements_width = self
            .elements
            .iter()
            .map(|w| {
                if let Ok(frame) = w.clone().downcast::<Frame>()
                    && let Some(child) = frame.child()
                    && let Ok(label) = child.downcast::<Label>()
                {
                    let size = label
                        .layout()
                        .pixel_size()
                        .0
                        .max(label.layout().pixel_size().1);
                    (size as f64 * 1.4) as i32
                } else {
                    0
                }
            })
            .chain(0..1)
            .max()
            .unwrap_or(0);
        max_elements_width
    }
}
