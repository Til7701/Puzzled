use crate::polyform::coord::hex::HexCoord;
use crate::polyform::poly::{Poly, TrimSides, UnitPoly};
use crate::polyform::prototile::{Hexagon, PrototileMutRef, PrototileRef};
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Hexomino<T>
where
    T: Clone + Hash,
{
    dim: HexCoord,
    cells: Vec<Hexagon<T, HexCoord>>,
}

impl<T> Hexomino<T>
where
    T: Clone + Hash,
{
    pub fn sized(_size: HexCoord, _value: T) -> Self {
        todo!()
    }

    pub fn from_vec<X, M>(_vec: &[Vec<X>], _mapper: &M) -> Self
    where
        M: Fn(&X, (usize, usize)) -> Option<T>,
    {
        todo!()
    }

    pub fn cells(&self) -> &[Hexagon<T, HexCoord>] {
        &self.cells
    }

    pub fn cells_mut(&mut self) -> &mut [Hexagon<T, HexCoord>] {
        &mut self.cells
    }
}

impl<T> Poly<T, HexCoord> for Hexomino<T>
where
    T: Clone + Hash,
{
    fn get(&self, _coord: HexCoord) -> Option<PrototileRef<'_, T>> {
        todo!()
    }

    fn get_mut(&mut self, _coord: HexCoord) -> Option<PrototileMutRef<'_, T>> {
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

    fn rotate_around(&mut self, _prime: HexCoord) -> HexCoord {
        todo!()
    }

    fn flip(&mut self, _prime: HexCoord) -> HexCoord {
        todo!()
    }

    fn transpose(&mut self) {
        todo!()
    }

    #[allow(refining_impl_trait_reachable)]
    fn mapped<R: Clone + Hash, M>(&self, _mapper: &M) -> Hexomino<R>
    where
        M: Fn(&T) -> R,
    {
        todo!()
    }

    #[allow(refining_impl_trait_reachable)]
    fn mapped_indexed<R: Clone + Hash, M>(&self, _mapper: &M) -> Hexomino<R>
    where
        M: Fn(&T, HexCoord) -> R,
    {
        todo!()
    }

    #[allow(refining_impl_trait_reachable)]
    fn filter_mapped<R: Clone + Hash, M>(&self, _mapper: &M) -> Hexomino<R>
    where
        M: Fn(&T) -> Option<R>,
    {
        todo!()
    }

    fn extend_adjacent(&mut self, _value: T) {
        todo!()
    }

    fn trim(&mut self) -> TrimSides<HexCoord> {
        todo!()
    }
}

impl UnitPoly for Hexomino<()> {
    fn place_on_all_positions(&self, _child: &Self) -> Vec<Self> {
        todo!()
    }

    fn remove_parent(&mut self, _parent: &Self) {
        todo!()
    }

    fn inverted(&self) -> Self {
        todo!()
    }

    fn as_2d_slice(&self) -> String {
        todo!()
    }

    fn count_biggest_connected_area_of_cells(&self) -> usize {
        todo!()
    }

    fn count_smallest_connected_area_of_cells(&self) -> usize {
        todo!()
    }

    fn from_bool_slice<const N: usize>(_slice: &[[bool; N]]) -> Self {
        todo!()
    }
}
