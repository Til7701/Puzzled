use crate::polyform::grid::{Coord, HexCoord, RegularCoord};
use crate::polyform::prototile::{Hexagon, PrototileMutRef, PrototileRef, Square};

pub mod grid;
pub mod prototile;
pub mod iterator;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Polyform<T>
where
    T: Clone,
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
    T: Clone,
{
    pub fn polyomino_sized(size: RegularCoord, value: T) -> Self {
        todo!()
    }

    pub fn polyomino_from_vec<X>(vec: &Vec<Vec<X>>, mapper: &dyn Fn(X, (usize, usize)) -> Option<T>) -> Self {
        todo!()
    }

    pub fn get(&self, coord: &Coord) -> Option<PrototileRef<T>> {
        self.iter().find(|p| p.coord() == coord)
    }

    pub fn get_mut(&mut self, coord: &Coord) -> Option<PrototileMutRef<T>> {
        self.iter_mut().find(|p| p.coord() == coord)
    }

    pub fn dim(&self) -> Coord {
        match self {
            Polyform::Polyomino { dim, .. } => dim.clone().into(),
            Polyform::Hexomino { dim, .. } => dim.clone().into()
        }
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

    pub fn map<R>(&mut self, mapper: fn(T) -> R) -> Polyform<R>
    where
        R: Clone,
    {
        todo!()
    }

    pub fn filter_map<R>(&mut self, mapper: &dyn Fn(T) -> Option<R>) -> Polyform<R>
    where
        R: Clone,
    {
        todo!()
    }

    pub fn trim(&mut self) -> TrimSides {
        let trim_sides = match self {
            Polyform::Polyomino { dim, cells } => {
                Self::polyomino_trim(dim, cells)
            }
            Polyform::Hexomino { .. } => { todo!() }
        };

        trim_sides
    }

    fn polyomino_trim(dim: &mut RegularCoord, cells: &mut Vec<Square<T>>) -> TrimSides {
        let mut lower = RegularCoord::new(0, 0);
        let mut upper = RegularCoord::new(0, 0);
        if dim.x() == 0 || dim.y() == 0 {
            return TrimSides {
                lower: lower.into(),
                upper: upper.into(),
            };
        }

        let min_y = cells.iter().map(|square| {
            match square.coord() {
                Coord::Regular(coord) => coord.y(),
                _ => unreachable!()
            }
        }).min().unwrap();
        cells.iter_mut().for_each(|square| {
            match square.coord().clone() {
                Coord::Regular(mut coord) => {
                    coord.set_y(coord.y() - min_y);
                    square.set_coord(coord.into());
                }
                _ => unreachable!()
            }
        });
        dim.set_y(dim.y() - min_y);
        lower.set_y(lower.y() + min_y);

        let max_y = cells.iter().map(|square| {
            match square.coord() {
                Coord::Regular(coord) => coord.y(),
                _ => unreachable!()
            }
        }).max().unwrap();
        dim.set_y(dim.y() - max_y);
        upper.set_y(upper.y() + max_y);

        let min_x = cells.iter().map(|square| {
            match square.coord() {
                Coord::Regular(coord) => coord.x(),
                _ => unreachable!()
            }
        }).min().unwrap();
        cells.iter_mut().for_each(|square| {
            match square.coord().clone() {
                Coord::Regular(mut coord) => {
                    coord.set_x(coord.x() - min_x);
                    square.set_coord(coord.into());
                }
                _ => unreachable!()
            }
        });
        dim.set_x(min_x);
        lower.set_x(lower.x() + min_x);

        let max_x = cells.iter().map(|square| {
            match square.coord() {
                Coord::Regular(coord) => coord.x(),
                _ => unreachable!()
            }
        }).max().unwrap();
        dim.set_x(max_x);
        upper.set_x(upper.x() + max_x);

        TrimSides {
            lower: lower.into(),
            upper: upper.into(),
        }
    }
}

impl Polyform<()> {
    //#[cfg(test)]
    pub fn polyomino_from_bool_slice<const N: usize>(slice: &[[bool; N]]) -> Self {
        todo!()
    }
}

/// Represents the number of rows and columns removed from the sides of a polyform.
#[derive(Debug, PartialEq, Eq)]
pub struct TrimSides {
    /// The number of rows removed from the lower side.
    pub lower: Coord,
    /// The number of columns removed from the higher side.
    pub upper: Coord,
}
