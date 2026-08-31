use crate::adw_ext;
use adw::gdk::RGBA;
use adw::gio;
use adw::glib;
use adw::prelude::GdkCairoContextExt;
use adw::subclass::prelude::*;
use gtk::cairo::Context;
use gtk::prelude::{DrawingAreaExtManual, WidgetExt};
use puzzle_config::ColorConfig;
use puzzled_common::polyform::Polyform;
use puzzled_common::polyform::grid::{Coord, RegularCoord};
use puzzled_common::polyform::prototile::Square;
use std::collections::HashMap;
use std::ops::Deref;

const HIGHLIGHT_OVERLAPPING_COLOR: RGBA = adw_ext::ERROR_BG_LIGHT;
const HIGHLIGHT_OUT_OF_BOUNDS_COLOR: RGBA = adw_ext::WARNING_BG_LIGHT;

/// Defines how a cell of a tile should be drawn, based on its state in the puzzle area.
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq)]
pub enum DrawingMode {
    /// Draw normally
    #[default]
    Normal,
    /// Draw with a highlight indicating that this cell overlaps with another tile
    Overlapping,
    /// Draw with a highlight indicating that this cell is out of bounds of the board
    OutOfBounds,
}

mod imp {
    use super::*;
    use puzzled_common::polyform::grid::{Coord, RegularCoord};
    use std::cell::{Cell, RefCell};
    use std::collections::HashMap;
    use std::ops::Deref;

    #[derive(Debug, Default)]
    pub struct PuzzledTileView {
        pub id: Cell<usize>,
        pub current_rotation: RefCell<Polyform<DrawingMode>>,
        pub color: RefCell<HashMap<DrawingMode, RGBA>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PuzzledTileView {
        const NAME: &'static str = "PuzzledTileView";
        type Type = TileView;
        type ParentType = gtk::DrawingArea;

        fn class_init(_: &mut Self::Class) {}

        fn instance_init(_: &glib::subclass::InitializingObject<Self>) {}
    }

    impl ObjectImpl for PuzzledTileView {}
    impl WidgetImpl for PuzzledTileView {
        fn contains(&self, x: f64, y: f64) -> bool {
            if x < 0.0 || y < 0.0 {
                return false;
            }

            let obj = self.obj();
            let width = obj.width() as f64;
            if x > width {
                return false;
            }
            let height = obj.height() as f64;
            if y > height {
                return false;
            }

            let current_rotation = self.current_rotation.borrow();
            match current_rotation.deref() {
                Polyform::Polyomino { dim: tile_dims, .. } => {
                    let cell_width = width / tile_dims.x() as f64;
                    let cell_height = height / tile_dims.y() as f64;

                    let cell_x = (x / cell_width) as u32;
                    let cell_y = (y / cell_height) as u32;

                    current_rotation.get(&Coord::Regular(RegularCoord::new(cell_x, cell_y))).is_some()
                }
                Polyform::Hexomino { .. } => { todo!() }
            }
        }
    }
    impl DrawingAreaImpl for PuzzledTileView {}
}

glib::wrapper! {
    pub struct TileView(ObjectSubclass<imp::PuzzledTileView>)
        @extends gtk::Widget, gtk::DrawingArea,
         @implements gtk::Buildable, gtk::Accessible, gtk::ConstraintTarget,
                  gtk::Native, gio::ActionGroup, gio::ActionMap;
}

impl TileView {
    /// Creates a new TileView with the given id and base layout.
    /// The name is used to refer to the tile layout when calculating possible solutions for given
    /// tiles.
    pub fn new(id: usize, mut base: Polyform<()>, color: ColorConfig) -> Self {
        let obj: TileView = glib::Object::builder().build();

        obj.imp().id.replace(id);
        obj.imp().current_rotation.replace(base.map(|_| DrawingMode::Normal));
        obj.init_color(color);

        obj.set_draw_func({
            let self_clone = obj.clone();
            move |_, cr, width, height| self_clone.draw(cr, width, height)
        });

        obj
    }

    fn init_color(&self, color: ColorConfig) {
        let color = RGBA::new(
            (color.red() as f64 / 255.0) as f32,
            (color.green() as f64 / 255.0) as f32,
            (color.blue() as f64 / 255.0) as f32,
            (color.alpha() as f64 / 255.0) as f32,
        );

        let mut color_map = HashMap::new();
        color_map.insert(DrawingMode::Normal, color);
        color_map.insert(DrawingMode::Overlapping, color.with_alpha(0.5));
        color_map.insert(DrawingMode::OutOfBounds, color.with_alpha(0.5));
        self.imp().color.replace(color_map);
    }

    fn draw(&self, cr: &Context, width: i32, height: i32) {
        let current_rotation = self.imp().current_rotation.borrow();

        match current_rotation.deref() {
            Polyform::Polyomino { dim, cells } => {
                self.draw_polyomino(cr, width, height, dim, cells);
            }
            Polyform::Hexomino { .. } => { todo!() }
        }
    }

    fn draw_polyomino(&self, cr: &Context, width: i32, height: i32, dim: &RegularCoord, squares: &[Square<DrawingMode>]) {
        let color_map = self.imp().color.borrow();
        for cell in squares.iter() {
            let coord = match cell.coord() {
                Coord::Regular(coord) => coord,
                _ => unreachable!()
            };
            let x = coord.x();
            let y = coord.y();
            let cell_width = width as f64 / dim.x() as f64;
            let cell_height = height as f64 / dim.y() as f64;
            let cell_x = x as f64 * cell_width;
            let cell_y = y as f64 * cell_height;

            let drawing_mode = cell.data();
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
                DrawingMode::Normal => None,
                DrawingMode::Overlapping => Some(HIGHLIGHT_OVERLAPPING_COLOR),
                DrawingMode::OutOfBounds => Some(HIGHLIGHT_OUT_OF_BOUNDS_COLOR),
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

    /// Returns the id of the tile to identify it.
    pub fn id(&self) -> usize {
        self.imp().id.get()
    }

    pub fn color(&self) -> RGBA {
        self.imp().color.borrow()[&DrawingMode::Normal]
    }

    /// Rotates the tile one step clockwise.
    pub fn rotate_clockwise(&self) {
        // We are calling counterclockwise here, since the tile is drawn transposed.
        self.imp()
            .current_rotation
            .borrow_mut()
            .rotate_clockwise();
    }

    /// Flips the tile horizontally.
    pub fn flip(&self) {
        self.imp().current_rotation.borrow_mut().flip();
    }

    /// Returns the current layout of the tile, which changes when the tile is rotated or flipped.
    pub fn current_rotation(&self) -> Polyform<()> {
        self.imp().current_rotation.borrow().clone().map(|_| ())
    }

    /// Sets the drawing mode for the cell at the given coordinates.
    pub fn set_drawing_mode_at(&self, coord: &Coord, drawing_mode: DrawingMode) {
        if let Some(mut prototile) = self.imp().current_rotation.borrow_mut().get_mut(coord) {
            prototile.set_data(drawing_mode);
        }
        self.queue_draw();
    }

    /// Resets the drawing mode for all cells to [DrawingMode::Normal].
    pub fn reset_drawing_modes(&self) {
        self.imp()
            .current_rotation
            .borrow_mut()
            .map(|_| DrawingMode::Normal);
        self.queue_draw();
    }
}
