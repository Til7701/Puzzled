use crate::polyform::coord::coord_box::CoordBox;
use crate::polyform::poly::hexomino::Hexomino;
use crate::polyform::poly::polyomino::Polyomino;
use crate::polyform::poly::{Poly, TrimSides, UnitPoly};
use crate::polyform::prototile::{Prototile, PrototileMutRef, PrototileRef};

pub mod iterator;
pub mod prototile;
pub mod poly;
pub mod coord;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Polyform<T>
where
    T: Clone,
{
    Polyomino(Polyomino<T>),
    Hexomino(Hexomino<T>),
}

impl<T> Poly<T, CoordBox> for Polyform<T>
where
    T: Clone,
{
    fn get(&self, coord: &CoordBox) -> Option<PrototileRef<'_, T>> {
        self.iter().find(|p| p.coord() == coord)
    }

    fn get_mut(&mut self, coord: &CoordBox) -> Option<PrototileMutRef<'_, T>> {
        self.iter_mut().find(|p| p.coord() == coord)
    }

    fn dim(&self) -> CoordBox {
        match self {
            Polyform::Polyomino(p) => p.dim().into(),
            Polyform::Hexomino(h) => h.dim().into(),
        }
    }

    fn relative_cartesian_dim(&self) -> (f64, f64) {
        match self {
            Polyform::Polyomino(p) => p.relative_cartesian_dim(),
            Polyform::Hexomino(h) => h.relative_cartesian_dim(),
        }
    }

    fn area(&self) -> usize {
        match self {
            Polyform::Polyomino(p) => p.area(),
            Polyform::Hexomino(h) => h.area(),
        }
    }

    fn rotate_to_landscape(&mut self) {
        match self {
            Polyform::Polyomino(p) => p.rotate_to_landscape(),
            Polyform::Hexomino(h) => h.rotate_to_landscape(),
        }
    }

    fn rotate_counterclockwise(&mut self) {
        match self {
            Polyform::Polyomino(p) => p.rotate_counterclockwise(),
            Polyform::Hexomino(h) => h.rotate_counterclockwise(),
        }
    }

    fn flip(&mut self) {
        match self {
            Polyform::Polyomino(p) => p.flip(),
            Polyform::Hexomino(h) => h.flip(),
        }
    }

    fn transpose(&mut self) {
        match self {
            Polyform::Polyomino(p) => p.transpose(),
            Polyform::Hexomino(h) => h.transpose(),
        }
    }

    fn mapped<R>(&self, mapper: fn(&T) -> R) -> Polyform<R>
    where
        R: Clone,
    {
        match self {
            Polyform::Polyomino(p) => Polyform::Polyomino(p.mapped(mapper)),
            Polyform::Hexomino(h) => Polyform::Hexomino(h.mapped(mapper)),
        }
    }

    fn mapped_indexed<R>(&self, mapper: &dyn Fn(&T, &CoordBox) -> R) -> Polyform<R>
    where
        R: Clone,
    {
        match self {
            Polyform::Polyomino(p) => Polyform::Polyomino(p.mapped_indexed(mapper)),
            Polyform::Hexomino(h) => Polyform::Hexomino(h.mapped_indexed(mapper)),
        }
    }

    fn filter_mapped<R>(&self, mapper: &dyn Fn(&T) -> Option<R>) -> Polyform<R>
    where
        R: Clone,
    {
        match self {
            Polyform::Polyomino(p) => Polyform::Polyomino(p.filter_mapped(mapper)),
            Polyform::Hexomino(h) => Polyform::Hexomino(h.filter_mapped(mapper)),
        }
    }

    fn extend_adjacent(&mut self, value: T) {
        match self {
            Polyform::Polyomino(p) => p.extend_adjacent(value),
            Polyform::Hexomino(h) => h.extend_adjacent(value),
        }
    }

    fn trim(&mut self) -> TrimSides<CoordBox> {
        match self {
            Polyform::Polyomino(p) => p.trim().into(),
            Polyform::Hexomino(h) => h.trim().into(),
        }
    }

    fn count_biggest_connected_area_of_cells_matching(&self, target_value: bool) -> usize {
        match self {
            Polyform::Polyomino(p) => p.count_biggest_connected_area_of_cells_matching(target_value),
            Polyform::Hexomino(h) => h.count_biggest_connected_area_of_cells_matching(target_value),
        }
    }

    fn count_smallest_connected_area_of_cells_matching(&self, target_value: bool) -> usize {
        match self {
            Polyform::Polyomino(p) => p.count_smallest_connected_area_of_cells_matching(target_value),
            Polyform::Hexomino(h) => h.count_smallest_connected_area_of_cells_matching(target_value),
        }
    }

    fn invert_to_empty(&mut self) {
        match self {
            Polyform::Polyomino(p) => p.invert_to_empty(),
            Polyform::Hexomino(h) => h.invert_to_empty(),
        }
    }

    fn as_2d_slice(&self) -> String {
        match self {
            Polyform::Polyomino(p) => p.as_2d_slice(),
            Polyform::Hexomino(h) => h.as_2d_slice(),
        }
    }
}

impl UnitPoly for Polyform<()> {
    fn place_on_all_positions(&self, child: &Self) -> Vec<Self> {
        todo!()
    }

    fn remove_parent(&mut self, parent: &Self) {
        todo!()
    }

    fn from_bool_slice<const N: usize>(_: &[[bool; N]]) -> Self {
        panic!("Not allowed")
    }
}

impl<T> Default for Polyform<T>
where
    T: Clone,
{
    fn default() -> Self {
        Polyform::Polyomino(Polyomino::default())
    }
}
