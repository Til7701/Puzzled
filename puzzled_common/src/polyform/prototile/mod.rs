mod hexagon;
mod square;

use crate::polyform::grid::Coord;

pub use hexagon::Hexagon;
pub use hexagon::HexagonOrientation;
pub use square::Square;
pub use square::SquareOrientation;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PrototileRef<'a, T>
where
    T: Clone,
{
    Square(&'a Square<T>),
    Hexagon(&'a Hexagon<T>),
}

impl<'a, T> PrototileRef<'a, T>
where
    T: Clone,
{
    pub fn coord(&self) -> &Coord {
        match self {
            PrototileRef::Square(square) => square.coord(),
            PrototileRef::Hexagon(hexagon) => hexagon.coord(),
        }
    }

    pub fn orientation(&self) -> Orientation {
        match self {
            PrototileRef::Square(square) => square.orientation().into(),
            PrototileRef::Hexagon(hexagon) => hexagon.orientation().into(),
        }
    }

    pub fn data(&self) -> &T {
        match self {
            PrototileRef::Square(square) => square.data(),
            PrototileRef::Hexagon(hexagon) => hexagon.data(),
        }
    }
}

impl<'a, T> From<&'a Square<T>> for PrototileRef<'a, T>
where
    T: Clone,
{
    fn from(value: &'a Square<T>) -> Self {
        PrototileRef::Square(value)
    }
}

impl<'a, T> From<&'a Hexagon<T>> for PrototileRef<'a, T>
where
    T: Clone,
{
    fn from(value: &'a Hexagon<T>) -> Self {
        PrototileRef::Hexagon(value)
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum PrototileMutRef<'a, T>
where
    T: Clone,
{
    Square(&'a mut Square<T>),
    Hexagon(&'a mut Hexagon<T>),
}

impl<'a, T> PrototileMutRef<'a, T>
where
    T: Clone,
{
    pub fn coord(&self) -> &Coord {
        match self {
            PrototileMutRef::Square(square) => square.coord(),
            PrototileMutRef::Hexagon(hexagon) => hexagon.coord(),
        }
    }

    pub fn orientation(&self) -> Orientation {
        match self {
            PrototileMutRef::Square(square) => square.orientation().into(),
            PrototileMutRef::Hexagon(hexagon) => hexagon.orientation().into(),
        }
    }

    pub fn data(&self) -> &T {
        match self {
            PrototileMutRef::Square(square) => square.data(),
            PrototileMutRef::Hexagon(hexagon) => hexagon.data(),
        }
    }

    pub fn set_data(&mut self, data: T) {
        match self {
            PrototileMutRef::Square(square) => square.set_data(data),
            PrototileMutRef::Hexagon(hexagon) => hexagon.set_data(data),
        }
    }
}

impl<'a, T> From<&'a mut Square<T>> for PrototileMutRef<'a, T>
where
    T: Clone,
{
    fn from(value: &'a mut Square<T>) -> Self {
        PrototileMutRef::Square(value)
    }
}

impl<'a, T> From<&'a mut Hexagon<T>> for PrototileMutRef<'a, T>
where
    T: Clone,
{
    fn from(value: &'a mut Hexagon<T>) -> Self {
        PrototileMutRef::Hexagon(value)
    }
}

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
