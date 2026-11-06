use adw::gdk::RGBA;
use adw::gdk::cairo::Context;
use adw::gdk::pango::{Alignment, SCALE, WrapMode};
use adw::prelude::GdkCairoContextExt;
use gtk::pango;
use pangocairo::functions::show_layout;
use puzzled_common::polyform::Polyform;
use puzzled_common::polyform::coord::coord_box::CoordBox;
use puzzled_common::polyform::poly::Poly;
use puzzled_common::polyform::poly::polyomino::Polyomino;
use puzzled_common::polyform::prototile::{Prototile, Square};

const BORDER_SIZE_RATIO: f64 = 0.03;
const HIGHLIGHT_SIZE_RATIO: f64 = 0.08;
const TEXT_SIZE_RATIO: f64 = 0.7;
const TRANSPARENT_PROTOTILE_ALPHA: f32 = 0.5;

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
    Hexomino(),
}

#[derive(Clone, Hash, Debug, Default)]
pub struct PolyominoSpecial {
    pub top_neighbour: bool,
    pub bottom_neighbour: bool,
    pub left_neighbour: bool,
    pub right_neighbour: bool,
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
        Polyform::Hexomino(_) => todo!(),
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
    pan.set_alignment(Alignment::Center);
    fit_font_size(
        pan,
        polyomino.cells().iter().map(|c| &c.data().text),
        cell_width,
        cell_height,
    );
    for cell in squares.iter() {
        draw_square(cr, pan, cell_width, cell_height, cell, drawing_settings);
    }
}

fn draw_square<const M: usize>(
    cr: &Context,
    pan: &pango::Layout,
    cell_width: f64,
    cell_height: f64,
    cell: &Square<PolyformDrawingData>,
    drawing_settings: &DrawingSettings<M>,
) {
    let coord = match cell.coord() {
        CoordBox::Regular(coord) => coord,
        _ => unreachable!(),
    };
    let x = coord.x();
    let y = coord.y();
    let cell_x = x as f64 * cell_width;
    let cell_y = y as f64 * cell_height;

    let drawing_data = cell.data();
    let special = match &drawing_data.special {
        PolyformDrawingDataSpecial::Polyomino(s) => s,
        _ => unreachable!(),
    };

    // Main Area
    let color = drawing_settings.main_colors[drawing_data.main_color_index];
    let color = match drawing_data.drawing_mode {
        PrototileDrawingMode::Normal => color,
        PrototileDrawingMode::Overlapping => color.with_alpha(TRANSPARENT_PROTOTILE_ALPHA),
        PrototileDrawingMode::OutOfBounds => color.with_alpha(TRANSPARENT_PROTOTILE_ALPHA),
        PrototileDrawingMode::Highlighted => color,
    };
    cr.set_source_color(&color);
    cr.rectangle(cell_x, cell_y, cell_width, cell_height);
    cr.fill().expect("Failed to fill");
    // Due to floating point inaccuracies, there might be 2px gaps between cells, so
    // additional rectangles are drawn to fill those gaps if the adjacent cells are filled.
    // This only solves the problem, if the color is not transparent, otherwise there
    // would be visible lines between the cells of the tile.
    if color.alpha() == 1.0 {
        if special.left_neighbour {
            cr.rectangle(cell_x + cell_width - 1.0, cell_y, 2.0, cell_height);
            cr.fill().expect("Failed to fill");
        }
        if special.top_neighbour {
            cr.rectangle(cell_x, cell_y + cell_height - 1.0, cell_width, 2.0);
            cr.fill().expect("Failed to fill");
        }
    }

    // Highlighting and Border
    if drawing_data.drawing_mode != PrototileDrawingMode::Normal {
        let highlight_colors = drawing_settings.highlight_colors;
        if let Some(highlight_color) = highlight_colors(&drawing_data.drawing_mode) {
            draw_outline(
                cr,
                cell_x,
                cell_y,
                cell_width,
                cell_height,
                &highlight_color,
                cell_width * HIGHLIGHT_SIZE_RATIO,
            );
        }
    } else if let Some(border_color) = drawing_settings.border_color {
        draw_outline(
            cr,
            cell_x,
            cell_y,
            cell_width,
            cell_height,
            &border_color,
            cell_width * BORDER_SIZE_RATIO,
        );
    }

    // Text
    if let Some(t) = &drawing_data.text {
        cr.save().expect("Failed to save");

        cr.set_source_color(&drawing_settings.text_color);
        pan.set_text(t);

        let (_, text_height) = pan.size();
        let text_height = text_height as f64 / SCALE as f64;
        let y_offset = (cell_height - text_height) / 2.0;
        cr.move_to(cell_x, cell_y + y_offset);

        show_layout(cr, pan);

        cr.restore().expect("Failed to restore");
    }
}

fn draw_outline(
    cr: &Context,
    cell_x: f64,
    cell_y: f64,
    cell_width: f64,
    cell_height: f64,
    color: &RGBA,
    outline_width: f64,
) {
    let half_outline_width = outline_width / 2.0;
    cr.set_source_color(color);
    cr.set_line_width(outline_width);
    cr.rectangle(
        cell_x + half_outline_width,
        cell_y + half_outline_width,
        cell_width - outline_width,
        cell_height - outline_width,
    );
    cr.stroke().expect("Failed to stroke");
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
    let max_width = (cell_width * TEXT_SIZE_RATIO) as i32;
    let max_height = (cell_height * TEXT_SIZE_RATIO) as i32;

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
