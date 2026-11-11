use crate::polyform::coord::Coord;
use crate::polyform::coord::regular::RegularCoord;
use crate::polyform::poly::{Poly, TrimSides, UnitPoly};
use crate::polyform::prototile::{
    MutPrototile, Prototile, PrototileMutRef, PrototileRef, Square, SquareOrientation,
};
use std::collections::HashSet;
use std::fmt::{Display, Formatter, Write};
use std::hash::{DefaultHasher, Hash, Hasher};

#[derive(Debug, Clone, Eq)]
pub struct Polyomino<T>
where
    T: Clone + Hash,
{
    dim: RegularCoord,
    cells: Vec<Square<T, RegularCoord>>,
}

impl<T> Polyomino<T>
where
    T: Clone + Hash,
{
    pub fn sized(size: RegularCoord, value: T) -> Self {
        let mut squares = Vec::with_capacity((size.x() * size.y()) as usize);
        for i in 0..size.x() {
            for j in 0..size.y() {
                squares.push(Square::new(
                    RegularCoord::new(i, j),
                    SquareOrientation::OnSide,
                    value.clone(),
                ))
            }
        }

        Polyomino {
            dim: size,
            cells: squares,
        }
    }

    pub fn from_vec<X>(vec: &[Vec<X>], mapper: &dyn Fn(&X, (usize, usize)) -> Option<T>) -> Self {
        let size = RegularCoord::new(
            vec.len() as i32,
            if !vec.is_empty() {
                vec[0].len() as i32
            } else {
                0
            },
        );
        let size = if size.x() == 0 || size.y() == 0 {
            RegularCoord::new(0, 0)
        } else {
            size
        };
        let mut squares = Vec::with_capacity(size.area());
        for (i, row) in vec.iter().enumerate() {
            for (j, value) in row.iter().enumerate() {
                if let Some(data) = mapper(value, (i, j)) {
                    squares.push(Square::new(
                        RegularCoord::new(i as i32, j as i32),
                        SquareOrientation::OnSide,
                        data,
                    ))
                }
            }
        }

        Polyomino {
            dim: size,
            cells: squares,
        }
    }

    pub fn cells(&self) -> &[Square<T, RegularCoord>] {
        &self.cells
    }

    pub fn cells_mut(&mut self) -> &mut [Square<T, RegularCoord>] {
        &mut self.cells
    }

    fn get_square(&self, coord: RegularCoord) -> Option<&Square<T, RegularCoord>> {
        self.cells.iter().find(|p| p.coord() == coord)
    }
}

