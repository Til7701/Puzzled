mod drawable;
mod square;
mod style;
mod style_filled;

use crate::app::components::polyform_drawing::drawable::Drawable;
use adw::gdk::RGBA;
use adw::gdk::cairo::Context;
use adw::gdk::pango::{Alignment, SCALE, WrapMode};
use gtk::pango;
use puzzled_polyform::{Poly, Polyform, Polyomino, Prototile};

const BORDER_SIZE_RATIO: f64 = 0.03;
const HIGHLIGHT_SIZE_RATIO: f64 = 0.08;
const TEXT_DEAD_ZONE: f64 = 0.15;

#[derive(Clone, Hash, Debug, Default)]
pub struct PolyformDrawingData {
    pub text: Option<String>,
    pub drawing_mode: PrototileDrawingMode,
    pub main_color_index: usize,
    pub special: PolyformDrawingDataSpecial,
}

/// Defines how a cell of a tile or board should be drawn.
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq)]
pub enum PrototileDrawingMode {
    /// Draw normally
    #[default]
    Normal,
    /// Draw with a highlight indicating that this cell overlaps with another tile
    Overlapping,
    /// Draw with a highlight indicating that this cell is out of bounds of the board
    OutOfBounds,
    /// General highlighting used for target selection on boards.
    Highlighted,
}

#[derive(Debug)]
pub struct DrawingSettings<const M: usize> {
    pub main_colors: [RGBA; M],
    pub highlight_colors: fn(&PrototileDrawingMode) -> Option<RGBA>,
    pub border_color: Option<RGBA>,
    pub text_color: RGBA,
}

#[derive(Clone, Hash, Debug)]
pub enum PolyformDrawingDataSpecial {
    Polyomino(PolyominoSpecial),
    Polyhex(),
}

#[derive(Clone, Hash, Debug, Default)]
pub struct PolyominoSpecial {
    pub x_neg_neighbour: bool,
    pub x_pos_neighbour: bool,
    pub y_neg_neighbour: bool,
    pub y_pos_neighbour: bool,
}

impl Default for PolyformDrawingDataSpecial {
    fn default() -> Self {
        PolyformDrawingDataSpecial::Polyomino(PolyominoSpecial::default())
    }
}

pub fn draw<const M: usize>(
    cr: &Context,
    pan: &pango::Layout,
    width: i32,
    height: i32,
    polyomino: &Polyform<PolyformDrawingData>,
    drawing_settings: &DrawingSettings<M>,
) {
    match polyomino {
        Polyform::Polyomino(p) => draw_polyomino(cr, pan, width, height, p, drawing_settings),
        Polyform::Polyhex(_) => todo!(),
    }
}

fn draw_polyomino<const M: usize>(
    cr: &Context,
    pan: &pango::Layout,
    width: i32,
    height: i32,
    polyomino: &Polyomino<PolyformDrawingData>,
    drawing_settings: &DrawingSettings<M>,
) {
    let dim = polyomino.dim();
    let squares = polyomino.cells();

    let cell_width = width as f64 / dim.x() as f64;
    let cell_height = height as f64 / dim.y() as f64;
    let first = &squares[0];
    let text_box = first.text_box(TEXT_DEAD_ZONE);
    let text_box_width = (text_box.1.0 - text_box.0.0) * cell_width;
    let text_box_height = (text_box.1.1 - text_box.0.1) * cell_height;
    pan.set_alignment(Alignment::Center);
    fit_font_size(
        pan,
        polyomino.cells().iter().map(|c| &c.data().text),
        text_box_width,
        text_box_height,
    );
    for cell in squares.iter() {
        let coord = cell.coord();
        let x = coord.x();
        let y = coord.y();
        let cell_x = x as f64 * cell_width;
        let cell_y = y as f64 * cell_height;
        style_filled::draw_drawable(cr, pan, cell_x, cell_y, cell_width, cell, drawing_settings);
    }
}

fn fit_font_size<'a>(
    layout: &pango::Layout,
    texts: impl Iterator<Item = &'a Option<String>>,
    cell_width: f64,
    cell_height: f64,
) {
    layout.set_width(cell_width as i32 * SCALE);
    layout.set_height(-1);
    layout.set_wrap(WrapMode::Word);

    let mut largest_text = (None, 0);
    for t in texts.flat_map(|t| t.iter()) {
        layout.set_text(t);
        let (width, _) = layout.size();
        if largest_text.1 < width {
            largest_text = (Some(t), width)
        }
    }
    if largest_text.0.is_none() {
        return;
    }
    layout.set_text(largest_text.0.unwrap());
    let max_font_size = cell_height * 0.7;
    let max_width = cell_width as i32;
    let max_height = cell_height as i32;

    let mut low = 1;
    let mut high = max_font_size as i32;
    let mut best = 1;
    let mut desc = layout.font_description().unwrap_or_default();

    while low <= high {
        let mid = low + (high - low) / 2;

        desc.set_size(mid * SCALE);
        layout.set_font_description(Some(&desc));

        let (width, height) = layout.size();

        if width <= max_width * SCALE && height <= max_height * SCALE {
            best = mid;
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    desc.set_size(best * SCALE);
    layout.set_font_description(Some(&desc));
}
