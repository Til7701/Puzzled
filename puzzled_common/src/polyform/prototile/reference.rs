use crate::polyform::coord::coord_box::CoordBox;
use crate::polyform::prototile::{Hexagon, Orientation, Prototile, Square};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PrototileRef<'a, T>
where
    T: Clone,
{
    Square(&'a Square<T>),
    Hexagon(&'a Hexagon<T>),
}

impl<'a, T> Prototile<T, CoordBox, Orientation> for PrototileRef<'a, T>
where
    T: Clone,
{
    fn coord(&self) -> &CoordBox {
        match self {
            PrototileRef::Square(square) => square.coord(),
            PrototileRef::Hexagon(hexagon) => hexagon.coord(),
        }
    }

    fn orientation(&self) -> Orientation {
        match self {
            PrototileRef::Square(square) => square.orientation().into(),
            PrototileRef::Hexagon(hexagon) => hexagon.orientation().into(),
        }
    }

    fn data(&self) -> &T {
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
