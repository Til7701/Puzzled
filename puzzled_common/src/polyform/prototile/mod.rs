use crate::polyform::coord::Coord;

mod hexagon;
mod square;
mod reference;
mod mut_reference;
mod orientation;

pub use reference::PrototileRef;
pub use mut_reference::PrototileMutRef;
pub use orientation::Orientation;
pub use hexagon::Hexagon;
pub use hexagon::HexagonOrientation;
pub use square::Square;
pub use square::SquareOrientation;

pub trait Prototile<T, C, O>
where
    T: Clone,
    C: Coord,
{
    fn coord(&self) -> &C;

    fn orientation(&self) -> O;

    fn data(&self) -> &T;
}

pub trait MutPrototile<T, C, O>: Prototile<T, C, O>
where
    T: Clone,
    C: Coord,
{
    fn set_coord(&mut self, coord: C);

    fn set_data(&mut self, data: T);

    fn rotate_counterclockwise(&mut self, viewport: &C);

    fn flip_default(&mut self, viewport: &C);

    fn transpose(&mut self);
}
