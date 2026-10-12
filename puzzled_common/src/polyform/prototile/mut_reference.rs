use crate::polyform::coord::coord_box::CoordBox;
use crate::polyform::prototile::{Hexagon, MutPrototile, Orientation, Prototile, Square};

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum PrototileMutRef<'a, T>
where
    T: Clone,
{
    Square(&'a mut Square<T>),
    Hexagon(&'a mut Hexagon<T>),
}
impl<'a, T> Prototile<T, CoordBox, Orientation> for PrototileMutRef<'a, T>
where
    T: Clone,
{
    fn coord(&self) -> &CoordBox {
        match self {
            PrototileMutRef::Square(square) => square.coord(),
            PrototileMutRef::Hexagon(hexagon) => hexagon.coord(),
        }
    }

    fn orientation(&self) -> Orientation {
        match self {
            PrototileMutRef::Square(square) => square.orientation().into(),
            PrototileMutRef::Hexagon(hexagon) => hexagon.orientation().into(),
        }
    }

    fn data(&self) -> &T {
        match self {
            PrototileMutRef::Square(square) => square.data(),
            PrototileMutRef::Hexagon(hexagon) => hexagon.data(),
        }
    }
}
impl<'a, T> MutPrototile<T, CoordBox, Orientation> for PrototileMutRef<'a, T>
where
    T: Clone,
{
    fn set_coord(&mut self, coord: CoordBox) {
        todo!()
    }

    fn set_data(&mut self, data: T) {
        match self {
            PrototileMutRef::Square(square) => square.set_data(data),
            PrototileMutRef::Hexagon(hexagon) => hexagon.set_data(data),
        }
    }

    fn rotate_counterclockwise(&mut self, viewport: &CoordBox) {
        todo!()
    }

    fn flip_default(&mut self, viewport: &CoordBox) {
        todo!()
    }

    fn transpose(&mut self) {
        todo!()
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
