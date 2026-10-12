use crate::polyform::coord::Coord;
use crate::polyform::prototile::{PrototileMutRef, PrototileRef};

pub mod polyomino;
pub mod hexomino;

pub trait Poly<T, C: Coord>
where
    T: Clone,
{
    fn get(&self, coord: &C) -> Option<PrototileRef<'_, T>>;

    fn get_mut(&mut self, coord: &C) -> Option<PrototileMutRef<'_, T>>;

    fn dim(&self) -> C;

    fn relative_cartesian_dim(&self) -> (f64, f64);

    fn area(&self) -> usize;

    fn rotate_to_landscape(&mut self);

    fn rotate_counterclockwise(&mut self);

    fn flip(&mut self);

    fn transpose(&mut self);

    fn mapped<R: Clone>(&self, mapper: fn(&T) -> R) -> impl Poly<R, C>;

    fn mapped_indexed<R: Clone>(&self, mapper: &dyn Fn(&T, &C) -> R) -> impl Poly<R, C>;

    fn filter_mapped<R: Clone>(&self, mapper: &dyn Fn(&T) -> Option<R>) -> impl Poly<R, C>;

    fn extend_adjacent(&mut self, value: T);

    fn trim(&mut self) -> TrimSides<C>;

    fn count_biggest_connected_area_of_cells_matching(&self, target_value: bool) -> usize;

    fn count_smallest_connected_area_of_cells_matching(&self, target_value: bool) -> usize;

    fn invert_to_empty(&mut self);

    fn as_2d_slice(&self) -> String;
}

pub trait UnitPoly: Sized {
    fn place_on_all_positions(&self, child: &Self) -> Vec<Self>;

    fn remove_parent(&mut self, parent: &Self);

    fn from_bool_slice<const N: usize>(slice: &[[bool; N]]) -> Self;
}

/// Represents the number of rows and columns removed from the sides of a polyform.
#[derive(Debug, PartialEq, Eq)]
pub struct TrimSides<C: Coord> {
    /// The number of rows removed from the lower side.
    pub lower: C,
    /// The number of columns removed from the higher side.
    pub upper: C,
}
