mod hexagon;
mod square;

use crate::polyform::grid::Coord;

pub use hexagon::Hexagon;
pub use hexagon::HexagonOrientation;
pub use square::Square;
pub use square::SquareOrientation;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Prototile<'a, T>
where
    T: Default + Clone,
{
    Square(&'a Square<T>),
    Hexagon(&'a Hexagon<T>),
}

impl<'a, T> Prototile<'a, T>
where
    T: Default + Clone,
{
    pub fn coord(&self) -> &Coord {
        match self {
            Prototile::Square(square) => square.coord(),
            Prototile::Hexagon(hexagon) => hexagon.coord()
        }
    }

    pub fn orientation(&self) -> Orientation {
        match self {
            Prototile::Square(square) => square.orientation().into(),
            Prototile::Hexagon(hexagon) => hexagon.orientation().into()
        }
    }

    pub fn data(&self) -> &T {
        match self {
            Prototile::Square(square) => square.data(),
            Prototile::Hexagon(hexagon) => hexagon.data()
        }
    }
}

impl<'a, T> From<&'a Square<T>> for Prototile<'a, T>
where
    T: Default + Clone,
{
    fn from(value: &'a Square<T>) -> Self {
        Prototile::Square(value)
    }
}

impl<'a, T> From<&'a Hexagon<T>> for Prototile<'a, T>
where
    T: Default + Clone,
{
    fn from(value: &'a Hexagon<T>) -> Self {
        Prototile::Hexagon(value)
    }
}

pub enum Orientation {
    Square(SquareOrientation),
    Hexagon(HexagonOrientation),
}

impl From<SquareOrientation> for Orientation
{
    fn from(value: SquareOrientation) -> Self {
        Orientation::Square(value)
    }
}

impl From<HexagonOrientation> for Orientation
{
    fn from(value: HexagonOrientation) -> Self {
        Orientation::Hexagon(value)
    }
}
