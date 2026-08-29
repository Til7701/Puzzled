use crate::polyform::grid::Coord;
use crate::polyform::prototile::{Orientation, Prototile};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Square<T, C: Coord>
where
    T: Clone,
{
    coord: C,
    orientation: SquareOrientation,
    data: T,
}

impl<T, C: Coord> Square<T, C>
where
    T: Clone,
{
    pub fn new(coord: C, orientation: SquareOrientation, data: T) -> Self {
        Square { coord, orientation, data }
    }

    pub fn rotate_counterclockwise(&mut self, viewport: (u32, u32)) {
        let old_x = self.x;
        let old_y = self.y;
        self.x = old_y;
        self.y = viewport.1 - old_x;
    }

    pub fn flip_default(&mut self, viewport: (u32, u32)) {
        let old_x = self.x;
        let old_y = self.y;
        self.x = viewport.0 - old_x;
        self.y = old_y;
    }

    pub fn transpose(&mut self, _: (u32, u32)) {
        let old_x = self.x;
        let old_y = self.y;
        self.x = old_x;
        self.y = old_y;
    }
}

impl<T, C: Coord> Prototile<SquareOrientation, C> for Square<T, C>
where
    T: Default + Clone,
{
    fn orientation(&self) -> &SquareOrientation {
        &self.orientation
    }

    fn coord(&self) -> &C {
        &self.coord
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SquareOrientation {
    OnSide,
    OnCorner,
}

impl Orientation for SquareOrientation {}