impl<T> Poly<T, RegularCoord> for Polyomino<T>
where
    T: Clone + Hash,
{
    fn get(&self, coord: RegularCoord) -> Option<PrototileRef<'_, T>> {
        self.get_square(coord).map(|s| s.into())
    }

    fn get_mut(&mut self, coord: RegularCoord) -> Option<PrototileMutRef<'_, T>> {
        self.cells
            .iter_mut()
            .find(|p| p.coord() == coord)
            .map(|s| s.into())
    }

    fn dim(&self) -> RegularCoord {
        self.dim
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
        }
    }

    fn rotate_around(&mut self, prime: RegularCoord) -> RegularCoord {
        let movement_request = {
            let x_mr = -(self.dim.y() - prime.y() - prime.x() - 1);
            let y_mr = prime.y() - prime.x();
            RegularCoord::new(x_mr, y_mr)
        };

        for cell in &mut self.cells {
            cell.set_coord(cell.coord().rotate_around(prime) - movement_request);
        }
        self.dim = self.dim.transpose();
        movement_request
    }

    fn flip(&mut self, prime: RegularCoord) -> RegularCoord {
        let middle = (self.dim.x() as f32 - 1.0) / 2.0;
        let movement_request = if (prime.x() as f32) < middle {
            RegularCoord::new(-(self.dim.x() - prime.x() - 1), 0)
        } else if (prime.x() as f32) > middle {
            RegularCoord::new(prime.x(), 0)
        } else {
            RegularCoord::new(0, 0)
        };

        for cell in &mut self.cells {
            cell.set_coord(cell.coord().flip_default(prime) - movement_request);
        }

        movement_request
    }

    fn transpose(&mut self) {
        self.dim = self.dim.transpose();
        self.cells.iter_mut().for_each(|s| s.transpose())
    }

    #[allow(refining_impl_trait)]
    fn mapped<R: Clone + Hash>(&self, mapper: &dyn Fn(&T) -> R) -> Polyomino<R> {
        let squares = self
            .cells
            .iter()
            .map(|s| {
                let data = s.data();
                let new_data = mapper(data);
                Square::new(s.coord(), s.orientation(), new_data)
            })
            .collect();
        Polyomino {
            dim: self.dim,
            cells: squares,
        }
    }

    #[allow(refining_impl_trait)]
    fn mapped_indexed<R: Clone + Hash>(
        &self,
        mapper: &dyn Fn(&T, RegularCoord) -> R,
    ) -> Polyomino<R> {
        let squares = self
            .cells
            .iter()
            .map(|s| {
                let data = s.data();
                let coord = s.coord();
                let new_data = mapper(data, coord);
                Square::new(s.coord(), s.orientation(), new_data)
            })
            .collect();
        Polyomino {
            dim: self.dim,
            cells: squares,
        }
    }

    #[allow(refining_impl_trait)]
    fn filter_mapped<R: Clone + Hash>(&self, mapper: &dyn Fn(&T) -> Option<R>) -> Polyomino<R> {
        let squares = self
            .cells
            .iter()
            .filter_map(|s| {
                let data = s.data();
                let new_data = mapper(data);
                new_data.map(|new_data| Square::new(s.coord(), s.orientation(), new_data))
            })
            .collect();
        Polyomino {
            dim: self.dim,
            cells: squares,
        }
    }

    fn extend_adjacent(&mut self, value: T) {
        self.dim = RegularCoord::new(self.dim.x() + 2, self.dim.y() + 2);
        for cell in &mut self.cells {
            cell.set_coord(cell.coord() + RegularCoord::new(1, 1))
        }

        let mut placed_coords = HashSet::new();
        let mut new_cells = Vec::new();

        for i in 1..self.dim.x() - 1 {
            for j in 1..self.dim.y() - 1 {
                if let Some(cell) = self.get_square(RegularCoord::new(i, j)) {
                    let x_neg = RegularCoord::new(i - 1, j);
                    if self.get(x_neg).is_none() && !placed_coords.contains(&x_neg) {
                        new_cells.push(Square::new(x_neg, cell.orientation(), value.clone()));
                        placed_coords.insert(x_neg);
                    }
                    let x_pos = RegularCoord::new(i + 1, j);
                    if self.get(x_pos).is_none() && !placed_coords.contains(&x_pos) {
                        new_cells.push(Square::new(x_pos, cell.orientation(), value.clone()));
                        placed_coords.insert(x_pos);
                    }
                    let y_neg = RegularCoord::new(i, j - 1);
                    if self.get(y_neg).is_none() && !placed_coords.contains(&y_neg) {
                        new_cells.push(Square::new(y_neg, cell.orientation(), value.clone()));
                        placed_coords.insert(y_neg);
                    }
                    let y_pos = RegularCoord::new(i, j + 1);
                    if self.get(y_pos).is_none() && !placed_coords.contains(&y_pos) {
                        new_cells.push(Square::new(y_pos, cell.orientation(), value.clone()));
                        placed_coords.insert(y_pos);
                    }
                }
            }
        }

        self.cells.extend(new_cells);
        self.trim();
    }

    fn trim(&mut self) -> TrimSides<RegularCoord> {
        let mut negative = RegularCoord::new(0, 0);
        let mut positive = RegularCoord::new(0, 0);
        if self.dim.x() == 0 || self.dim.y() == 0 {
            return TrimSides { negative, positive };
        }
        if self.cells.is_empty() {
            let old_dim = self.dim;
            self.dim = RegularCoord::new(0, 0);
            return TrimSides {
                negative: RegularCoord::new(old_dim.x() - old_dim.y(), old_dim.y()),
                positive,
            };
        }

        if let Some(min_y) = self.cells.iter().map(|square| square.coord().y()).min() {
            self.cells.iter_mut().for_each(|square| {
                let coord = square.coord();
                square.set_coord(coord.with_y(coord.y() - min_y));
            });
            negative = negative.with_y(min_y);
            self.dim = self.dim.with_y(self.dim.y() - min_y);
        };

        if let Some(max_y) = self.cells.iter().map(|square| square.coord().y()).max() {
            positive = positive.with_y(self.dim.y() - max_y - 1);
            self.dim = self.dim.with_y(max_y + 1);
        }

        if let Some(min_x) = self.cells.iter().map(|square| square.coord().x()).min() {
            self.cells.iter_mut().for_each(|square| {
                let coord = square.coord();
                square.set_coord(coord.with_x(coord.x() - min_x));
            });
            negative = negative.with_x(min_x);
            self.dim = self.dim.with_x(self.dim.x() - min_x);
        }

        if let Some(max_x) = self.cells.iter().map(|square| square.coord().x()).max() {
            positive = positive.with_x(self.dim.x() - max_x - 1);
            self.dim = self.dim.with_x(max_x + 1);
        }

        if self.dim.x() == 0 || self.dim.y() == 0 {
            self.dim = RegularCoord::new(0, 0);
        }

        TrimSides { negative, positive }
    }
}

