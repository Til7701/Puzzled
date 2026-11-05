use crate::polyform::coord::Coord;
use crate::polyform::coord::coord_box::CoordBox;
use crate::polyform::coord::regular::RegularCoord;
use crate::polyform::poly::{Poly, TrimSides, UnitPoly};
use crate::polyform::prototile::{
    MutPrototile, Prototile, PrototileMutRef, PrototileRef, Square, SquareOrientation,
};
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Polyomino<T>
where
    T: Clone + Hash,
{
    dim: RegularCoord,
    cells: Vec<Square<T>>,
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
                    CoordBox::Regular(RegularCoord::new(i, j)),
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
        let size = RegularCoord::new(vec.len() as u32, vec[0].len() as u32);
        let mut squares = Vec::with_capacity(size.area());
        for (i, row) in vec.iter().enumerate() {
            for (j, value) in row.iter().enumerate() {
                if let Some(data) = mapper(value, (i, j)) {
                    squares.push(Square::new(
                        CoordBox::Regular(RegularCoord::new(i as u32, j as u32)),
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

    pub fn cells(&self) -> &[Square<T>] {
        &self.cells
    }

    pub fn cells_mut(&mut self) -> &mut [Square<T>] {
        &mut self.cells
    }

    fn get_square(&self, coord: RegularCoord) -> Option<&Square<T>> {
        self.cells
            .iter()
            .find(|p| p.coord() == CoordBox::Regular(coord.clone()))
            .map(|s| s)
    }
}

impl<T> Poly<T, RegularCoord> for Polyomino<T>
where
    T: Clone + Hash,
{
    fn get(&self, coord: RegularCoord) -> Option<PrototileRef<'_, T>> {
        self.cells
            .iter()
            .find(|p| p.coord() == CoordBox::Regular(coord.clone()))
            .map(|s| s.into())
    }

    fn get_mut(&mut self, coord: RegularCoord) -> Option<PrototileMutRef<'_, T>> {
        self.cells
            .iter_mut()
            .find(|p| p.coord() == CoordBox::Regular(coord.clone()))
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
        self.cells
            .iter_mut()
            .for_each(|s| s.rotate_counterclockwise(viewport))
    }

    fn flip(&mut self) {
        let viewport = CoordBox::Regular(self.dim.clone());
        self.cells.iter_mut().for_each(|s| s.flip_default(viewport))
    }

    fn transpose(&mut self) {
        self.dim.transpose();
        self.cells.iter_mut().for_each(|s| s.transpose())
    }

    #[allow(refining_impl_trait)]
    fn mapped<R: Clone + Hash>(&self, mapper: fn(&T) -> R) -> Polyomino<R> {
        let squares = self
            .cells
            .iter()
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
    fn mapped_indexed<R: Clone + Hash>(
        &self,
        mapper: &dyn Fn(&T, RegularCoord) -> R,
    ) -> Polyomino<R> {
        let squares = self
            .cells
            .iter()
            .map(|s| {
                let data = s.data();
                let coord = match s.coord() {
                    CoordBox::Regular(regular) => regular,
                    _ => unreachable!(),
                };
                let new_data = mapper(data, coord);
                Square::new(s.coord().clone(), s.orientation(), new_data)
            })
            .collect();
        Polyomino {
            dim: self.dim.clone(),
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
                new_data.map(|new_data| Square::new(s.coord().clone(), s.orientation(), new_data))
            })
            .collect();
        Polyomino {
            dim: self.dim.clone(),
            cells: squares,
        }
    }

    fn extend_adjacent(&mut self, value: T) {
        self.dim.set_x(self.dim.x() + 2);
        self.dim.set_y(self.dim.y() + 2);
        for cell in &mut self.cells {
            cell.set_coord(cell.coord() + CoordBox::Regular(RegularCoord::new(1, 1)))
        }

        let mut new_cells = Vec::new();

        for i in 1..self.dim.x() - 1 {
            for j in 1..self.dim.y() - 1 {
                if let Some(cell) = self.get_square(RegularCoord::new(i, j)) {
                    if self.get(RegularCoord::new(i - 1, j)).is_none() {
                        new_cells.push(Square::new(
                            RegularCoord::new(i - 1, j).into(),
                            cell.orientation(),
                            value.clone(),
                        ));
                    }
                    if self.get(RegularCoord::new(i + 1, j)).is_none() {
                        new_cells.push(Square::new(
                            RegularCoord::new(i + 1, j).into(),
                            cell.orientation(),
                            value.clone(),
                        ));
                    }
                    if self.get(RegularCoord::new(i, j - 1)).is_none() {
                        new_cells.push(Square::new(
                            RegularCoord::new(i - 1, j).into(),
                            cell.orientation(),
                            value.clone(),
                        ));
                    }
                    if self.get(RegularCoord::new(i, j + 1)).is_none() {
                        new_cells.push(Square::new(
                            RegularCoord::new(i + 1, j).into(),
                            cell.orientation(),
                            value.clone(),
                        ));
                    }
                }
            }
        }

        self.cells.extend(new_cells);
        self.trim();
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

        let min_y = self
            .cells
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

        let max_y = self
            .cells
            .iter()
            .map(|square| match square.coord() {
                CoordBox::Regular(coord) => coord.y(),
                _ => unreachable!(),
            })
            .max()
            .unwrap();
        self.dim.set_y(self.dim.y() - max_y);
        upper.set_y(upper.y() + max_y);

        let min_x = self
            .cells
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

        let max_x = self
            .cells
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
                        let parent_cell =
                            self.get(RegularCoord::new(row_offset + r, col_offset + c));
                        match (parent_cell, child_cell) {
                            (Some(_), Some(_)) => {
                                valid = false;
                                break;
                            }
                            (Some(PrototileRef::Square(parent_c)), None) => {
                                new_array.cells.push(parent_c.clone())
                            }
                            (None, Some(PrototileRef::Square(child_c))) => {
                                new_array.cells.push(child_c.clone())
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
            dim: self.dim.clone(),
            cells: Vec::with_capacity((self.dim.x() * self.dim.y()) as usize - self.cells.len()),
        };

        for i in 0..self.dim.x() {
            for j in 0..self.dim.y() {
                let coord = CoordBox::Regular(RegularCoord::new(i, j));
                if self.cells.iter().any(|c| c.coord() == coord) {
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
        result.push_str("[");
        for i in 0..self.dim.x() {
            result.push_str("[");
            for j in 0..self.dim.y() {
                let cell = self.get(RegularCoord::new(i, j));
                if cell.is_some() {
                    result.push_str("1");
                } else {
                    result.push_str("0");
                }
                if j < self.dim.y() - 1 {
                    result.push_str(",");
                }
            }
            result.push_str("]");
            if i < self.dim.x() - 1 {
                result.push_str(",");
            }
        }
        result.push_str("]");
        result
    }

    fn count_biggest_connected_area_of_cells_matching(&self, target_value: bool) -> usize {
        todo!()
    }

    fn count_smallest_connected_area_of_cells_matching(&self, target_value: bool) -> usize {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rotate_counterclockwise_square_size_1() {
        let mut shape = Polyomino::from_bool_slice(&[[true]]);
        shape.rotate_counterclockwise();
        let expected = Polyomino::from_bool_slice(&[[true]]);
        assert_eq!(expected, shape);
    }

    #[test]
    fn test_rotate_counterclockwise_square_size_2() {
        let mut shape = Polyomino::from_bool_slice(&[[true, false]]);
        shape.rotate_counterclockwise();
        let expected = Polyomino::from_bool_slice(&[[false], [true]]);
        assert_eq!(expected, shape);
    }

    #[test]
    fn test_rotate_counterclockwise_square() {
        let mut shape = Polyomino::from_bool_slice(&[
            [true, false, false],
            [true, true, true],
            [true, false, true],
        ]);
        shape.rotate_counterclockwise();
        let expected = Polyomino::from_bool_slice(&[
            [false, true, true],
            [false, true, false],
            [true, true, true],
        ]);
        assert_eq!(expected, shape);
    }

    #[test]
    fn test_rotate_to_landscape_empty() {
        let mut shape = Polyomino::from_bool_slice(&[[]]);
        shape.rotate_to_landscape();
        let expected = Polyomino::from_bool_slice(&[[]]);
        assert_eq!(expected, shape);
    }

    #[test]
    fn test_rotate_to_landscape_one() {
        let mut shape = Polyomino::from_bool_slice(&[[true]]);
        shape.rotate_to_landscape();
        let expected = Polyomino::from_bool_slice(&[[true]]);
        assert_eq!(expected, shape);
    }

    // #[test]
    // fn test_trim_sides_empty() {
    //     let mut array = Polyomino::from_bool_slice(&[[true]]);
    //     let trim_sides = array.trim_matching(true);
    //     let expected = Polyomino::from_bool_slice(&[[true]]);
    //     assert_eq!(expected, array);
    //     let expected_trim_sides = TrimSides {
    //         lower_x: 0,
    //         lower_y: 0,
    //         upper_y: 0,
    //         upper_x: 0,
    //     };
    //     assert_eq!(expected_trim_sides, trim_sides);
    // }
    //
    // #[test]
    // fn test_trim_sides_true() {
    //     let mut array = Polyomino::from_bool_slice(&[[true]]);
    //     let trim_sides = array.trim_matching(true);
    //     let expected = Polyomino::from_bool_slice(&[[]]);
    //     assert_eq!(expected, array);
    //     let expected_trim_sides = TrimSides {
    //         lower_x: 0,
    //         lower_y: 1,
    //         upper_y: 0,
    //         upper_x: 0,
    //     };
    //     assert_eq!(expected_trim_sides, trim_sides);
    // }
    //
    // #[test]
    // fn test_trim_sides_false() {
    //     let mut array = Polyomino::from_bool_slice(&[[false]]);
    //     let trim_sides = array.trim_matching(false);
    //     let expected = Polyomino::from_bool_slice(&[[]]);
    //     assert_eq!(expected, array);
    //     let expected_trim_sides = TrimSides {
    //         lower_x: 0,
    //         lower_y: 1,
    //         upper_y: 0,
    //         upper_x: 0,
    //     };
    //     assert_eq!(expected_trim_sides, trim_sides);
    // }
    //
    // #[test]
    // fn test_trim_sides_lower_y_upper_y() {
    //     let mut array = Polyomino::from_bool_slice(&[
    //         [true, true, false, true],
    //         [true, false, false, true],
    //         [true, true, false, true],
    //     ]);
    //     let trim_sides = array.trim_matching(true);
    //     let expected = Polyomino::from_bool_slice(&[[true, false], [false, false], [true, false]]);
    //     assert_eq!(expected, array);
    //     let expected_trim_sides = TrimSides {
    //         lower_x: 0,
    //         lower_y: 1,
    //         upper_y: 1,
    //         upper_x: 0,
    //     };
    //     assert_eq!(expected_trim_sides, trim_sides);
    // }
    //
    // #[test]
    // fn test_trim_sides_lower_x_upper_x() {
    //     let mut array = Polyomino::from_bool_slice(&[
    //         [true, true, true, true],
    //         [false, true, false, false],
    //         [true, true, true, true],
    //     ]);
    //     let trim_sides = array.trim_matching(true);
    //     let expected = Polyomino::from_bool_slice(&[[false, true, false, false]]);
    //     assert_eq!(expected, array);
    //     let expected_trim_sides = TrimSides {
    //         lower_x: 1,
    //         lower_y: 0,
    //         upper_y: 0,
    //         upper_x: 1,
    //     };
    //     assert_eq!(expected_trim_sides, trim_sides);
    // }
    //
    // #[test]
    // fn test_trim_sides_all_sides() {
    //     let mut array = Polyomino::from_bool_slice(&[
    //         [true, true, true, true, true],
    //         [true, true, false, false, true],
    //         [true, false, true, false, true],
    //         [true, true, true, true, true],
    //     ]);
    //     let trim_sides = array.trim_matching(true);
    //     let expected = Polyomino::from_bool_slice(&[[true, false, false], [false, true, false]]);
    //     assert_eq!(expected, array);
    //     let expected_trim_sides = TrimSides {
    //         lower_x: 1,
    //         lower_y: 1,
    //         upper_y: 1,
    //         upper_x: 1,
    //     };
    //     assert_eq!(expected_trim_sides, trim_sides);
    // }
    //
    // #[test]
    // fn test_trim_sides_from_lower_y() {
    //     let mut array = Polyomino::from_bool_slice(&[[true, true, false, false], [true, false, true, false]]);
    //     let trim_sides = array.trim_matching(true);
    //     let expected = Polyomino::from_bool_slice(&[[true, false, false], [false, true, false]]);
    //     assert_eq!(expected, array);
    //     let expected_trim_sides = TrimSides {
    //         lower_x: 0,
    //         lower_y: 1,
    //         upper_y: 0,
    //         upper_x: 0,
    //     };
    //     assert_eq!(expected_trim_sides, trim_sides);
    // }
    //
    // #[test]
    // fn test_trim_sides_from_upper_y() {
    //     let mut array = Polyomino::from_bool_slice(&[[false, false, true, true], [false, true, false, true]]);
    //     let trim_sides = array.trim_matching(true);
    //     let expected = Polyomino::from_bool_slice(&[[false, false, true], [false, true, false]]);
    //     assert_eq!(expected, array);
    //     let expected_trim_sides = TrimSides {
    //         lower_x: 0,
    //         lower_y: 0,
    //         upper_y: 1,
    //         upper_x: 0,
    //     };
    //     assert_eq!(expected_trim_sides, trim_sides);
    // }
    //
    // #[test]
    // fn test_trim_sides_from_lower_x() {
    //     let mut array = Polyomino::from_bool_slice(&[
    //         [true, true, true],
    //         [false, true, false],
    //         [true, false, true],
    //     ]);
    //     let trim_sides = array.trim_matching(true);
    //     let expected = Polyomino::from_bool_slice(&[[false, true, false], [true, false, true]]);
    //     assert_eq!(expected, array);
    //     let expected_trim_sides = TrimSides {
    //         lower_x: 1,
    //         lower_y: 0,
    //         upper_y: 0,
    //         upper_x: 0,
    //     };
    //     assert_eq!(expected_trim_sides, trim_sides);
    // }
    //
    // #[test]
    // fn test_trim_sides_from_upper_x() {
    //     let mut array = Polyomino::from_bool_slice(&[
    //         [false, true, false],
    //         [true, false, true],
    //         [true, true, true],
    //     ]);
    //     let trim_sides = array.trim_matching(true);
    //     let expected = Polyomino::from_bool_slice(&[[false, true, false], [true, false, true]]);
    //     assert_eq!(expected, array);
    //     let expected_trim_sides = TrimSides {
    //         lower_x: 0,
    //         lower_y: 0,
    //         upper_y: 0,
    //         upper_x: 1,
    //     };
    //     assert_eq!(expected_trim_sides, trim_sides);
    // }
    //
    // #[test]
    // fn test_trim_sides_rows_cols_test() {
    //     let mut array = Polyomino::from_bool_slice(&[
    //         [false, false, false, false],
    //         [false, false, false, false],
    //         [true, true, true, true],
    //         [false, true, false, true],
    //         [true, true, true, true],
    //     ]);
    //     let trim_sides = array.trim_matching(true);
    //     let expected = Polyomino::from_bool_slice(&[
    //         [false, false, false, false],
    //         [false, false, false, false],
    //         [true, true, true, true],
    //         [false, true, false, true],
    //     ]);
    //     assert_eq!(expected, array);
    //     let expected_trim_sides = TrimSides {
    //         lower_x: 0,
    //         lower_y: 0,
    //         upper_y: 0,
    //         upper_x: 1,
    //     };
    //     assert_eq!(expected_trim_sides, trim_sides);
    // }

    #[test]
    fn test_count_biggest_connected_area_of_cells_matching_empty() {
        let shape = Polyomino::from_bool_slice(&[[]]);

        let count_true = shape.count_biggest_connected_area_of_cells_matching(true);
        let count_false = shape.count_biggest_connected_area_of_cells_matching(false);

        assert_eq!(count_true, 0);
        assert_eq!(count_false, 0);
    }

    #[test]
    fn test_count_biggest_connected_area_of_cells_matching_true() {
        let shape = Polyomino::from_bool_slice(&[[true]]);

        let count_true = shape.count_biggest_connected_area_of_cells_matching(true);
        let count_false = shape.count_biggest_connected_area_of_cells_matching(false);

        assert_eq!(count_true, 1);
        assert_eq!(count_false, 0);
    }

    #[test]
    fn test_count_biggest_connected_area_of_cells_matching_false() {
        let shape = Polyomino::from_bool_slice(&[[false]]);

        let count_true = shape.count_biggest_connected_area_of_cells_matching(true);
        let count_false = shape.count_biggest_connected_area_of_cells_matching(false);

        assert_eq!(count_true, 0);
        assert_eq!(count_false, 1);
    }

    #[test]
    fn test_count_biggest_connected_area_of_cells_matching_ring() {
        let shape = Polyomino::from_bool_slice(&[
            [true, true, true],
            [true, false, true],
            [true, true, true],
        ]);

        let count_true = shape.count_biggest_connected_area_of_cells_matching(true);
        let count_false = shape.count_biggest_connected_area_of_cells_matching(false);

        assert_eq!(count_true, 8);
        assert_eq!(count_false, 1);
    }

    #[test]
    fn test_count_biggest_connected_area_of_cells_matching_complex() {
        let shape = Polyomino::from_bool_slice(&[
            [true, true, true, false, true],
            [true, false, true, false, true],
            [true, true, true, false, true],
        ]);

        let count_true = shape.count_biggest_connected_area_of_cells_matching(true);
        let count_false = shape.count_biggest_connected_area_of_cells_matching(false);

        assert_eq!(count_true, 8);
        assert_eq!(count_false, 3);
    }

    // #[test]
    // fn test_or_arrays_at() {
    //     let parent = Polyomino::from_bool_slice(&[
    //         [false, false, false],
    //         [false, false, false],
    //         [false, false, false],
    //     ]);
    //     let child = Polyomino::from_bool_slice(&[[true, false], [false, true]]);
    //     let result = parent.or_at(&child, 1, 1);
    //     let expected = Polyomino::from_bool_slice(&[
    //         [false, false, false],
    //         [false, true, false],
    //         [false, false, true],
    //     ]);
    //     assert_eq!(expected, result);
    // }
    //
    // #[test]
    // fn test_or_arrays_at_empty_child() {
    //     let parent = Polyomino::from_bool_slice(&[
    //         [false, false, false],
    //         [false, false, false],
    //         [false, false, false],
    //     ]);
    //     let child = Polyomino::from_bool_slice(&[[]]);
    //     let result = parent.or_at(&child, 1, 1);
    //     let expected = Polyomino::from_bool_slice(&[
    //         [false, false, false],
    //         [false, false, false],
    //         [false, false, false],
    //     ]);
    //     assert_eq!(expected, result);
    // }
    //
    // #[test]
    // fn test_or_arrays_at_child_1x1() {
    //     let parent = Polyomino::from_bool_slice(&[
    //         [false, false, false],
    //         [false, false, false],
    //         [false, false, false],
    //     ]);
    //     let child = Polyomino::from_bool_slice(&[[true]]);
    //     let result = parent.or_at(&child, 1, 1);
    //     let expected = Polyomino::from_bool_slice(&[
    //         [false, false, false],
    //         [false, true, false],
    //         [false, false, false],
    //     ]);
    //     assert_eq!(expected, result);
    // }
    //
    // #[test]
    // fn test_or_arrays_at_child_off_parent() {
    //     let parent = Polyomino::from_bool_slice(&[
    //         [false, false, false],
    //         [false, false, false],
    //         [false, false, false],
    //     ]);
    //     let child = Polyomino::from_bool_slice(&[[true, true], [true, true]]);
    //     let result = parent.or_at(&child, 2, 2);
    //     let expected = Polyomino::from_bool_slice(&[
    //         [false, false, false],
    //         [false, false, false],
    //         [false, false, true],
    //     ]);
    //     assert_eq!(expected, result);
    // }
    //
    // #[test]
    // fn test_or_arrays_at_true_parent() {
    //     let parent = Polyomino::from_bool_slice(&[[true, true, true], [true, true, true], [true, true, true]]);
    //     let child = Polyomino::from_bool_slice(&[[true, false], [true, false]]);
    //     let result = parent.or_at(&child, 1, 1);
    //     let expected = Polyomino::from_bool_slice(&[[true, true, true], [true, true, true], [true, true, true]]);
    //     assert_eq!(expected, result);
    // }
    //
    // #[test]
    // fn test_or_arrays_at_smaller_parent() {
    //     let parent = Polyomino::from_bool_slice(&[[false, false], [false, false]]);
    //     let child = Polyomino::from_bool_slice(&[[true, true, true], [true, true, true], [true, true, true]]);
    //     let result = parent.or_at(&child, 0, 0);
    //     let expected = Polyomino::from_bool_slice(&[[true, true], [true, true]]);
    //     assert_eq!(expected, result);
    // }

    #[test]
    fn test_place_on_all_positions() {
        let parent = Polyomino::from_bool_slice(&[
            [false, false, false],
            [false, false, false],
            [false, false, false],
        ]);
        let child = Polyomino::from_bool_slice(&[[true, false], [false, true]]);
        let placements = parent.place_on_all_positions(&child);
        assert_eq!(placements.len(), 4);
        assert!(placements.contains(&Polyomino::from_bool_slice(&[
            [true, false, false],
            [false, true, false],
            [false, false, false],
        ])));
        assert!(placements.contains(&Polyomino::from_bool_slice(&[
            [false, true, false],
            [false, false, true],
            [false, false, false],
        ])));
        assert!(placements.contains(&Polyomino::from_bool_slice(&[
            [false, false, false],
            [true, false, false],
            [false, true, false],
        ])));
        assert!(placements.contains(&Polyomino::from_bool_slice(&[
            [false, false, false],
            [false, true, false],
            [false, false, true],
        ])));
    }

    #[test]
    fn test_place_on_all_positions_same_size() {
        let parent = Polyomino::from_bool_slice(&[[false, false], [false, false]]);
        let child = Polyomino::from_bool_slice(&[[true, false], [false, true]]);
        let placements = parent.place_on_all_positions(&child);
        assert_eq!(placements.len(), 1);
        assert!(placements.contains(&Polyomino::from_bool_slice(
            &[[true, false], [false, true],]
        )));
    }

    #[test]
    fn test_place_on_all_positions_smaller_parent() {
        let parent = Polyomino::from_bool_slice(&[[false, false], [false, false]]);
        let child = Polyomino::from_bool_slice(&[[true, false, true], [false, true, false]]);
        let placements = parent.place_on_all_positions(&child);
        assert_eq!(placements.len(), 0);
    }

    #[test]
    fn test_place_on_all_positions_with_blocking() {
        let parent = Polyomino::from_bool_slice(&[
            [false, false, false],
            [false, true, false],
            [false, false, false],
        ]);
        let child = Polyomino::from_bool_slice(&[[true, false], [false, true]]);
        let placements = parent.place_on_all_positions(&child);
        assert_eq!(placements.len(), 2);
        assert!(placements.contains(&Polyomino::from_bool_slice(&[
            [false, true, false],
            [false, true, true],
            [false, false, false],
        ])));
        assert!(placements.contains(&Polyomino::from_bool_slice(&[
            [false, false, false],
            [true, true, false],
            [false, true, false],
        ])));
    }

    #[test]
    fn test_remove_parent() {
        let parent = Polyomino::from_bool_slice(&[
            [true, false, true],
            [false, true, false],
            [true, true, true],
        ]);
        let mut child = Polyomino::from_bool_slice(&[
            [true, true, true],
            [true, true, true],
            [true, true, true],
        ]);
        child.remove_parent(&parent);
        let expected = Polyomino::from_bool_slice(&[
            [false, true, false],
            [true, false, true],
            [false, false, false],
        ]);
        assert_eq!(expected, child);
    }

    #[test]
    fn test_remove_parent_smaller_parent() {
        let parent = Polyomino::from_bool_slice(&[[true, false], [false, true]]);
        let mut child = Polyomino::from_bool_slice(&[
            [true, true, true],
            [true, true, true],
            [true, true, true],
        ]);
        child.remove_parent(&parent);
        let expected = Polyomino::from_bool_slice(&[
            [false, true, true],
            [true, false, true],
            [true, true, true],
        ]);
        assert_eq!(expected, child);
    }

    #[test]
    #[should_panic]
    fn test_remove_parent_bigger_parent_panic() {
        let parent = Polyomino::from_bool_slice(&[
            [true, false, true],
            [false, true, false],
            [true, true, true],
        ]);
        let mut child = Polyomino::from_bool_slice(&[[true, true], [true, true]]);
        child.remove_parent(&parent);
    }

    #[test]
    fn test_count_biggest_connected_area_of_cells_matching() {
        let array = Polyomino::from_bool_slice(&[
            [true, false, true],
            [false, true, false],
            [true, true, true],
        ]);
        let count_true = array.count_biggest_connected_area_of_cells_matching(true);
        let count_false = array.count_biggest_connected_area_of_cells_matching(false);
        assert_eq!(count_true, 4);
        assert_eq!(count_false, 1);
    }

    #[test]
    fn test_count_smallest_connected_area_of_cells_matching() {
        let array = Polyomino::from_bool_slice(&[
            [true, false, false],
            [true, false, true],
            [false, true, true],
        ]);
        let count_true = array.count_smallest_connected_area_of_cells_matching(true);
        let count_false = array.count_smallest_connected_area_of_cells_matching(false);
        assert_eq!(count_true, 2);
        assert_eq!(count_false, 1);
    }
}
