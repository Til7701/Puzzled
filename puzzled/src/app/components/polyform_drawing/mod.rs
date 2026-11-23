mod drawable;
mod hexagon;
mod prototile;
mod square;
mod style;
mod style_filled;

use crate::app::components::polyform_drawing::drawable::Drawable;
use crate::app::components::polyform_drawing::style::DrawingStyle;

use crate::adw_ext;
use adw::gdk::RGBA;
use adw::gdk::cairo::Context;
use adw::gdk::pango::{Alignment, SCALE, WrapMode};
use adw::prelude::GdkCairoContextExt;
use gtk::pango;
use puzzled_polyform::{HexagonOrientation, Poly, Polyform, Polyhex, Polyomino, Prototile};
use std::hash::Hash;

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
    Polyhex(PolyhexSpecial),
}

impl PolyformDrawingDataSpecial {
    pub fn for_polyform<T: Clone + Hash>(polyform: &Polyform<T>) -> PolyformDrawingDataSpecial {
        match &polyform {
            Polyform::Polyomino(_) => {
                PolyformDrawingDataSpecial::Polyomino(PolyominoSpecial::default())
            }
            Polyform::Polyhex(_) => PolyformDrawingDataSpecial::Polyhex(PolyhexSpecial::default()),
        }
    }
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

#[derive(Clone, Hash, Debug, Default)]
pub struct PolyhexSpecial {
    pub q_neg_neighbour: bool,
    pub q_pos_neighbour: bool,
    pub r_neg_neighbour: bool,
    pub r_pos_neighbour: bool,
    pub s_neg_neighbour: bool,
    pub s_pos_neighbour: bool,
}

pub fn draw<const M: usize>(
    cr: &Context,
    pan: &pango::Layout,
    width: i32,
    height: i32,
    polyform: &Polyform<PolyformDrawingData>,
    drawing_settings: &DrawingSettings<M>,
) {
    let style: DrawingStyle = DrawingStyle::Filled; // TODO read from settings
    match polyform {
        Polyform::Polyomino(p) => {
            draw_polyomino(style, cr, pan, width, height, p, drawing_settings)
        }
        Polyform::Polyhex(p) => draw_polyhex(style, cr, pan, width, height, p, drawing_settings),
    }
}

fn draw_polyomino<const M: usize>(
    style: DrawingStyle,
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
    let text_box = first.normalized_text_box(TEXT_DEAD_ZONE);
    fit_font_size(
        pan,
        polyomino.cells().iter().map(|c| &c.data().text),
        (text_box.1.0 - text_box.0.0) * cell_width,
        (text_box.1.1 - text_box.0.1) * cell_height,
    );

    let draw_function = match style {
        DrawingStyle::Filled => style_filled::draw_drawable,
    };
    for cell in squares.iter() {
        let coord = cell.coord();
        let x = coord.x();
        let y = coord.y();
        let cell_x = x as f64 * cell_width;
        let cell_y = y as f64 * cell_height;
        draw_function(cr, pan, cell_x, cell_y, cell_width, cell, drawing_settings);
    }
}

fn draw_polyhex<const M: usize>(
    style: DrawingStyle,
    cr: &Context,
    pan: &pango::Layout,
    width: i32,
    height: i32,
    polyhex: &Polyhex<PolyformDrawingData>,
    drawing_settings: &DrawingSettings<M>,
) {
    let dim = polyhex.dim();
    let hexagons = polyhex.cells();
    let first = &hexagons[0];
    let orientation = first.orientation();

    let size = match orientation {
        HexagonOrientation::OnSide => todo!(),
        HexagonOrientation::OnCorner => {
            let pixel_dim = dim.to_pixel();
            log::debug!("{:?}", dim);
            log::debug!("{:?}", pixel_dim);
            let cell_width = width as f64 / pixel_dim.0;
            let cell_height = height as f64 / pixel_dim.1;
            cell_width.max(cell_height)
        }
    };

    let text_box = first.normalized_text_box(TEXT_DEAD_ZONE);
    fit_font_size(
        pan,
        polyhex.cells().iter().map(|c| &c.data().text),
        (text_box.1.0 - text_box.0.0) * size,
        (text_box.1.1 - text_box.0.1) * size,
    );

    let draw_function = match style {
        DrawingStyle::Filled => style_filled::draw_drawable,
    };
    for cell in hexagons.iter() {
        let coord = cell.coord();
        let offset = match orientation {
            HexagonOrientation::OnSide => todo!(),
            HexagonOrientation::OnCorner => {
                let offset_coord = coord.to_offset();
                let hex_width = (size / 2.0) * 3f64.sqrt();
                let side = (size - hex_width) / 2.0;
                ((offset_coord.0 * 2 + 1) as f64 * -side, 0.0)
            }
        };
        let (x, y) = coord.to_pixel();
        let cell_x = x * (size / 3f64.sqrt()) + offset.0;
        let cell_y = y * (size / 2.0) + offset.1;
        draw_function(cr, pan, cell_x, cell_y, size, cell, drawing_settings);
    }

    // cr.rectangle(0.0, 0.0, width as f64, height as f64);
    // cr.set_line_width(1.0);
    // cr.set_source_color(&adw_ext::RED_1);
    // cr.stroke().unwrap();
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
    layout.set_alignment(Alignment::Center);

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
    let max_font_size = cell_height;
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
