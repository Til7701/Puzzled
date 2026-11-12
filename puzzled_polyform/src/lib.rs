use std::fmt::{Display, Formatter};
use std::hash::Hash;

mod coord;
mod iterator;
mod poly;
mod prototile;

pub use coord::Coord;
pub use coord::coord_box::CoordBox;
pub use coord::hex::HexCoord;
pub use coord::regular::RegularCoord;
pub use poly::Poly;
pub use poly::TrimSides;
pub use poly::UnitPoly;
pub use poly::polyhex::Polyhex;
pub use poly::polyomino::Polyomino;
pub use prototile::Hexagon;
pub use prototile::HexagonOrientation;
pub use prototile::MutPrototile;
pub use prototile::Prototile;
pub use prototile::PrototileMutRef;
pub use prototile::PrototileRef;
pub use prototile::Square;
pub use prototile::SquareOrientation;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Polyform<T>
where
    T: Clone + Hash,
{
    Polyomino(Polyomino<T>),
    Polyhex(Polyhex<T>),
}

impl<T> Poly<T, CoordBox> for Polyform<T>
where
    T: Clone + Hash,
{
    fn get(&self, coord: CoordBox) -> Option<PrototileRef<'_, T>> {
        self.iter().find(|p| p.coord() == coord)
    }

    fn get_mut(&mut self, coord: CoordBox) -> Option<PrototileMutRef<'_, T>> {
        self.iter_mut().find(|p| p.coord() == coord)
    }

    fn dim(&self) -> CoordBox {
        match self {
            Polyform::Polyomino(p) => p.dim().into(),
            Polyform::Polyhex(h) => h.dim().into(),
        }
    }

    fn relative_cartesian_dim(&self) -> (f64, f64) {
        match self {
            Polyform::Polyomino(p) => p.relative_cartesian_dim(),
            Polyform::Polyhex(h) => h.relative_cartesian_dim(),
        }
    }

    fn area(&self) -> usize {
        match self {
            Polyform::Polyomino(p) => p.area(),
            Polyform::Polyhex(h) => h.area(),
        }
    }

    fn rotate_to_landscape(&mut self) {
        match self {
            Polyform::Polyomino(p) => p.rotate_to_landscape(),
            Polyform::Polyhex(h) => h.rotate_to_landscape(),
        }
    }

    fn rotate_around(&mut self, prime: CoordBox) -> CoordBox {
        match (self, prime) {
            (Polyform::Polyomino(p), CoordBox::Regular(reg)) => p.rotate_around(reg).into(),
            (Polyform::Polyhex(h), CoordBox::Hex(hex)) => h.rotate_around(hex).into(),
            _ => unreachable!(),
        }
    }

    fn flip(&mut self, prime: CoordBox) -> CoordBox {
        match (self, prime) {
            (Polyform::Polyomino(p), CoordBox::Regular(reg)) => p.flip(reg).into(),
            (Polyform::Polyhex(h), CoordBox::Hex(hex)) => h.flip(hex).into(),
            _ => unreachable!(),
        }
    }

    fn transpose(&mut self) {
        match self {
            Polyform::Polyomino(p) => p.transpose(),
            Polyform::Polyhex(h) => h.transpose(),
        }
    }

    #[allow(refining_impl_trait_reachable)]
    fn mapped<R, M>(&self, mapper: &M) -> Polyform<R>
    where
        R: Clone + Hash,
        M: Fn(&T) -> R,
    {
        match self {
            Polyform::Polyomino(p) => Polyform::Polyomino(p.mapped(mapper)),
            Polyform::Polyhex(h) => Polyform::Polyhex(h.mapped(mapper)),
        }
    }

    #[allow(refining_impl_trait_reachable)]
    fn mapped_indexed<R, M>(&self, mapper: &M) -> Polyform<R>
    where
        R: Clone + Hash,
        M: Fn(&T, CoordBox) -> R,
    {
        match self {
            Polyform::Polyomino(p) => Polyform::Polyomino(
                p.mapped_indexed(&|data, coord| mapper(data, CoordBox::Regular(coord))),
            ),
            Polyform::Polyhex(h) => Polyform::Polyhex(
                h.mapped_indexed(&|data, coord| mapper(data, CoordBox::Hex(coord))),
            ),
        }
    }

    #[allow(refining_impl_trait_reachable)]
    fn filter_mapped<R, M>(&self, mapper: &M) -> Polyform<R>
    where
        R: Clone + Hash,
        M: Fn(&T) -> Option<R>,
    {
        match self {
            Polyform::Polyomino(p) => Polyform::Polyomino(p.filter_mapped(mapper)),
            Polyform::Polyhex(h) => Polyform::Polyhex(h.filter_mapped(mapper)),
        }
    }

    fn extend_adjacent(&mut self, value: T) {
        match self {
            Polyform::Polyomino(p) => p.extend_adjacent(value),
            Polyform::Polyhex(h) => h.extend_adjacent(value),
        }
    }

    fn trim(&mut self) -> TrimSides<CoordBox> {
        match self {
            Polyform::Polyomino(p) => {
                let trim = p.trim();
                TrimSides {
                    negative: trim.negative.into(),
                    positive: trim.positive.into(),
                }
            }
            Polyform::Polyhex(h) => {
                let trim = h.trim();
                TrimSides {
                    negative: trim.negative.into(),
                    positive: trim.positive.into(),
                }
            }
        }
    }
}

