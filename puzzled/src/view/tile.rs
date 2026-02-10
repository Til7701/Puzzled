use crate::offset::CellOffset;
use crate::offset::PixelOffset;
use adw::gdk::RGBA;
use adw::gio;
use adw::glib;
use adw::glib::random_double;
use adw::prelude::GdkCairoContextExt;
use adw::subclass::prelude::*;
use gtk::cairo::Context;
use gtk::prelude::{DrawingAreaExtManual, WidgetExt};
use log::error;
use ndarray::{Array2, Axis};
use std::collections::HashMap;

const HIGHLIGHT_OVERLAPPING_COLOR: RGBA = RGBA::new(1.0, 0.0, 0.0, 1.0);
const HIGHLIGHT_OUT_OF_BOUNDS_COLOR: RGBA = RGBA::new(1.0, 1.0, 0.0, 1.0);

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq)]
pub enum HighlightMode {
    #[default]
    None,
    Overlapping,
    OutOfBounds,
}

mod imp {
    use super::*;
    use std::cell::{Cell, RefCell};

    #[derive(Debug, Default)]
    pub struct PuzzledTileView {
        pub id: Cell<usize>,
        pub base: RefCell<Array2<bool>>,
        pub current_rotation: RefCell<Array2<bool>>,
        pub position_cells: Cell<Option<CellOffset>>,
        pub position_pixels: Cell<PixelOffset>,
        pub color: RefCell<HashMap<HighlightMode, RGBA>>,
        pub highlights: RefCell<Array2<HighlightMode>>,
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
    impl WidgetImpl for PuzzledTileView {}
    impl DrawingAreaImpl for PuzzledTileView {}
}

glib::wrapper! {
    pub struct TileView(ObjectSubclass<imp::PuzzledTileView>)
        @extends gtk::Widget, gtk::DrawingArea,
         @implements gtk::Buildable, gtk::Accessible, gtk::ConstraintTarget,
                  gtk::Native, gio::ActionGroup, gio::ActionMap;
}

impl TileView {
    pub fn new(id: usize, base: Array2<bool>) -> Self {
        let obj: TileView = glib::Object::builder().build();

        obj.imp().id.replace(id);
        obj.imp().base.replace(base.clone());
        obj.imp().highlights.replace(Array2::default(base.dim()));
        obj.imp().current_rotation.replace(base);
        obj.init_color(random_color()); // TODO Replace with color defined in config

        obj.set_draw_func({
            let self_clone = obj.clone();
            move |_, cr, width, height| self_clone.draw(cr, width, height)
        });

        obj
    }

    fn init_color(&self, color: RGBA) {
        let mut color_map = HashMap::new();
        color_map.insert(HighlightMode::None, color);
        color_map.insert(HighlightMode::Overlapping, color.with_alpha(0.5));
        color_map.insert(HighlightMode::OutOfBounds, color.with_alpha(0.5));
        self.imp().color.replace(color_map);
    }

    fn draw(&self, cr: &Context, width: i32, height: i32) {
        error!("Drawing tile {}", self.id());
        let current_rotation = self.imp().current_rotation.borrow();
        let highlights = self.imp().highlights.borrow();

        let color_map = self.imp().color.borrow();
        for ((x, y), cell) in current_rotation.indexed_iter() {
            if *cell {
                let cell_width = width as f64 / current_rotation.dim().0 as f64;
                let cell_height = height as f64 / current_rotation.dim().1 as f64;
                let cell_x = x as f64 * cell_width;
                let cell_y = y as f64 * cell_height;

                let highlight_mode = &highlights[(x, y)];
                cr.set_source_color(&color_map[highlight_mode]);
                cr.rectangle(cell_x, cell_y, cell_width, cell_height);
                cr.fill().expect("Failed to fill");

                // Border
                let border_color = match highlight_mode {
                    HighlightMode::None => None,
                    HighlightMode::Overlapping => Some(HIGHLIGHT_OVERLAPPING_COLOR),
                    HighlightMode::OutOfBounds => Some(HIGHLIGHT_OUT_OF_BOUNDS_COLOR),
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

    pub fn id(&self) -> usize {
        self.imp().id.get()
    }

    pub fn base(&self) -> Array2<bool> {
        self.imp().base.borrow().clone()
    }

    pub fn rotate_clockwise(&self) {
        let base = self.current_rotation();
        let mut rotated = base.reversed_axes();
        rotated.invert_axis(Axis(0));
        self.set_current_rotation(rotated);
    }

    pub fn flip_horizontal(&self) {
        let mut base = self.current_rotation();
        base.invert_axis(Axis(0));
        self.set_current_rotation(base);
    }

    fn set_current_rotation(&self, rotation: Array2<bool>) {
        self.imp()
            .highlights
            .replace(Array2::default(rotation.dim()));
        self.imp().current_rotation.replace(rotation);
        error!("Set current rotation for tile {}", self.id());
        self.queue_draw();
        error!("Queued draw for tile {}", self.id());
    }

    pub fn current_rotation(&self) -> Array2<bool> {
        self.imp().current_rotation.borrow().clone()
    }

    pub fn set_position_cells(&self, position_cells: Option<CellOffset>) {
        self.imp().position_cells.replace(position_cells);
    }

    pub fn position_cells(&self) -> Option<CellOffset> {
        self.imp().position_cells.get()
    }

    pub fn position_pixels(&self) -> PixelOffset {
        self.imp().position_pixels.get()
    }

    pub fn set_position_pixels(&self, position_pixels: PixelOffset) {
        self.imp().position_pixels.replace(position_pixels);
    }

    pub fn highlights(&self) -> Array2<HighlightMode> {
        self.imp().highlights.borrow().clone()
    }

    pub fn set_highlights(&self, highlights: Array2<HighlightMode>) {
        self.imp().highlights.replace(highlights);
        error!("Set highlights for tile {}", self.id());
        self.queue_draw();
        error!("Queued draw for tile {} after highlights", self.id());
    }
}

fn random_color() -> RGBA {
    RGBA::new(
        random_double() as f32,
        random_double() as f32,
        random_double() as f32,
        1.0,
    )
}
