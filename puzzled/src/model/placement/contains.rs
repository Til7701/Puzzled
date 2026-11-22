use crate::model::placement::PixelPosition;
use puzzled_polyform::{CoordBox, Poly, Polyform, Polyomino, Prototile, RegularCoord};
use std::hash::Hash;

/// Returns the coordinate of the prototile indicated by the location given the size.
/// None, if the location is out of bounds of the size.
pub fn polyform_contains_pixel<T: Clone + Hash>(
    polyform: &Polyform<T>,
    size: PixelPosition,
    location: PixelPosition,
) -> Option<CoordBox> {
    match polyform {
        Polyform::Polyomino(polyomino) => polyomino_contains_pixel(polyomino, size, location),
        Polyform::Polyhex(_) => todo!(),
    }
}

pub fn polyomino_contains_pixel<T: Clone + Hash>(
    polyomino: &Polyomino<T>,
    size: PixelPosition,
    location: PixelPosition,
) -> Option<CoordBox> {
    if location.0 < 0.0 || location.1 < 0.0 {
        return None;
    }

    let width = size.0;
    if location.0 > width {
        return None;
    }
    let height = size.1;
    if location.1 > height {
        return None;
    }

    let tile_dims = polyomino.dim();
    let cell_width = width / tile_dims.x() as f64;
    let cell_height = height / tile_dims.y() as f64;

    let cell_x = (location.0 / cell_width) as i32;
    let cell_y = (location.1 / cell_height) as i32;

    polyomino
        .get(RegularCoord::new(cell_x, cell_y))
        .map(|p| p.coord())
}
