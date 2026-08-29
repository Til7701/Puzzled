mod hexagon;
mod square;

use crate::polyform::grid::Coord;

pub use hexagon::Hexagon;
pub use hexagon::HexagonOrientation;
pub use square::Square;
pub use square::SquareOrientation;

pub trait Prototile<O: Orientation, C: Coord> {
    fn orientation(&self) -> &O;

    fn coord(&self) -> &C;
}

pub trait Orientation: Copy {}
