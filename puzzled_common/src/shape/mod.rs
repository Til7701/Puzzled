mod iterators;

use crate::ShapeType::*;
use ndarray::{arr2, s, Array2, Axis};
use std::fmt::{Display, Formatter};
use std::ops::{Index, IndexMut};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Shape {
    shape_type: ShapeType,
    data: Array2<bool>,
}

impl Shape {
    /// Creates a new `Shape` instance with the specified `shape_type` and 2D boolean array `data`.
    ///
    /// # Arguments
    ///
    /// * `shape_type`: the shape type.
    /// * `data`: the data defining the shape
    ///
    /// returns: Shape
    ///
    /// # Examples
    ///
    /// ```
    /// use ndarray::arr2;
    /// use puzzled_common::Shape;
    /// use puzzled_common::ShapeType;
    ///
    /// let shape = Shape::new(ShapeType::Square, arr2(&[[true, false], [false, true]]));
    ///
    /// assert_eq!(shape.shape_type(), ShapeType::Square);
    /// assert_eq!(shape.dim(), (2, 2));
    /// assert_eq!(shape.get((0, 0)), Some(&true));
    /// assert_eq!(shape.get((0, 1)), Some(&false));
    /// assert_eq!(shape.get((1, 0)), Some(&false));
    /// assert_eq!(shape.get((1, 1)), Some(&true));
    /// ```
    pub fn new(shape_type: ShapeType, data: Array2<bool>) -> Self {
        Self { shape_type, data }
    }

    /// Creates a new `Shape` instance with the specified dimensions, shape type, and initial value
    /// for all cells.
    ///
    /// # Arguments
    ///
    /// * `(x, y)`: the dimensions of the shape
    /// * `shape_type`: the shape type for the new shape
    /// * `value`: the value to set all cells to
    ///
    /// returns: Shape
    ///
    /// # Examples
    ///
    /// ```
    /// use puzzled_common::Shape;
    /// use puzzled_common::ShapeType;
    ///
    ///let shape = Shape::from_elem((1, 2), ShapeType::Square, true);
    ///
    /// assert_eq!(shape.shape_type(), ShapeType::Square);
    /// assert_eq!(shape.dim(), (1, 2));
    /// // FIXME remove some
    /// assert_eq!(shape.get((0, 0)), Some(&true));
    /// assert_eq!(shape.get((0, 1)), Some(&true));
    /// ```
    pub fn from_elem((x, y): (usize, usize), shape_type: ShapeType, value: bool) -> Self {
        Self {
            shape_type,
            data: Array2::from_elem((x, y), value),
        }
    }

    pub fn shape_type(&self) -> ShapeType {
        self.shape_type
    }

