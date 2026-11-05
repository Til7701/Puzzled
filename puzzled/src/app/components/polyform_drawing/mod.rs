use crate::app::components::polyform_drawing::mode::PrototileDrawingMode;
use adw::gdk::RGBA;
use adw::gdk::cairo::Context;
use adw::prelude::GdkCairoContextExt;
use puzzled_common::polyform::Polyform;
use puzzled_common::polyform::coord::coord_box::CoordBox;
use puzzled_common::polyform::poly::Poly;
use puzzled_common::polyform::poly::polyomino::Polyomino;
use puzzled_common::polyform::prototile::Prototile;

pub mod mode;

#[derive(Clone, Hash, Debug, Default)]
pub struct PolyformDrawingData {
    pub drawing_mode: PrototileDrawingMode,
    pub background_color: Option<RGBA>,
    pub border_color: Option<RGBA>,
    pub highlight_color: Option<RGBA>,
    pub text: Option<String>,
}

pub fn draw(cr: &Context, width: i32, height: i32, polyomino: &Polyform<PolyformDrawingData>) {
    match polyomino {
        Polyform::Polyomino(p) => draw_polyomino(cr, width, height, p),
        Polyform::Hexomino(_) => todo!(),
    }
}

fn draw_polyomino(
    cr: &Context,
    width: i32,
    height: i32,
    polyomino: &Polyomino<PolyformDrawingData>,
) {
    let dim = polyomino.dim();
    let squares = polyomino.cells();
    let color_map = self.imp().color.borrow();
    for cell in squares.iter() {
        let coord = match cell.coord() {
            CoordBox::Regular(coord) => coord,
            _ => unreachable!(),
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
            PrototileDrawingMode::Normal => None,
            PrototileDrawingMode::Overlapping => Some(),
            PrototileDrawingMode::OutOfBounds => Some(HIGHLIGHT_OUT_OF_BOUNDS_COLOR),
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