impl UnitPoly for Polyform<()> {
    fn place_on_all_positions(&self, child: &Self) -> Vec<Self> {
        match (self, child) {
            (Polyform::Polyomino(p), Polyform::Polyomino(c)) => p
                .place_on_all_positions(c)
                .into_iter()
                .map(|a| a.into())
                .collect(),
            (Polyform::Polyhex(h), Polyform::Polyhex(c)) => h
                .place_on_all_positions(c)
                .into_iter()
                .map(|a| a.into())
                .collect(),
            _ => unreachable!(),
        }
    }

    fn remove_parent(&mut self, parent: &Self) {
        match (self, parent) {
            (Polyform::Polyomino(p), Polyform::Polyomino(par)) => p.remove_parent(par),
            (Polyform::Polyhex(h), Polyform::Polyhex(par)) => h.remove_parent(par),
            _ => unreachable!(),
        }
    }

    fn inverted(&self) -> Self {
        match self {
            Polyform::Polyomino(p) => p.inverted().into(),
            Polyform::Polyhex(h) => h.inverted().into(),
        }
    }

    fn as_2d_slice(&self) -> String {
        match self {
            Polyform::Polyomino(p) => p.as_2d_slice(),
            Polyform::Polyhex(h) => h.as_2d_slice(),
        }
    }

    fn count_biggest_connected_area_of_cells(&self) -> usize {
        match self {
            Polyform::Polyomino(p) => p.count_biggest_connected_area_of_cells(),
            Polyform::Polyhex(h) => h.count_biggest_connected_area_of_cells(),
        }
    }

    fn count_smallest_connected_area_of_cells(&self) -> usize {
        match self {
            Polyform::Polyomino(p) => p.count_smallest_connected_area_of_cells(),
            Polyform::Polyhex(h) => h.count_smallest_connected_area_of_cells(),
        }
    }

    fn from_bool_slice<const N: usize>(_: &[[bool; N]]) -> Self {
        panic!("Not allowed")
    }
}

impl<T> Default for Polyform<T>
where
    T: Clone + Hash,
{
    fn default() -> Self {
        Polyform::Polyomino(Polyomino::default())
    }
}

impl<T: Clone + Hash> From<Polyomino<T>> for Polyform<T> {
    fn from(value: Polyomino<T>) -> Self {
        Polyform::Polyomino(value)
    }
}

impl<T: Clone + Hash> From<Polyhex<T>> for Polyform<T> {
    fn from(value: Polyhex<T>) -> Self {
        Polyform::Polyhex(value)
    }
}

impl<T> Display for Polyform<T>
where
    T: Clone + Hash,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Polyform::Polyomino(p) => p.fmt(f),
            Polyform::Polyhex(_) => todo!(),
        }
    }
}
