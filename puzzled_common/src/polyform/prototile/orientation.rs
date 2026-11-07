use crate::polyform::prototile::{HexagonOrientation, SquareOrientation};

pub enum Orientation {
    Square(SquareOrientation),
    Hexagon(HexagonOrientation),
}

impl From<SquareOrientation> for Orientation {
    fn from(value: SquareOrientation) -> Self {
        Orientation::Square(value)
    }
}

impl From<HexagonOrientation> for Orientation {
    fn from(value: HexagonOrientation) -> Self {
        Orientation::Hexagon(value)
    }
}