    pub fn dim(&self) -> (usize, usize) {
        self.data.dim()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn get(&self, index: (usize, usize)) -> Option<&bool> {
        self.data.get(index)
    }

    pub fn map<F>(&self, f: F) -> Self
    where
        F: FnMut(&bool) -> bool,
    {
        Shape {
            shape_type: self.shape_type,
            data: self.data.map(f),
        }
    }

    pub fn fill(&mut self, value: bool) {
        self.data.fill(value);
    }

    pub fn rotate_clockwise(&mut self) {
        match self.shape_type {
            Square => {
                self.data.reverse_axes();
                self.data.invert_axis(Axis(0));
            }
            Triangle => {
                todo!()
            }
            Hexagon => {
                todo!()
            }
        };
    }

    pub fn rotate_counterclockwise(&mut self) {
        match self.shape_type {
            Square => {
                self.data.reverse_axes();
                self.data.invert_axis(Axis(1));
            }
            Triangle => {
                todo!()
            }
            Hexagon => {
                todo!()
            }
        };
    }

    pub fn rotate_to_landscape(&mut self) {
        let dim = self.dim();
        if dim.0 < dim.1 {
            self.data.reverse_axes();
        }
    }

    pub fn flip_default(&mut self) {
        match self.shape_type {
            Square => {
                self.data.invert_axis(Axis(0));
            }
            Triangle => {
                todo!()
            }
            Hexagon => {
                todo!()
            }
        }
    }

    pub fn transpose(&mut self) {
        self.data.reverse_axes();
    }

    pub fn transposed(&self) -> Self {
        Self {
            shape_type: self.shape_type,
            data: self.data.t().into_owned(),
        }
    }

    /// Removes rows and columns from the sides of a 2D boolean array where all cells are matching`
    /// the given value.
    ///
    /// # Arguments
    ///
    /// * `to_trim`: The values to trim.
    ///
    /// returns: ()
    pub fn trim_matching(&mut self, to_trim: bool) -> TrimSides {
        let mut trim_sides = TrimSides::default();
        loop {
            if self.data.nrows() == 0 || self.data.ncols() == 0 {
                break;
            }

            let left_col_all_true = self.data.column(0).iter().all(|&cell| cell == to_trim);
            if left_col_all_true {
                self.data = self.data.slice(s![.., 1..]).to_owned();
                trim_sides.lower_y += 1;
                continue;
            }

            let right_col_all_true = self
                .data
                .column(self.data.ncols() - 1)
                .iter()
                .all(|&cell| cell == to_trim);
            if right_col_all_true {
                self.data = self.data.slice(s![.., ..self.data.ncols() - 1]).to_owned();
                trim_sides.upper_y += 1;
                continue;
            }

            let top_row_all_true = self.data.row(0).iter().all(|&cell| cell == to_trim);
            if top_row_all_true {
                self.data = self.data.slice(s![1.., ..]).to_owned();
                trim_sides.lower_x += 1;
                continue;
            }

            let bottom_row_all_true = self
                .data
                .row(self.data.nrows() - 1)
                .iter()
                .all(|&cell| cell == to_trim);
            if bottom_row_all_true {
                self.data = self.data.slice(s![..self.data.nrows() - 1, ..]).to_owned();
                trim_sides.upper_x += 1;
                continue;
            }

            break;
        }
        trim_sides
    }

    pub fn count_biggest_connected_area_of_cells_matching(&self, target_value: bool) -> usize {
        let mut visited = Array2::from_elem(self.dim(), false);
        let mut max_area = 0;

        for ((x, y), value) in self.indexed_iter() {
            if *value == target_value && !visited[[x, y]] {
                let mut area = 0;
                let mut stack = vec![(x, y)];

                while let Some((cx, cy)) = stack.pop() {
                    if cx < self.data.nrows()
                        && cy < self.data.ncols()
                        && !visited[[cx, cy]]
                        && self[(cx, cy)] == target_value
                    {
                        visited[[cx, cy]] = true;
                        area += 1;

                        // Add neighbors to the stack
                        if cx > 0 {
                            stack.push((cx - 1, cy));
                        }
                        if cx < self.data.nrows() - 1 {
                            stack.push((cx + 1, cy));
                        }
                        if cy > 0 {
                            stack.push((cx, cy - 1));
                        }
                        if cy < self.data.ncols() - 1 {
                            stack.push((cx, cy + 1));
                        }
                    }
                }

                max_area = max_area.max(area);
            }
        }

        max_area
    }

    /// Places the `child` array onto `self` at the specified offsets using a logical OR
    /// operation.
    /// This means that if either the parent or child cell is `true`, the resulting cell will be
    /// `true`.
    ///
    /// # Arguments
    ///
    /// * `child`: The child shape to be placed onto the parent.
    /// * `x_offset`: The x-axis offset for placing the child.
    /// * `y_offset`: The y-axis offset for placing the child.
    ///
    /// returns: Shape
    pub fn or_at(&self, child: &Shape, x_offset: isize, y_offset: isize) -> Shape {
        let mut new_array = self.clone();
        let child_xs = child.data.nrows();
        let child_ys = child.data.ncols();

        for x in 0..child_xs {
            for y in 0..child_ys {
                let parent_x = x as isize + x_offset;
                let parent_y = y as isize + y_offset;
                if parent_x >= 0
                    && parent_x < self.data.nrows() as isize
                    && parent_y >= 0
                    && parent_y < self.data.ncols() as isize
                {
                    new_array[(parent_x as usize, parent_y as usize)] |= child[(x, y)];
                }
            }
        }

        new_array
    }

    /// Generates all possible placements of the `child` array onto self using a logical OR
    /// operation.
    ///
    /// # Arguments
    ///
    /// * `child`: The child shape to be placed onto self.
    ///
    /// returns: Vec<Shape>
    pub fn place_on_all_positions(&self, child: &Shape) -> Vec<Shape> {
        let mut placements = Vec::new();
        let parent_rows = self.data.nrows();
        let parent_cols = self.data.ncols();
        let child_rows = child.data.nrows();
        let child_cols = child.data.ncols();

        if child_rows > parent_rows || child_cols > parent_cols {
            return placements;
        }

        for row_offset in 0..=(parent_rows - child_rows) {
            for col_offset in 0..=(parent_cols - child_cols) {
                let mut new_array = self.clone();
                let mut valid = true;
                for r in 0..child_rows {
                    for c in 0..child_cols {
                        if child[(r, c)] && self[(row_offset + r, col_offset + c)] {
                            valid = false;
                            break;
                        }
                        new_array[(row_offset + r, col_offset + c)] |= child[(r, c)];
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

    /// Removes the `true` values from `self` wherever parent has `true` values.
    ///
    /// # Arguments
    ///
    /// * `parent`: The mutable reference to the parent shape to be removed from self.
    ///
    /// returns: ()
    pub fn remove_parent(&mut self, parent: &Shape) {
        for row in 0..parent.data.nrows() {
            for col in 0..parent.data.ncols() {
                if parent[(row, col)] {
                    self[(row, col)] = false;
                }
            }
        }
    }

    /// Prints a 2D boolean array to the debug log, using '#' for `true` and '-' for `false`.
    #[allow(dead_code)]
    pub fn debug_print(&self) {
        if cfg!(debug_assertions) {
            for row in self.data.rows() {
                let row_str: String = row
                    .iter()
                    .map(|&cell| if cell { '#' } else { '-' })
                    .collect();
                println!("{}", row_str);
            }
        }
    }
}

impl Index<(usize, usize)> for Shape {
    type Output = bool;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<(usize, usize)> for Shape {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl Display for Shape {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.data.fmt(f)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShapeType {
    #[default]
    Square,
    Triangle,
    Hexagon,
}

/// Represents the number of rows and columns removed from the sides of a 2D array.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct TrimSides {
    /// The number of rows removed from the lower side of the x-axis.
    pub lower_x: usize,
    /// The number of rows removed from the higher side of the x-axis.
    pub upper_x: usize,
    /// The number of columns removed from the lower side of the y-axis.
    pub lower_y: usize,
    /// The number of columns removed from the higher side of the y-axis.
    pub upper_y: usize,
}

pub fn shape_square<const N: usize>(data: &[[bool; N]]) -> Shape {
    Shape::new(Square, arr2(data))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rotate_counterclockwise_square_size_1() {
        let mut shape = shape_square(&[[true]]);
        shape.rotate_counterclockwise();
        let expected = shape_square(&[[true]]);
        assert_eq!(expected, shape);
    }

    #[test]
    fn test_rotate_counterclockwise_square_size_2() {
        let mut shape = shape_square(&[[true, false]]);
        shape.rotate_counterclockwise();
        let expected = shape_square(&[[true], [false]]);
        assert_eq!(expected, shape);
    }

    #[test]
    fn test_rotate_counterclockwise_square() {
        let mut shape = shape_square(&[
            [true, false, false],
            [true, true, true],
            [true, false, true],
        ]);
        shape.rotate_counterclockwise();
        let expected = shape_square(&[
            [true, true, true],
            [false, true, false],
            [true, true, false],
        ]);
        assert_eq!(expected, shape);
    }

    #[test]
    fn test_trim_sides_empty() {
        let mut array = Shape::from_elem((0, 0), Square, true);
        let trim_sides = array.trim_matching(true);
        let expected = Shape::from_elem((0, 0), Square, true);
        assert_eq!(expected, array);
        let expected_trim_sides = TrimSides {
            lower_x: 0,
            lower_y: 0,
            upper_y: 0,
            upper_x: 0,
        };
        assert_eq!(expected_trim_sides, trim_sides);
    }

    #[test]
    fn test_trim_sides_true() {
        let mut array = shape_square(&[[true]]);
        let trim_sides = array.trim_matching(true);
        let expected = shape_square(&[[]]);
        assert_eq!(expected, array);
        let expected_trim_sides = TrimSides {
            lower_x: 0,
            lower_y: 1,
            upper_y: 0,
            upper_x: 0,
        };
        assert_eq!(expected_trim_sides, trim_sides);
    }

    #[test]
    fn test_trim_sides_false() {
        let mut array = shape_square(&[[false]]);
        let trim_sides = array.trim_matching(false);
        let expected = shape_square(&[[]]);
        assert_eq!(expected, array);
        let expected_trim_sides = TrimSides {
            lower_x: 0,
            lower_y: 1,
            upper_y: 0,
            upper_x: 0,
        };
        assert_eq!(expected_trim_sides, trim_sides);
    }

    #[test]
    fn test_trim_sides_lower_y_upper_y() {
        let mut array = shape_square(&[
            [true, true, false, true],
            [true, false, false, true],
            [true, true, false, true],
        ]);
        let trim_sides = array.trim_matching(true);
        let expected = shape_square(&[[true, false], [false, false], [true, false]]);
        assert_eq!(expected, array);
        let expected_trim_sides = TrimSides {
            lower_x: 0,
            lower_y: 1,
            upper_y: 1,
            upper_x: 0,
        };
        assert_eq!(expected_trim_sides, trim_sides);
    }

    #[test]
    fn test_trim_sides_lower_x_upper_x() {
        let mut array = shape_square(&[
            [true, true, true, true],
            [false, true, false, false],
            [true, true, true, true],
        ]);
        let trim_sides = array.trim_matching(true);
        let expected = shape_square(&[[false, true, false, false]]);
        assert_eq!(expected, array);
        let expected_trim_sides = TrimSides {
            lower_x: 1,
            lower_y: 0,
            upper_y: 0,
            upper_x: 1,
        };
        assert_eq!(expected_trim_sides, trim_sides);
    }

    #[test]
    fn test_trim_sides_all_sides() {
        let mut array = shape_square(&[
            [true, true, true, true, true],
            [true, true, false, false, true],
            [true, false, true, false, true],
            [true, true, true, true, true],
        ]);
        let trim_sides = array.trim_matching(true);
        let expected = shape_square(&[[true, false, false], [false, true, false]]);
        assert_eq!(expected, array);
        let expected_trim_sides = TrimSides {
            lower_x: 1,
            lower_y: 1,
            upper_y: 1,
            upper_x: 1,
        };
        assert_eq!(expected_trim_sides, trim_sides);
    }

    #[test]
    fn test_trim_sides_from_lower_y() {
        let mut array = shape_square(&[[true, true, false, false], [true, false, true, false]]);
        let trim_sides = array.trim_matching(true);
        let expected = shape_square(&[[true, false, false], [false, true, false]]);
        assert_eq!(expected, array);
        let expected_trim_sides = TrimSides {
            lower_x: 0,
            lower_y: 1,
            upper_y: 0,
            upper_x: 0,
        };
        assert_eq!(expected_trim_sides, trim_sides);
    }

    #[test]
    fn test_trim_sides_from_upper_y() {
        let mut array = shape_square(&[[false, false, true, true], [false, true, false, true]]);
        let trim_sides = array.trim_matching(true);
        let expected = shape_square(&[[false, false, true], [false, true, false]]);
        assert_eq!(expected, array);
        let expected_trim_sides = TrimSides {
            lower_x: 0,
            lower_y: 0,
            upper_y: 1,
            upper_x: 0,
        };
        assert_eq!(expected_trim_sides, trim_sides);
    }

    #[test]
    fn test_trim_sides_from_lower_x() {
        let mut array = shape_square(&[
            [true, true, true],
            [false, true, false],
            [true, false, true],
        ]);
        let trim_sides = array.trim_matching(true);
        let expected = shape_square(&[[false, true, false], [true, false, true]]);
        assert_eq!(expected, array);
        let expected_trim_sides = TrimSides {
            lower_x: 1,
            lower_y: 0,
            upper_y: 0,
            upper_x: 0,
        };
        assert_eq!(expected_trim_sides, trim_sides);
    }

    #[test]
    fn test_trim_sides_from_upper_x() {
        let mut array = shape_square(&[
            [false, true, false],
            [true, false, true],
            [true, true, true],
        ]);
        let trim_sides = array.trim_matching(true);
        let expected = shape_square(&[[false, true, false], [true, false, true]]);
        assert_eq!(expected, array);
        let expected_trim_sides = TrimSides {
            lower_x: 0,
            lower_y: 0,
            upper_y: 0,
            upper_x: 1,
        };
        assert_eq!(expected_trim_sides, trim_sides);
    }

    #[test]
    fn test_trim_sides_rows_cols_test() {
        let mut array = shape_square(&[
            [false, false, false, false],
            [false, false, false, false],
            [true, true, true, true],
            [false, true, false, true],
            [true, true, true, true],
        ]);
        let trim_sides = array.trim_matching(true);
        let expected = shape_square(&[
            [false, false, false, false],
            [false, false, false, false],
            [true, true, true, true],
            [false, true, false, true],
        ]);
        assert_eq!(expected, array);
        let expected_trim_sides = TrimSides {
            lower_x: 0,
            lower_y: 0,
            upper_y: 0,
            upper_x: 1,
        };
        assert_eq!(expected_trim_sides, trim_sides);
    }

    #[test]
    fn test_or_arrays_at() {
        let parent = shape_square(&[
            [false, false, false],
            [false, false, false],
            [false, false, false],
        ]);
        let child = shape_square(&[[true, false], [false, true]]);
        let result = parent.or_at(&child, 1, 1);
        let expected = shape_square(&[
            [false, false, false],
            [false, true, false],
            [false, false, true],
        ]);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_or_arrays_at_empty_child() {
        let parent = shape_square(&[
            [false, false, false],
            [false, false, false],
            [false, false, false],
        ]);
        let child = shape_square(&[[]]);
        let result = parent.or_at(&child, 1, 1);
        let expected = shape_square(&[
            [false, false, false],
            [false, false, false],
            [false, false, false],
        ]);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_or_arrays_at_child_1x1() {
        let parent = shape_square(&[
            [false, false, false],
            [false, false, false],
            [false, false, false],
        ]);
        let child = shape_square(&[[true]]);
        let result = parent.or_at(&child, 1, 1);
        let expected = shape_square(&[
            [false, false, false],
            [false, true, false],
            [false, false, false],
        ]);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_or_arrays_at_child_off_parent() {
        let parent = shape_square(&[
            [false, false, false],
            [false, false, false],
            [false, false, false],
        ]);
        let child = shape_square(&[[true, true], [true, true]]);
        let result = parent.or_at(&child, 2, 2);
        let expected = shape_square(&[
            [false, false, false],
            [false, false, false],
            [false, false, true],
        ]);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_or_arrays_at_true_parent() {
        let parent = shape_square(&[[true, true, true], [true, true, true], [true, true, true]]);
        let child = shape_square(&[[true, false], [true, false]]);
        let result = parent.or_at(&child, 1, 1);
        let expected = shape_square(&[[true, true, true], [true, true, true], [true, true, true]]);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_or_arrays_at_smaller_parent() {
        let parent = shape_square(&[[false, false], [false, false]]);
        let child = shape_square(&[[true, true, true], [true, true, true], [true, true, true]]);
        let result = parent.or_at(&child, 0, 0);
        let expected = shape_square(&[[true, true], [true, true]]);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_place_on_all_positions() {
        let parent = shape_square(&[
            [false, false, false],
            [false, false, false],
            [false, false, false],
        ]);
        let child = shape_square(&[[true, false], [false, true]]);
        let placements = parent.place_on_all_positions(&child);
        assert_eq!(placements.len(), 4);
        assert!(placements.contains(&shape_square(&[
            [true, false, false],
            [false, true, false],
            [false, false, false],
        ])));
        assert!(placements.contains(&shape_square(&[
            [false, true, false],
            [false, false, true],
            [false, false, false],
        ])));
        assert!(placements.contains(&shape_square(&[
            [false, false, false],
            [true, false, false],
            [false, true, false],
        ])));
        assert!(placements.contains(&shape_square(&[
            [false, false, false],
            [false, true, false],
            [false, false, true],
        ])));
    }

    #[test]
    fn test_place_on_all_positions_same_size() {
        let parent = shape_square(&[[false, false], [false, false]]);
        let child = shape_square(&[[true, false], [false, true]]);
        let placements = parent.place_on_all_positions(&child);
        assert_eq!(placements.len(), 1);
        assert!(placements.contains(&shape_square(&[[true, false], [false, true],])));
    }

    #[test]
    fn test_place_on_all_positions_smaller_parent() {
        let parent = shape_square(&[[false, false], [false, false]]);
        let child = shape_square(&[[true, false, true], [false, true, false]]);
        let placements = parent.place_on_all_positions(&child);
        assert_eq!(placements.len(), 0);
    }

    #[test]
    fn test_place_on_all_positions_with_blocking() {
        let parent = shape_square(&[
            [false, false, false],
            [false, true, false],
            [false, false, false],
        ]);
        let child = shape_square(&[[true, false], [false, true]]);
        let placements = parent.place_on_all_positions(&child);
        assert_eq!(placements.len(), 2);
        assert!(placements.contains(&shape_square(&[
            [false, true, false],
            [false, true, true],
            [false, false, false],
        ])));
        assert!(placements.contains(&shape_square(&[
            [false, false, false],
            [true, true, false],
            [false, true, false],
        ])));
    }

    #[test]
    fn test_remove_parent() {
        let parent = shape_square(&[
            [true, false, true],
            [false, true, false],
            [true, true, true],
        ]);
        let mut child = shape_square(&[[true, true, true], [true, true, true], [true, true, true]]);
        child.remove_parent(&parent);
        let expected = shape_square(&[
            [false, true, false],
            [true, false, true],
            [false, false, false],
        ]);
        assert_eq!(expected, child);
    }

    #[test]
    fn test_remove_parent_smaller_parent() {
        let parent = shape_square(&[[true, false], [false, true]]);
        let mut child = shape_square(&[[true, true, true], [true, true, true], [true, true, true]]);
        child.remove_parent(&parent);
        let expected =
            shape_square(&[[false, true, true], [true, false, true], [true, true, true]]);
        assert_eq!(expected, child);
    }

    #[test]
    #[should_panic]
    fn test_remove_parent_bigger_parent_panic() {
        let parent = shape_square(&[
            [true, false, true],
            [false, true, false],
            [true, true, true],
        ]);
        let mut child = shape_square(&[[true, true], [true, true]]);
        child.remove_parent(&parent);
    }

    #[test]
    fn test_count_biggest_connected_area_of_cells_matching() {
        let array = shape_square(&[
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
    fn test_tile_rotation_iterator() {
        let base = shape_square(&[[true, false], [false, false]]);
        let mut iter = base.rotations_flips_iter();

        assert_eq!(
            iter.next(),
            Some(shape_square(&[[true, false], [false, false]]))
        );
        assert_eq!(
            iter.next(),
            Some(shape_square(&[[false, true], [false, false]]))
        );
        assert_eq!(
            iter.next(),
            Some(shape_square(&[[false, false], [false, true]]))
        );
        assert_eq!(
            iter.next(),
            Some(shape_square(&[[false, false], [true, false]]))
        );

        assert_eq!(
            iter.next(),
            Some(shape_square(&[[true, false], [false, false]]))
        );
        assert_eq!(
            iter.next(),
            Some(shape_square(&[[false, true], [false, false]]))
        );
        assert_eq!(
            iter.next(),
            Some(shape_square(&[[false, false], [false, true]]))
        );
        assert_eq!(
            iter.next(),
            Some(shape_square(&[[false, false], [true, false]]))
        );
        assert_eq!(iter.next(), None);
    }
}
