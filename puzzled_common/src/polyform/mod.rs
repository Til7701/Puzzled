use crate::polyform::grid::{Coord, HexCoord, RegularCoord};
use crate::polyform::prototile::{Hexagon, Prototile, Square};

pub mod grid;
pub mod prototile;
pub mod iterator;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Polyform<T>
where
    T: Default + Clone,
{
    Polyomino {
        dim: RegularCoord,
        cells: Vec<Square<T>>,
    },
    Hexomino {
        dim: HexCoord,
        cells: Vec<Hexagon<T>>,
    },
}

impl<T> Polyform<T>
where
    T: Default + Clone,
{
    pub fn polyomino_from_vec<X>(vec: &Vec<Vec<X>>, mapper: fn(X, (usize, usize)) -> Option<T>) -> Self {
        todo!()
    }

    pub fn get(&self, coord: Coord) -> Option<Prototile<T>> {
        self.iter().find(|p| p.coord() == &coord)
    }

    pub fn rotate_to_landscape(&mut self) {
        todo!()
    }

    pub fn rotate_clockwise(&mut self) {
        todo!()
    }

    pub fn flip_horizontally(&mut self) {
        todo!()
    }

    pub fn transpose(&mut self) {
        todo!()
    }
}

impl Polyform<()> {
    pub fn polyomino_sized(x: usize, y: usize) -> Self {
        todo!()
    }

    //#[cfg(test)]
    pub fn polyomino_from_bool_slice<const N: usize>(slice: &[[bool; N]]) -> Self {
        todo!()
    }
}
