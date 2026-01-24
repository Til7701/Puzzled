use adw::prelude::Cast;
use gtk::prelude::{FrameExt, GridExt};
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
            if *value {
                let cell = match board_config {
                    BoardConfig::Simple { .. } => {
                        let css_classes: Vec<String> =
                            vec!["board-cell".to_string(), "board-cell-simple".to_string()];
                        let cell = Frame::builder().css_classes(css_classes).build();

                        let label = Label::new(Some(format!("({}, {})", x, y).as_str()));
                        cell.set_child(Some(&label));
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
                };

                grid.attach(&cell, x as i32, y as i32, 1, 1);
                elements.push(cell.upcast::<Widget>());
            }
        }

        Ok(BoardView {
            parent: grid,
            elements,
        })
    }
}
