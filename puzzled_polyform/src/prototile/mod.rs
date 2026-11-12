use crate::coord::Coord;
use std::hash::Hash;

mod hexagon;
mod mut_reference;
mod orientation;
mod reference;
mod square;

pub use hexagon::Hexagon;
pub use hexagon::HexagonOrientation;
pub use mut_reference::PrototileMutRef;
pub use orientation::Orientation;
pub use reference::PrototileRef;
pub use square::Square;
pub use square::SquareOrientation;

pub trait Prototile<T, C, O>: Hash
where
    T: Clone,
    C: Coord,
{
    fn coord(&self) -> C;

    fn orientation(&self) -> O;

    fn data(&self) -> &T;
}

pub trait MutPrototile<T, C, O>: Prototile<T, C, O>
where
    T: Clone,
    C: Coord,
{
    fn set_coord(&mut self, coord: C);

    fn data_mut(&mut self) -> &mut T;

    fn set_data(&mut self, data: T);

    fn rotate_counterclockwise(&mut self, prime: C);

    fn flip_default(&mut self, prime: C);

    fn transpose(&mut self);
}
