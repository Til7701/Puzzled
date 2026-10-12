use crate::polyform::coord::Coord;
use crate::polyform::coord::coord_box::CoordBox;
use crate::polyform::prototile::{MutPrototile, Prototile};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Hexagon<T>
where
    T: Clone,
{
    coord: CoordBox,
    orientation: HexagonOrientation,
    data: T,
}

impl<T> Hexagon<T>
where
    T: Clone,
{
    pub fn new(coord: CoordBox, orientation: HexagonOrientation, data: T) -> Self {
        Hexagon {
            coord,
            orientation,
            data,
        }
    }
}

impl<T> Prototile<T, CoordBox, HexagonOrientation> for Hexagon<T>
where
    T: Clone,
{
    fn coord(&self) -> &CoordBox {
        &self.coord
    }

    fn orientation(&self) -> HexagonOrientation {
        self.orientation
    }

    fn data(&self) -> &T {
        &self.data
    }
}

impl<T> MutPrototile<T, CoordBox, HexagonOrientation> for Hexagon<T>
where
    T: Clone,
{
    fn set_coord(&mut self, coord: CoordBox) {
        todo!()
    }

    fn set_data(&mut self, data: T) {
        self.data = data;
    }

    fn rotate_counterclockwise(&mut self, viewport: &CoordBox) {
        self.coord.rotate_counterclockwise(viewport);
    }

    fn flip_default(&mut self, viewport: &CoordBox) {
        self.coord.flip_default(viewport);
    }

    fn transpose(&mut self) {
        self.coord.transpose();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HexagonOrientation {
    OnSide,
    OnCorner,
}
