use crate::global::settings::{Preferences, ShowBoardGridLines};
use adw::gio;
use adw::glib;
use adw::prelude::Cast;
use adw::subclass::prelude::*;
use gtk::prelude::*;
use gtk::{Frame, Label, Widget};
use puzzle_config::BoardConfig;
use std::cell::Ref;

const SHOW_GRID_LINES_CLASS: &str = "show-grid-lines";

mod imp {
    use super::*;
    use adw::glib::Properties;
    use std::cell::RefCell;

    #[derive(Debug, Default, Properties)]
    #[properties(wrapper_type = super::BoardView)]
    pub struct PuzzledBoardView {
        #[property(name = "show-grid-lines", get, set)]
        pub show_grid_lines: RefCell<bool>,

        pub elements: RefCell<Vec<Widget>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PuzzledBoardView {
        const NAME: &'static str = "PuzzledBoardView";
        type Type = BoardView;
        type ParentType = gtk::Grid;

        fn class_init(_: &mut Self::Class) {}

        fn instance_init(_: &glib::subclass::InitializingObject<Self>) {}
    }

    #[glib::derived_properties]
    impl ObjectImpl for PuzzledBoardView {}
    impl WidgetImpl for PuzzledBoardView {}
    impl GridImpl for PuzzledBoardView {}
}

glib::wrapper! {
    pub struct BoardView(ObjectSubclass<imp::PuzzledBoardView>)
        @extends Widget, gtk::Grid,
         @implements gtk::Buildable, gtk::Accessible, gtk::ConstraintTarget,
                  gtk::Native, gio::ActionGroup, gio::ActionMap, gtk::Orientable;
}

impl BoardView {
    pub fn new(board_config: &BoardConfig) -> Result<BoardView, String> {
        let obj: BoardView = glib::Object::builder().build();

        let board_layout = &board_config.layout();

        obj.add_css_class("board-grid");
        obj.set_row_homogeneous(true);
        obj.set_column_homogeneous(true);

        let mut elements: Vec<Widget> = Vec::new();

        for ((x, y), value) in board_layout.indexed_iter() {
            let cell = if *value {
                match board_config {
                    BoardConfig::Simple { .. } => {
                        let css_classes: Vec<String> =
                            vec!["board-cell".to_string(), "board-cell-simple".to_string()];

                        Frame::builder().css_classes(css_classes).build()
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
            obj.attach(&cell, x as i32, y as i32, 1, 1);
            elements.push(cell.upcast::<Widget>());
        }
        obj.imp().elements.replace(elements);

        let preferences = Preferences::default();
        preferences.bind(ShowBoardGridLines, &obj, "show-grid-lines");
        obj.connect_show_grid_lines_notify({
            let preferences = preferences.clone();
            move |obj| {
                let show_grid_lines = preferences.get(ShowBoardGridLines);
                obj.update_grid_lines(show_grid_lines);
            }
        });
        obj.update_grid_lines(preferences.get(ShowBoardGridLines));

        Ok(obj)
    }

    pub fn elements(&self) -> Ref<'_, Vec<Widget>> {
        self.imp().elements.borrow()
    }

    pub fn get_min_element_size(&self) -> i32 {
        self.elements()
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
            .unwrap_or(0)
    }

    fn update_grid_lines(&self, show_grid_lines: bool) {
        if show_grid_lines {
            self.add_css_class(SHOW_GRID_LINES_CLASS);
        } else {
            self.remove_css_class(SHOW_GRID_LINES_CLASS);
        }
    }
}
