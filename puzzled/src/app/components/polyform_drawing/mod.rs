mod polyhex;
mod polyomino;

use crate::app::components::polyform_drawing::polyhex::draw_polyhex;
use crate::app::components::polyform_drawing::polyomino::draw_polyomino;
use adw::gdk::RGBA;
use adw::gdk::cairo::Context;
use adw::gdk::pango::{SCALE, WrapMode};
use gtk::pango;
use puzzled_polyform::Polyform;
use std::hash::Hash;

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
    polyomino: &Polyform<PolyformDrawingData>,
    drawing_settings: &DrawingSettings<M>,
) {
    match polyomino {
        Polyform::Polyomino(p) => draw_polyomino(cr, pan, width, height, p, drawing_settings),
        Polyform::Polyhex(p) => draw_polyhex(cr, pan, width, height, p, drawing_settings),
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
