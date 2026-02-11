use crate::adw_ext;
use crate::offset::CellOffset;
use crate::offset::PixelOffset;
use adw::gdk::RGBA;
use adw::gio;
use adw::glib;
use adw::prelude::GdkCairoContextExt;
use adw::subclass::prelude::*;
use gtk::cairo::Context;
use gtk::prelude::{DrawingAreaExtManual, WidgetExt};
use ndarray::{Array2, Axis};
use std::cell::Ref;
use std::collections::HashMap;

const HIGHLIGHT_OVERLAPPING_COLOR: RGBA = adw_ext::ERROR_BG_LIGHT;
const HIGHLIGHT_OUT_OF_BOUNDS_COLOR: RGBA = adw_ext::WARNING_BG_LIGHT;

const COLORS: [RGBA; 35] = [
    adw_ext::BLUE_5,
    adw_ext::GREEN_5,
    adw_ext::YELLOW_5,
    adw_ext::RED_5,
    adw_ext::PURPLE_5,
    adw_ext::ORANGE_5,
    adw_ext::BROWN_5,
    adw_ext::BLUE_2,
    adw_ext::GREEN_2,
    adw_ext::YELLOW_2,
    adw_ext::RED_2,
    adw_ext::PURPLE_2,
    adw_ext::ORANGE_2,
    adw_ext::BROWN_2,
    adw_ext::BLUE_4,
    adw_ext::GREEN_4,
    adw_ext::YELLOW_4,
    adw_ext::RED_4,
    adw_ext::PURPLE_4,
    adw_ext::ORANGE_4,
    adw_ext::BROWN_4,
    adw_ext::BLUE_3,
    adw_ext::GREEN_3,
    adw_ext::YELLOW_3,
    adw_ext::RED_3,
    adw_ext::PURPLE_3,
    adw_ext::ORANGE_3,
    adw_ext::BROWN_3,
    adw_ext::BLUE_1,
    adw_ext::GREEN_1,
    adw_ext::YELLOW_1,
    adw_ext::RED_1,
    adw_ext::PURPLE_1,
    adw_ext::ORANGE_1,
    adw_ext::BROWN_1,
];

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
    use std::cell::{Cell, RefCell};
    use std::collections::HashMap;

    #[derive(Debug, Default)]
    pub struct PuzzledTileView {
        pub id: Cell<usize>,
        pub base: RefCell<Array2<bool>>,
        pub current_rotation: RefCell<Array2<bool>>,
        pub position_cells: Cell<Option<CellOffset>>,
        pub position_pixels: Cell<PixelOffset>,
        pub color: RefCell<HashMap<DrawingMode, RGBA>>,
        pub drawing_modes: RefCell<Array2<DrawingMode>>,
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
            let tile_dims = current_rotation.dim();

            let cell_width = width / tile_dims.0 as f64;
            let cell_height = height / tile_dims.1 as f64;

            let cell_x = (x / cell_width) as usize;
            let cell_y = (y / cell_height) as usize;

            *current_rotation.get((cell_x, cell_y)).unwrap_or(&false)
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
    pub fn new(id: usize, base: Array2<bool>) -> Self {
        let obj: TileView = glib::Object::builder().build();

        obj.imp().id.replace(id);
        obj.imp().base.replace(base.clone());
        obj.imp().drawing_modes.replace(Array2::default(base.dim()));
        obj.imp().current_rotation.replace(base);
        obj.init_color(COLORS[id % COLORS.len()]);

        obj.set_draw_func({
            let self_clone = obj.clone();
            move |_, cr, width, height| self_clone.draw(cr, width, height)
        });

        obj
    }

    fn init_color(&self, color: RGBA) {
        let mut color_map = HashMap::new();
        color_map.insert(DrawingMode::Normal, color);
        color_map.insert(DrawingMode::Overlapping, color.with_alpha(0.5));
        color_map.insert(DrawingMode::OutOfBounds, color.with_alpha(0.5));
        self.imp().color.replace(color_map);
    }

    fn draw(&self, cr: &Context, width: i32, height: i32) {
        let current_rotation = self.imp().current_rotation.borrow();
        let drawing_modes = self.imp().drawing_modes.borrow();

        let color_map = self.imp().color.borrow();
        for ((x, y), cell) in current_rotation.indexed_iter() {
            if *cell {
                let cell_width = width as f64 / current_rotation.dim().0 as f64;
                let cell_height = height as f64 / current_rotation.dim().1 as f64;
                let cell_x = x as f64 * cell_width;
                let cell_y = y as f64 * cell_height;

                let drawing_mode = &drawing_modes[(x, y)];
                cr.set_source_color(&color_map[drawing_mode]);
                cr.rectangle(cell_x, cell_y, cell_width, cell_height);
                cr.fill().expect("Failed to fill");

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
    }

    /// Returns the id of the tile to identify it.
    pub fn id(&self) -> usize {
        self.imp().id.get()
    }

    /// Returns the base layout of the tile, which is the original orientation without any
    /// rotations or flips.
    /// This does not change over the lifetime of the tile.
    pub fn base(&self) -> Ref<'_, Array2<bool>> {
        self.imp().base.borrow()
    }

    /// Rotates the tile one step clockwise.
    pub fn rotate_clockwise(&self) {
        let previous = self.current_rotation().clone();
        let mut layout = previous.reversed_axes();
        layout.invert_axis(Axis(0));
        self.set_current_rotation(layout);
    }

    /// Flips the tile horizontally.
    pub fn flip_horizontal(&self) {
        let mut layout = self.current_rotation().clone();
        layout.invert_axis(Axis(0));
        self.set_current_rotation(layout);
    }

    fn set_current_rotation(&self, rotation: Array2<bool>) {
        self.imp()
            .drawing_modes
            .replace(Array2::default(rotation.dim()));
        self.imp().current_rotation.replace(rotation);
        self.queue_draw();
    }

    /// Returns the current layout of the tile, which changes when the tile is rotated or flipped.
    pub fn current_rotation(&self) -> Ref<'_, Array2<bool>> {
        self.imp().current_rotation.borrow()
    }

    /// Returns the position of the tile in terms of cell coordinates, which is used by the puzzle
    /// area presenter to move the tile to the correct position.
    /// This has to be set manually by the puzzle area presenter when the tile is moved.
    pub fn position_cells(&self) -> Option<CellOffset> {
        self.imp().position_cells.get()
    }

    /// Sets the position of the tile in terms of cell coordinates.
    pub fn set_position_cells(&self, position_cells: Option<CellOffset>) {
        self.imp().position_cells.replace(position_cells);
    }

    /// Returns the position of the tile in terms of pixel coordinates, which is set by the puzzle
    /// area presenter when the tile is moved, and is used to draw the tile at the correct position.
    /// This has to be set manually by the puzzle area presenter.
    pub fn position_pixels(&self) -> PixelOffset {
        self.imp().position_pixels.get()
    }

    /// Sets the position of the tile in terms of pixel coordinates.
    pub fn set_position_pixels(&self, position_pixels: PixelOffset) {
        self.imp().position_pixels.replace(position_pixels);
    }

    /// Sets the drawing mode for the cell at the given coordinates.
    pub fn set_drawing_mode_at(&self, x: usize, y: usize, drawing_mode: DrawingMode) {
        self.imp().drawing_modes.borrow_mut()[(x, y)] = drawing_mode;
        self.queue_draw();
    }

    /// Resets the drawing mode for all cells to [DrawingMode::Normal].
    pub fn reset_drawing_modes(&self) {
        self.imp()
            .drawing_modes
            .borrow_mut()
            .fill(DrawingMode::Normal);
        self.queue_draw();
    }
}
