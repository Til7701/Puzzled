use crate::app::components::polyform_drawing::mode::PrototileDrawingMode;
use crate::global::settings::{Preferences, ShowBoardGridLines};
use adw::gdk::cairo::Context;
use adw::gio;
use adw::glib;
use adw::subclass::prelude::*;
use gtk::Widget;
use gtk::prelude::*;
use puzzle_config::BoardConfig;
use puzzled_common::polyform::Polyform;
use puzzled_common::polyform::grid::{Coord, RegularCoord};
use puzzled_common::polyform::prototile::Square;
use std::ops::Deref;

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

        pub layout: RefCell<Option<Polyform<(Option<String>, PrototileDrawingMode)>>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PuzzledBoardView {
        const NAME: &'static str = "PuzzledBoardView";
        type Type = BoardView;
        type ParentType = gtk::DrawingArea;

        fn class_init(_: &mut Self::Class) {}

        fn instance_init(_: &glib::subclass::InitializingObject<Self>) {}
    }

    #[glib::derived_properties]
    impl ObjectImpl for PuzzledBoardView {}
    impl WidgetImpl for PuzzledBoardView {}
    impl DrawingAreaImpl for PuzzledBoardView {}
}

glib::wrapper! {
    pub struct BoardView(ObjectSubclass<imp::PuzzledBoardView>)
        @extends Widget, gtk::DrawingArea,
         @implements gtk::Buildable, gtk::Accessible, gtk::ConstraintTarget,
                  gtk::Native, gio::ActionGroup, gio::ActionMap;
}

impl BoardView {
    pub fn new(board_config: &BoardConfig) -> Result<BoardView, String> {
        let obj: BoardView = glib::Object::builder().build();

        let layout = match board_config {
            BoardConfig::Simple { layout } => {
                layout.clone()
                    .map(|_| (None, PrototileDrawingMode::Normal))
            }
            BoardConfig::Area { layout, .. } => {
                layout.clone()
                    .map(|value| (Some(value.display_value.clone()), PrototileDrawingMode::Normal))
            }
        };
        obj.imp().layout.replace(Some(layout));

        obj.set_draw_func({
            let self_clone = obj.clone();
            move |_, cr, width, height| self_clone.draw(cr, width, height)
        });

        let preferences = Preferences::default();
        preferences.bind(ShowBoardGridLines, &obj, "show-grid-lines");
        obj.update_grid_lines(preferences.get(ShowBoardGridLines));
        obj.connect_show_grid_lines_notify({
            move |obj| {
                obj.queue_draw();
            }
        });

        Ok(obj)
    }

    fn draw(&self, cr: &Context, width: i32, height: i32) {
        let layout = self.imp().layout.borrow();
        if let Some(l) = layout.deref() {
            match l {
                Polyform::Polyomino { dim, cells } => {
                    self.draw_polyomino(cr, width, height, dim, cells);
                }
                Polyform::Hexomino { .. } => {
                    todo!()
                }
            };
        }
    }

    fn draw_polyomino(
        &self,
        cr: &Context,
        width: i32,
        height: i32,
        dim: &RegularCoord,
        squares: &[Square<(Option<String>, PrototileDrawingMode)>],
    ) {
        let color_map = self.imp().color.borrow();
        for cell in squares.iter() {
            let coord = match cell.coord() {
                Coord::Regular(coord) => coord,
                _ => unreachable!(),
            };
            let x = coord.x();
            let y = coord.y();
            let cell_width = width as f64 / dim.x() as f64;
            let cell_height = height as f64 / dim.y() as f64;
            let cell_x = x as f64 * cell_width;
            let cell_y = y as f64 * cell_height;

            let (display_value, drawing_mode) = cell.data();
            let color = &color_map[drawing_mode];
            cr.set_source_color(color);
            cr.rectangle(cell_x, cell_y, cell_width, cell_height);
            cr.fill().expect("Failed to fill");
            // Due to floating point inaccuracies, there might be 2px gaps between cells, so
            // additional rectangles are drawn to fill those gaps if the adjacent cells are filled.
            // This only solves the problem, if the color is not transparent, otherwise there
            // would be visible lines between the cells of the tile.
            // if color.alpha() == 1.0 {
            //     if current_rotation.get((x + 1, y)).unwrap_or(&false) {
            //         cr.rectangle(cell_x + cell_width - 1.0, cell_y, 2.0, cell_height);
            //         cr.fill().expect("Failed to fill");
            //     }
            //     if current_rotation.get((x, y + 1)).unwrap_or(&false) {
            //         cr.rectangle(cell_x, cell_y + cell_height - 1.0, cell_width, 2.0);
            //         cr.fill().expect("Failed to fill");
            //     }
            // }

            // Border
            let border_color = match drawing_mode {
                PrototileDrawingMode::Normal => None,
                PrototileDrawingMode::Overlapping => None,
                PrototileDrawingMode::OutOfBounds => None,
                PrototileDrawingMode::Highlighted => None,
            };
            if let Some(border_color) = border_color {
                cr.set_source_color(&border_color);
                const BORDER_WIDTH: f64 = 3.0;
                const HALF_BORDER_WIDTH: f64 = BORDER_WIDTH / 2.0;
                cr.set_line_width(BORDER_WIDTH);
                cr.rectangle(
                    cell_x + HALF_BORDER_WIDTH,
                    cell_y + HALF_BORDER_WIDTH,
                    cell_width - BORDER_WIDTH,
                    cell_height - BORDER_WIDTH,
                );
                cr.stroke().expect("Failed to stroke");
            }
        }
    }

    pub fn get_min_element_size(&self) -> u32 {
        20u32
    }

    fn update_grid_lines(&self, show_grid_lines: bool) {
        if show_grid_lines {
            self.add_css_class(SHOW_GRID_LINES_CLASS);
        } else {
            self.remove_css_class(SHOW_GRID_LINES_CLASS);
        }
    }

    pub fn highlight(&self, coord: &Coord) {
        let mut layout = self.imp().layout.borrow_mut();
        if let Some(l) = layout.as_mut() {
            let prototile = l.get_mut(coord);
            if let Some(mut p) = prototile {
                let data = p.data();
                p.set_data((data.0.clone(), PrototileDrawingMode::Highlighted));
            }
        }
    }

    pub fn remove_highlights(&self) {
        let mut layout = self.imp().layout.borrow_mut();
        if let Some(l) = layout.as_mut() {
            let new = l.map(|(value, _)| (value.clone(), PrototileDrawingMode::Normal));
            layout.replace(new);
        }
    }
}
