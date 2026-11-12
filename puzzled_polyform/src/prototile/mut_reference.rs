use crate::coord::coord_box::CoordBox;
use crate::coord::hex::HexCoord;
use crate::coord::regular::RegularCoord;
use crate::prototile::{Hexagon, MutPrototile, Orientation, Prototile, Square};
use std::hash::Hash;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum PrototileMutRef<'a, T>
where
    T: Clone + Hash,
{
    Polyomino(&'a mut Square<T, RegularCoord>),
    Polyhex(&'a mut Hexagon<T, HexCoord>),
}
impl<'a, T> Prototile<T, CoordBox, Orientation> for PrototileMutRef<'a, T>
where
    T: Clone + Hash,
{
    fn coord(&self) -> CoordBox {
        match self {
            PrototileMutRef::Polyomino(square) => square.coord().into(),
            PrototileMutRef::Polyhex(hexagon) => hexagon.coord().into(),
        }
    }

    fn orientation(&self) -> Orientation {
        match self {
            PrototileMutRef::Polyomino(square) => square.orientation().into(),
            PrototileMutRef::Polyhex(hexagon) => hexagon.orientation().into(),
        }
    }

    fn data(&self) -> &T {
        match self {
            PrototileMutRef::Polyomino(square) => square.data(),
            PrototileMutRef::Polyhex(hexagon) => hexagon.data(),
        }
    }
}
impl<'a, T> MutPrototile<T, CoordBox, Orientation> for PrototileMutRef<'a, T>
where
    T: Clone + Hash,
{
    fn set_coord(&mut self, coord: CoordBox) {
        match (self, coord) {
            (PrototileMutRef::Polyomino(square), CoordBox::Regular(reg)) => square.set_coord(reg),
            (PrototileMutRef::Polyhex(hexagon), CoordBox::Hex(hex)) => hexagon.set_coord(hex),
            _ => unreachable!(),
        }
    }

    fn data_mut(&mut self) -> &mut T {
        match self {
            PrototileMutRef::Polyomino(square) => square.data_mut(),
            PrototileMutRef::Polyhex(hexagon) => hexagon.data_mut(),
        }
    }

    fn set_data(&mut self, data: T) {
        match self {
            PrototileMutRef::Polyomino(square) => square.set_data(data),
            PrototileMutRef::Polyhex(hexagon) => hexagon.set_data(data),
        }
    }

    fn rotate_counterclockwise(&mut self, prime: CoordBox) {
        match (self, prime) {
            (PrototileMutRef::Polyomino(square), CoordBox::Regular(p_reg)) => {
                square.rotate_counterclockwise(p_reg)
            }
            (PrototileMutRef::Polyhex(hexagon), CoordBox::Hex(p_hex)) => {
                hexagon.rotate_counterclockwise(p_hex)
            }
            _ => unreachable!(),
        }
    }

    fn flip_default(&mut self, prime: CoordBox) {
        match (self, prime) {
            (PrototileMutRef::Polyomino(square), CoordBox::Regular(p_reg)) => {
                square.flip_default(p_reg)
            }
            (PrototileMutRef::Polyhex(hexagon), CoordBox::Hex(p_hex)) => {
                hexagon.flip_default(p_hex)
            }
            _ => unreachable!(),
        }
    }

    fn transpose(&mut self) {
        match self {
            PrototileMutRef::Polyomino(square) => square.transpose(),
            PrototileMutRef::Polyhex(hexagon) => hexagon.transpose(),
        }
    }
}

impl<'a, T> From<&'a mut Square<T, RegularCoord>> for PrototileMutRef<'a, T>
where
    T: Clone + Hash,
{
    fn from(value: &'a mut Square<T, RegularCoord>) -> Self {
        PrototileMutRef::Polyomino(value)
    }
}

impl<'a, T> From<&'a mut Hexagon<T, HexCoord>> for PrototileMutRef<'a, T>
where
    T: Clone + Hash,
{
    fn from(value: &'a mut Hexagon<T, HexCoord>) -> Self {
        PrototileMutRef::Polyhex(value)
    }
}
