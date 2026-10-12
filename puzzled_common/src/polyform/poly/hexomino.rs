use crate::polyform::coord::hex::HexCoord;
use crate::polyform::poly::{Poly, TrimSides, UnitPoly};
use crate::polyform::prototile::{PrototileMutRef, PrototileRef, Square};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Hexomino<T>
where
    T: Clone,
{
    dim: HexCoord,
    cells: Vec<Square<T>>,
}

impl<T> Hexomino<T>
where
    T: Clone,
{
    pub fn sized(size: HexCoord, value: T) -> Self {
        todo!()
    }

    pub fn from_vec<X>(
        vec: &[Vec<X>],
        mapper: &dyn Fn(&X, (usize, usize)) -> Option<T>,
    ) -> Self {
        todo!()
    }
}

impl<T> Poly<T, HexCoord> for Hexomino<T>
where
    T: Clone,
{
    fn get(&self, coord: &HexCoord) -> Option<PrototileRef<'_, T>> {
        todo!()
    }

    fn get_mut(&mut self, coord: &HexCoord) -> Option<PrototileMutRef<'_, T>> {
        todo!()
    }

    fn dim(&self) -> HexCoord {
        todo!()
    }

    fn relative_cartesian_dim(&self) -> (f64, f64) {
        todo!()
    }

    fn area(&self) -> usize {
        todo!()
    }

    fn rotate_to_landscape(&mut self) {
        todo!()
    }

    fn rotate_counterclockwise(&mut self) {
        todo!()
    }

    fn flip(&mut self) {
        todo!()
    }

    fn transpose(&mut self) {
        todo!()
    }

    fn mapped<R: Clone>(&self, mapper: fn(&T) -> R) -> Hexomino<R> {
        todo!()
    }

    fn mapped_indexed<R: Clone>(&self, mapper: &dyn Fn(&T, &HexCoord) -> R) -> Hexomino<R> {
        todo!()
    }

    fn filter_mapped<R: Clone>(&self, mapper: &dyn Fn(&T) -> Option<R>) -> Hexomino<R> {
        todo!()
    }

    fn extend_adjacent(&mut self, value: T) {
        todo!()
    }

    fn trim(&mut self) -> TrimSides<HexCoord> {
        todo!()
    }

    fn count_biggest_connected_area_of_cells_matching(&self, target_value: bool) -> usize {
        todo!()
    }

    fn count_smallest_connected_area_of_cells_matching(&self, target_value: bool) -> usize {
        todo!()
    }

    fn invert_to_empty(&mut self) {
        todo!()
    }

    fn as_2d_slice(&self) -> String {
        todo!()
    }
}

impl UnitPoly for Hexomino<()> {
    fn place_on_all_positions(&self, child: &Self) -> Vec<Self> {
        todo!()
    }

    fn remove_parent(&mut self, parent: &Self) {
        todo!()
    }

    fn from_bool_slice<const N: usize>(slice: &[[bool; N]]) -> Self {
        todo!()
    }
}
