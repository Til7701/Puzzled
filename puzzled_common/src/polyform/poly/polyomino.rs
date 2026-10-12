use crate::polyform::coord::Coord;
use crate::polyform::coord::coord_box::CoordBox;
use crate::polyform::coord::regular::RegularCoord;
use crate::polyform::poly::{Poly, TrimSides, UnitPoly};
use crate::polyform::prototile::{MutPrototile, Prototile, PrototileMutRef, PrototileRef, Square, SquareOrientation};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Polyomino<T>
where
    T: Clone,
{
    dim: RegularCoord,
    cells: Vec<Square<T>>,
}

impl<T> Polyomino<T>
where
    T: Clone,
{
    pub fn sized(size: RegularCoord, value: T) -> Self {
        let mut squares = Vec::with_capacity((size.x() * size.y()) as usize);
        for i in 0..size.x() {
            for j in 0..size.y() {
                squares.push(Square::new(CoordBox::Regular(RegularCoord::new(i, j)), SquareOrientation::OnSide, value.clone()))
            }
        }

        Polyomino {
            dim: size,
            cells: squares,
        }
    }

    pub fn from_vec<X>(
        vec: &[Vec<X>],
        mapper: &dyn Fn(&X, (usize, usize)) -> Option<T>,
    ) -> Self {
        let size = RegularCoord::new(vec.len() as u32, vec[0].len() as u32);
        let mut squares = Vec::with_capacity(size.area());
        for (i, row) in vec.iter().enumerate() {
            for (j, value) in row.iter().enumerate() {
                if let Some(data) = mapper(value, (i, j)) {
                    squares.push(Square::new(CoordBox::Regular(RegularCoord::new(i as u32, j as u32)), SquareOrientation::OnSide, data))
                }
            }
        }

        Polyomino {
            dim: size,
            cells: squares,
        }
    }
}

impl<T> Poly<T, RegularCoord> for Polyomino<T>
where
    T: Clone,
{
    fn get(&self, coord: &RegularCoord) -> Option<PrototileRef<'_, T>> {
        self.cells.iter()
            .find(|p| p.coord() == &CoordBox::Regular(coord.clone()))
            .map(|s| s.into())
    }

    fn get_mut(&mut self, coord: &RegularCoord) -> Option<PrototileMutRef<'_, T>> {
        self.cells.iter_mut()
            .find(|p| p.coord() == &CoordBox::Regular(coord.clone()))
            .map(|s| s.into())
    }

    fn dim(&self) -> RegularCoord {
        self.dim.clone()
    }

    fn relative_cartesian_dim(&self) -> (f64, f64) {
        (self.dim.x() as f64, self.dim.y() as f64)
    }

    fn area(&self) -> usize {
        self.dim.area()
    }

    fn rotate_to_landscape(&mut self) {
        if self.dim.x() < self.dim.y() {
            self.transpose();
            self.rotate_counterclockwise();
            self.rotate_counterclockwise();
        }
    }

    fn rotate_counterclockwise(&mut self) {
        self.dim.transpose();
        let viewport = CoordBox::Regular(self.dim.clone());
        self.cells.iter_mut().for_each(|s| s.rotate_counterclockwise(&viewport))
    }

    fn flip(&mut self) {
        let viewport = CoordBox::Regular(self.dim.clone());
        self.cells.iter_mut().for_each(|s| s.flip_default(&viewport))
    }

    fn transpose(&mut self) {
        self.dim.transpose();
        self.cells.iter_mut().for_each(|s| s.transpose())
    }

    #[allow(refining_impl_trait)]
    fn mapped<R: Clone>(&self, mapper: fn(&T) -> R) -> Polyomino<R> {
        let squares = self.cells.iter()
            .map(|s| {
                let data = s.data();
                let new_data = mapper(data);
                Square::new(s.coord().clone(), s.orientation(), new_data)
            })
            .collect();
        Polyomino {
            dim: self.dim.clone(),
            cells: squares,
        }
    }

    #[allow(refining_impl_trait)]
    fn mapped_indexed<R: Clone>(&self, mapper: &dyn Fn(&T, &RegularCoord) -> R) -> Polyomino<R> {
        let squares = self.cells.iter()
            .map(|s| {
                let data = s.data();
                let new_data = mapper(data, s.coord());
                Square::new(s.coord().clone(), s.orientation(), new_data)
            })
            .collect();
        Polyomino {
            dim: self.dim.clone(),
            cells: squares,
        }
    }

    #[allow(refining_impl_trait)]
    fn filter_mapped<R: Clone>(&self, mapper: &dyn Fn(&T) -> Option<R>) -> Polyomino<R> {
        let squares = self.cells.iter()
            .filter_map(|s| {
                let data = s.data();
                let new_data = mapper(data);
                new_data.map(|new_data| Square::new(s.coord().clone(), s.orientation(), new_data))
            })
            .collect();
        Polyomino {
            dim: self.dim.clone(),
            cells: squares,
        }
    }

    fn extend_adjacent(&mut self, value: T) {
        todo!()
    }

    fn trim(&mut self) -> TrimSides<RegularCoord> {
        let mut lower = RegularCoord::new(0, 0);
        let mut upper = RegularCoord::new(0, 0);
        if self.dim.x() == 0 || self.dim.y() == 0 {
            return TrimSides {
                lower: lower.into(),
                upper: upper.into(),
            };
        }

        let min_y = self.cells
            .iter()
            .map(|square| match square.coord() {
                CoordBox::Regular(coord) => coord.y(),
                _ => unreachable!(),
            })
            .min()
            .unwrap();
        self.cells
            .iter_mut()
            .for_each(|square| match square.coord().clone() {
                CoordBox::Regular(mut coord) => {
                    coord.set_y(coord.y() - min_y);
                    square.set_coord(coord.into());
                }
                _ => unreachable!(),
            });
        self.dim.set_y(self.dim.y() - min_y);
        lower.set_y(lower.y() + min_y);

        let max_y = self.cells
            .iter()
            .map(|square| match square.coord() {
                CoordBox::Regular(coord) => coord.y(),
                _ => unreachable!(),
            })
            .max()
            .unwrap();
        self.dim.set_y(self.dim.y() - max_y);
        upper.set_y(upper.y() + max_y);

        let min_x = self.cells
            .iter()
            .map(|square| match square.coord() {
                CoordBox::Regular(coord) => coord.x(),
                _ => unreachable!(),
            })
            .min()
            .unwrap();
        self.cells
            .iter_mut()
            .for_each(|square| match square.coord().clone() {
                CoordBox::Regular(mut coord) => {
                    coord.set_x(coord.x() - min_x);
                    square.set_coord(coord.into());
                }
                _ => unreachable!(),
            });
        self.dim.set_x(min_x);
        lower.set_x(lower.x() + min_x);

        let max_x = self.cells
            .iter()
            .map(|square| match square.coord() {
                CoordBox::Regular(coord) => coord.x(),
                _ => unreachable!(),
            })
            .max()
            .unwrap();
        self.dim.set_x(max_x);
        upper.set_x(upper.x() + max_x);

        TrimSides {
            lower: lower.into(),
            upper: upper.into(),
        }
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

impl UnitPoly for Polyomino<()> {
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

impl<T> Default for Polyomino<T>
where
    T: Clone,
{
    fn default() -> Self {
        Polyomino {
            dim: RegularCoord::new(0, 0),
            cells: vec![],
        }
    }
}
