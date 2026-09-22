use crate::polyform::grid::{Coord, HexCoord, RegularCoord};
use crate::polyform::prototile::{Hexagon, PrototileMutRef, PrototileRef, Square, SquareOrientation};

pub mod grid;
pub mod iterator;
pub mod prototile;

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
        let mut squares = Vec::with_capacity((size.x() * size.y()) as usize);
        for i in 0..size.x() {
            for j in 0..size.y() {
                squares.push(Square::new(Coord::Regular(RegularCoord::new(i, j)), SquareOrientation::OnSide, value.clone()))
            }
        }

        Self::Polyomino {
            dim: size,
            cells: squares,
        }
    }

    pub fn polyomino_from_vec<X>(
        vec: &[Vec<X>],
        mapper: &dyn Fn(&X, (usize, usize)) -> Option<T>,
    ) -> Self {
        let size = RegularCoord::new(vec.len() as u32, vec[0].len() as u32);
        let mut squares = Vec::with_capacity(size.area());
        for (i, row) in vec.iter().enumerate() {
            for (j, value) in row.iter().enumerate() {
                if let Some(data) = mapper(value, (i, j)) {
                    squares.push(Square::new(Coord::Regular(RegularCoord::new(i as u32, j as u32)), SquareOrientation::OnSide, data))
                }
            }
        }

        Self::Polyomino {
            dim: size,
            cells: squares,
        }
    }

    pub fn get(&self, coord: &Coord) -> Option<PrototileRef<'_, T>> {
        self.iter().find(|p| p.coord() == coord)
    }

    pub fn get_mut(&mut self, coord: &Coord) -> Option<PrototileMutRef<'_, T>> {
        self.iter_mut().find(|p| p.coord() == coord)
    }

    pub fn dim(&self) -> Coord {
        match self {
            Polyform::Polyomino { dim, .. } => dim.clone().into(),
            Polyform::Hexomino { dim, .. } => dim.clone().into(),
        }
    }

    pub fn relative_cartesian_dim(&self) -> (f64, f64) {
        match self {
            Polyform::Polyomino { dim, .. } => (dim.x() as f64, dim.y() as f64),
            Polyform::Hexomino { .. } => todo!(),
        }
    }

    pub fn area(&self) -> usize {
        match self {
            Polyform::Polyomino { dim, .. } => dim.area(),
            Polyform::Hexomino { dim, .. } => dim.area(),
        }
    }

    pub fn rotate_to_landscape(&mut self) {
        match self {
            Polyform::Polyomino { dim, .. } => {
                if dim.x() < dim.y() {
                    self.transpose();
                    self.rotate_counterclockwise();
                    self.rotate_counterclockwise();
                }
            }
            Polyform::Hexomino { .. } => todo!(),
        }
    }

    pub fn rotate_counterclockwise(&mut self) {
        match self {
            Polyform::Polyomino { dim, cells } => {
                let viewport = Coord::Regular(dim.clone());
                cells.iter_mut().for_each(|s| s.rotate_counterclockwise(&viewport))
            }
            Polyform::Hexomino { dim, cells } => {
                let viewport = Coord::Hex(dim.clone());
                cells.iter_mut().for_each(|s| s.rotate_counterclockwise(&viewport))
            }
        }
    }

    pub fn flip(&mut self) {
        match self {
            Polyform::Polyomino { dim, cells } => {
                let viewport = Coord::Regular(dim.clone());
                cells.iter_mut().for_each(|s| s.flip_default(&viewport))
            }
            Polyform::Hexomino { dim, cells } => {
                let viewport = Coord::Hex(dim.clone());
                cells.iter_mut().for_each(|s| s.flip_default(&viewport))
            }
        }
    }

    pub fn transpose(&mut self) {
        match self {
            Polyform::Polyomino { cells, .. } => {
                cells.iter_mut().for_each(|s| s.transpose())
            }
            Polyform::Hexomino { cells, .. } => {
                cells.iter_mut().for_each(|s| s.transpose())
            }
        }
    }

    pub fn map<R>(&mut self, mapper: fn(&T) -> R) -> Polyform<R>
    where
        R: Clone,
    {
        match self {
            Polyform::Polyomino { dim, cells } => {
                let squares = cells.iter()
                    .map(|s| {
                        let data = s.data();
                        let new_data = mapper(data);
                        Square::new(s.coord().clone(), s.orientation(), new_data)
                    })
                    .collect();
                Polyform::Polyomino {
                    dim: dim.clone(),
                    cells: squares,
                }
            }
            Polyform::Hexomino { dim, cells } => {
                let hexagons = cells.iter()
                    .map(|h| {
                        let data = h.data();
                        let new_data = mapper(data);
                        Hexagon::new(h.coord().clone(), h.orientation(), new_data)
                    })
                    .collect();
                Polyform::Hexomino {
                    dim: dim.clone(),
                    cells: hexagons,
                }
            }
        }
    }

    pub fn map_indexed<R>(&mut self, mapper: &dyn Fn(&T, &Coord) -> R) -> Polyform<R>
    where
        R: Clone,
    {
        match self {
            Polyform::Polyomino { dim, cells } => {
                let squares = cells.iter()
                    .map(|s| {
                        let data = s.data();
                        let new_data = mapper(data, s.coord());
                        Square::new(s.coord().clone(), s.orientation(), new_data)
                    })
                    .collect();
                Polyform::Polyomino {
                    dim: dim.clone(),
                    cells: squares,
                }
            }
            Polyform::Hexomino { dim, cells } => {
                let hexagons = cells.iter()
                    .map(|h| {
                        let data = h.data();
                        let new_data = mapper(data, h.coord());
                        Hexagon::new(h.coord().clone(), h.orientation(), new_data)
                    })
                    .collect();
                Polyform::Hexomino {
                    dim: dim.clone(),
                    cells: hexagons,
                }
            }
        }
    }

    pub fn filter_map<R>(&mut self, mapper: &dyn Fn(&T) -> Option<R>) -> Polyform<R>
    where
        R: Clone,
    {
        match self {
            Polyform::Polyomino { dim, cells } => {
                let squares = cells.iter()
                    .filter_map(|s| {
                        let data = s.data();
                        let new_data = mapper(data);
                        new_data.map(|new_data| Square::new(s.coord().clone(), s.orientation(), new_data))
                    })
                    .collect();
                Polyform::Polyomino {
                    dim: dim.clone(),
                    cells: squares,
                }
            }
            Polyform::Hexomino { dim, cells } => {
                let hexagons = cells.iter()
                    .filter_map(|h| {
                        let data = h.data();
                        let new_data = mapper(data);
                        new_data.map(|new_data| Hexagon::new(h.coord().clone(), h.orientation(), new_data))
                    })
                    .collect();
                Polyform::Hexomino {
                    dim: dim.clone(),
                    cells: hexagons,
                }
            }
        }
    }

    pub fn extend_adjacent(&self, value: T) {
        todo!()
    }

    pub fn trim(&mut self) -> TrimSides {
        match self {
            Polyform::Polyomino { dim, cells } => Self::polyomino_trim(dim, cells),
            Polyform::Hexomino { .. } => {
                todo!()
            }
        }
    }

    fn polyomino_trim(dim: &mut RegularCoord, cells: &mut [Square<T>]) -> TrimSides {
        let mut lower = RegularCoord::new(0, 0);
        let mut upper = RegularCoord::new(0, 0);
        if dim.x() == 0 || dim.y() == 0 {
            return TrimSides {
                lower: lower.into(),
                upper: upper.into(),
            };
        }

        let min_y = cells
            .iter()
            .map(|square| match square.coord() {
                Coord::Regular(coord) => coord.y(),
                _ => unreachable!(),
            })
            .min()
            .unwrap();
        cells
            .iter_mut()
            .for_each(|square| match square.coord().clone() {
                Coord::Regular(mut coord) => {
                    coord.set_y(coord.y() - min_y);
                    square.set_coord(coord.into());
                }
                _ => unreachable!(),
            });
        dim.set_y(dim.y() - min_y);
        lower.set_y(lower.y() + min_y);

        let max_y = cells
            .iter()
            .map(|square| match square.coord() {
                Coord::Regular(coord) => coord.y(),
                _ => unreachable!(),
            })
            .max()
            .unwrap();
        dim.set_y(dim.y() - max_y);
        upper.set_y(upper.y() + max_y);

        let min_x = cells
            .iter()
            .map(|square| match square.coord() {
                Coord::Regular(coord) => coord.x(),
                _ => unreachable!(),
            })
            .min()
            .unwrap();
        cells
            .iter_mut()
            .for_each(|square| match square.coord().clone() {
                Coord::Regular(mut coord) => {
                    coord.set_x(coord.x() - min_x);
                    square.set_coord(coord.into());
                }
                _ => unreachable!(),
            });
        dim.set_x(min_x);
        lower.set_x(lower.x() + min_x);

        let max_x = cells
            .iter()
            .map(|square| match square.coord() {
                Coord::Regular(coord) => coord.x(),
                _ => unreachable!(),
            })
            .max()
            .unwrap();
        dim.set_x(max_x);
        upper.set_x(upper.x() + max_x);

        TrimSides {
            lower: lower.into(),
            upper: upper.into(),
        }
    }

    pub fn count_biggest_connected_area_of_cells_matching(&self, target_value: bool) -> usize {
        match self {
            Polyform::Polyomino { .. } => {
                self.polyomino_count_biggest_connected_area_of_cells_matching(target_value)
            }
            Polyform::Hexomino { .. } => {
                todo!()
            }
        }
    }

    fn polyomino_count_biggest_connected_area_of_cells_matching(
        &self,
        target_value: bool,
    ) -> usize {
        todo!()
    }

    pub fn count_smallest_connected_area_of_cells_matching(&self, target_value: bool) -> usize {
        match self {
            Polyform::Polyomino { .. } => {
                self.polyomino_count_smallest_connected_area_of_cells_matching(target_value)
            }
            Polyform::Hexomino { .. } => {
                todo!()
            }
        }
    }

    fn polyomino_count_smallest_connected_area_of_cells_matching(
        &self,
        target_value: bool,
    ) -> usize {
        todo!()
    }

    pub fn invert_to_empty(&mut self) {
        todo!()
    }

    pub fn as_2d_slice(&self) -> String {
        todo!()
    }

    #[allow(dead_code)]
    pub fn debug_print(&self) {
        if cfg!(debug_assertions) {
            todo!()
        }
    }
}

impl Polyform<()> {
    pub fn place_on_all_positions(&self, child: &Self) -> Vec<Self> {
        todo!()
    }

    pub fn remove_parent(&mut self, parent: &Self) {
        todo!()
    }

    //#[cfg(test)]
    pub fn polyomino_from_bool_slice<const N: usize>(slice: &[[bool; N]]) -> Self {
        todo!()
    }
}

impl<T> Default for Polyform<T>
where
    T: Clone,
{
    fn default() -> Self {
        Polyform::Polyomino {
            dim: RegularCoord::new(0, 0),
            cells: vec![],
        }
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
