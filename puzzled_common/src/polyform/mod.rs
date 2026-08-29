use crate::polyform::grid::{HexCoord, RegularCoord};
use crate::polyform::prototile::{Hexagon, Square};

pub mod grid;
pub mod prototile;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Polyform<T>
where
    T: Default + Clone,
{
    Polyomino {
        dim: RegularCoord,
        data: Vec<Square<T, RegularCoord>>,
    },
    Hexomino {
        dim: HexCoord,
        data: Vec<Hexagon<T, HexCoord>>,
    },
}