impl UnitPoly for Polyomino<()> {
    fn place_on_all_positions(&self, child: &Self) -> Vec<Self> {
        let mut placements = Vec::new();
        let parent_rows = self.dim.x();
        let parent_cols = self.dim.y();
        let child_rows = child.dim.x();
        let child_cols = child.dim.y();

        if child_rows > parent_rows || child_cols > parent_cols {
            return placements;
        }

        for row_offset in 0..=(parent_rows - child_rows) {
            for col_offset in 0..=(parent_cols - child_cols) {
                let mut new_array = self.clone();
                let mut valid = true;
                for r in 0..child_rows {
                    for c in 0..child_cols {
                        let child_cell = child.get(RegularCoord::new(r, c));
                        let parent_coord = RegularCoord::new(row_offset + r, col_offset + c);
                        let parent_cell = self.get(parent_coord);
                        match (parent_cell, child_cell) {
                            (Some(_), Some(_)) => {
                                valid = false;
                                break;
                            }
                            (Some(_), None) => {} // Already in cells due to clone above
                            (None, Some(PrototileRef::Polyomino(child_c))) => {
                                let new_cell = Square::new(
                                    parent_coord,
                                    child_c.orientation(),
                                    child_c.data().clone(),
                                );
                                new_array.cells.push(new_cell);
                            }
                            (None, None) => {}
                            (_, _) => unreachable!(),
                        }
                    }
                    if !valid {
                        break;
                    }
                }
                if valid {
                    placements.push(new_array);
                }
            }
        }

        placements
    }

    fn remove_parent(&mut self, parent: &Self) {
        self.cells.retain(|c| !parent.cells.contains(c));
    }

    fn inverted(&self) -> Self {
        let mut result = Self {
            dim: self.dim,
            cells: Vec::with_capacity((self.dim.x() * self.dim.y()) as usize - self.cells.len()),
        };

        for i in 0..self.dim.x() {
            for j in 0..self.dim.y() {
                let coord = RegularCoord::new(i, j);
                if !self.cells.iter().any(|c| c.coord() == coord) {
                    result
                        .cells
                        .push(Square::new(coord, SquareOrientation::OnSide, ()));
                }
            }
        }

        result
    }

    fn as_2d_slice(&self) -> String {
        let mut result = String::new();
        result.push('[');
        for i in 0..self.dim.x() {
            result.push('[');
            for j in 0..self.dim.y() {
                let cell = self.get(RegularCoord::new(i, j));
                if cell.is_some() {
                    result.push('1');
                } else {
                    result.push('0');
                }
                if j < self.dim.y() - 1 {
                    result.push(',');
                }
            }
            result.push(']');
            if i < self.dim.x() - 1 {
                result.push(',');
            }
        }
        result.push(']');
        result
    }

    fn count_biggest_connected_area_of_cells_matching(&self, _target_value: bool) -> usize {
        todo!()
    }

    fn count_smallest_connected_area_of_cells_matching(&self, _target_value: bool) -> usize {
        todo!()
    }

    fn from_bool_slice<const N: usize>(slice: &[[bool; N]]) -> Self {
        let vec: Vec<Vec<bool>> = slice.iter().map(|&v| v.to_vec()).collect();
        Polyomino::from_vec(&vec, &|b, _| if *b { Some(()) } else { None })
    }
}

impl<T> Default for Polyomino<T>
where
    T: Clone + Hash,
{
    fn default() -> Self {
        Polyomino {
            dim: RegularCoord::new(0, 0),
            cells: vec![],
        }
    }
}

impl<T> PartialEq for Polyomino<T>
where
    T: Clone + Hash + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        if self.dim != other.dim || self.cells.len() != other.cells.len() {
            return false;
        }

        for cell in &self.cells {
            if let Some(other_cell) = other.get_square(cell.coord()) {
                if cell != other_cell {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }
}

impl<T> Hash for Polyomino<T>
where
    T: Clone + Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut hashes: Vec<u64> = self
            .cells
            .iter()
            .map(|item| {
                let mut hasher = DefaultHasher::new();
                item.hash(&mut hasher);
                hasher.finish()
            })
            .collect();

        // Sort the hashes to ensure order-independence
        hashes.sort_unstable();

        for hash in hashes {
            hash.hash(state);
        }
        self.dim.hash(state);
    }
}

impl<T> Display for Polyomino<T>
where
    T: Clone + Hash,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char('\n')?;
        for i in 0..self.dim.x() {
            for j in 0..self.dim.y() {
                let text = if self.get(RegularCoord::new(i, j)).is_some() {
                    '#'
                } else {
                    '.'
                };
                f.write_char(text)?;
            }
            f.write_char('\n')?;
        }

        Ok(())
    }
}
