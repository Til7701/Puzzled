use crate::coord::coord_box::CoordBox;
use crate::coord::hex::HexCoord;
use crate::coord::regular::RegularCoord;
use crate::prototile::{Hexagon, Orientation, Prototile, Square};
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PrototileRef<'a, T>
where
    T: Clone + Hash,
{
    Polyomino(&'a Square<T, RegularCoord>),
    Polyhex(&'a Hexagon<T, HexCoord>),
}

impl<'a, T> Prototile<T, CoordBox, Orientation> for PrototileRef<'a, T>
where
    T: Clone + Hash,
{
    fn coord(&self) -> CoordBox {
        match self {
            PrototileRef::Polyomino(square) => square.coord().into(),
            PrototileRef::Polyhex(hexagon) => hexagon.coord().into(),
        }
    }

    fn orientation(&self) -> Orientation {
        match self {
            PrototileRef::Polyomino(square) => square.orientation().into(),
            PrototileRef::Polyhex(hexagon) => hexagon.orientation().into(),
        }
    }

    fn data(&self) -> &T {
        match self {
            PrototileRef::Polyomino(square) => square.data(),
            PrototileRef::Polyhex(hexagon) => hexagon.data(),
        }
    }
}

impl<'a, T> From<&'a Square<T, RegularCoord>> for PrototileRef<'a, T>
where
    T: Clone + Hash,
{
    fn from(value: &'a Square<T, RegularCoord>) -> Self {
        PrototileRef::Polyomino(value)
    }
}

impl<'a, T> From<&'a Hexagon<T, HexCoord>> for PrototileRef<'a, T>
where
    T: Clone + Hash,
{
    fn from(value: &'a Hexagon<T, HexCoord>) -> Self {
        PrototileRef::Polyhex(value)
    }
}
