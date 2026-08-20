mod iterators;
mod prototile;

use crate::ShapeType::*;
use ndarray::{Array2, arr2};
use std::fmt::{Display, Formatter, Pointer};
use std::ops::Index;

pub type Shape = Polyform;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Polyform {
    Polyomino {
        dim: (usize, usize),
        data: Vec<prototile::Square>,
    },
    Hexomino {
        data: Vec<prototile::Hexagon>
    },
}

impl Polyform {
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
    #[deprecated]
    pub fn new(shape_type: ShapeType, data: Array2<bool>) -> Self {
        match shape_type {
            Square => {
                let dim = data.dim();
                let squares = data.indexed_iter().filter_map(|((i, j), e)| if *e {
                    Some(prototile::Square::new(i as u32, j as u32))
                } else {
                    None
                }).collect();
                Polyform::Polyomino { dim, data: squares }
            }
            Triangle => { todo!() }
            Hexagon => { todo!() }
        }
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
    /// assert_eq!(shape.get((0, 0)), Some(&true));
    /// assert_eq!(shape.get((0, 1)), Some(&true));
    /// ```
    #[deprecated]
    pub fn from_elem((x, y): (usize, usize), shape_type: ShapeType, value: bool) -> Self {
        Self::new(shape_type, Array2::from_elem((x, y), value))
    }

    /// Returns the shape type of this shape
    ///
    /// # Examples
    ///
    /// ```
    /// use ndarray::arr2;
    /// use puzzled_common::Shape;
    /// use puzzled_common::ShapeType;
    ///
    /// let shape = Shape::new(ShapeType::Square, arr2(&[[true]]));
    ///
    /// assert_eq!(shape.shape_type(), ShapeType::Square);
    /// ```
    #[deprecated]
    pub fn shape_type(&self) -> ShapeType {
        match self {
            Polyform::Polyomino { .. } => { Square }
            Polyform::Hexomino { .. } => { Hexagon }
        }
    }

    /// Returns the dimensions of this shape
    ///
    /// # Examples
    ///
    /// ```
    /// use ndarray::arr2;
    /// use puzzled_common::Shape;
    /// use puzzled_common::ShapeType;
    ///
    /// let shape = Shape::new(ShapeType::Square, arr2(&[[true, true]]));
    ///
    /// assert_eq!(shape.dim(), (1, 2));
    /// ```
    pub fn dim(&self) -> (usize, usize) {
        match self {
            Polyform::Polyomino { dim, .. } => { *dim }
            Polyform::Hexomino { .. } => { todo!() }
        }
    }

    /// Returns the number of cells in the shape
    ///
    /// # Examples
    ///
    /// ```
    /// use ndarray::arr2;
    /// use puzzled_common::Shape;
    /// use puzzled_common::ShapeType;
    ///
    /// let shape = Shape::new(ShapeType::Square, arr2(&[[true, true], [false, true]]));
    ///
    /// assert_eq!(shape.len(), 4);
    /// ```
    pub fn len(&self) -> usize {
        match self {
            Polyform::Polyomino { dim, .. } => { dim.0 * dim.1 }
            Polyform::Hexomino { .. } => { todo!() }
        }
    }

    /// Returns true, if the shape does not have any cells.
    ///
    /// # Examples
    ///
    /// ```
    /// use ndarray::arr2;
    /// use puzzled_common::Shape;
    /// use puzzled_common::ShapeType;
    ///
    /// let shape = Shape::new(ShapeType::Square, arr2(&[[true, true], [false, true]]));
    /// assert_eq!(shape.is_empty(), false);
    /// let shape = Shape::new(ShapeType::Square, arr2(&[[]]));
    /// assert_eq!(shape.is_empty(), true);
    /// ```
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the value of the cell at the given position in the shape or none, if the index
    /// is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use ndarray::arr2;
    /// use puzzled_common::Shape;
    /// use puzzled_common::ShapeType;
    ///
    /// let shape = Shape::new(ShapeType::Square, arr2(&[[true, false]]));
    ///
    /// assert_eq!(shape.get((0, 0)), Some(&true));
    /// assert_eq!(shape.get((0, 1)), Some(&false));
    /// assert_eq!(shape.get((1, 1)), None);
    /// ```
    pub fn get(&self, index: (usize, usize)) -> Option<&bool> {
        match self {
            Polyform::Polyomino { dim, data } => {
                if index.0 >= dim.1 || index.1 >= dim.1 {
                    None
                } else {
                    data.iter().find(|square| square.x() == index.0 as u32 && square.y() == index.1 as u32)
                        .map(|_| &true)
                        .or(Some(&false))
                }
            }
            Polyform::Hexomino { .. } => { todo!() }
        }
    }

    /// Maps all values in the shape and returns a new shape.
    ///
    /// # Arguments
    ///
    /// * `f`: the mapping function
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
    /// let expected = Shape::new(ShapeType::Square, arr2(&[[false, true], [true, false]]));
    ///
    /// assert_eq!(shape.map(|v| !v), expected);
    /// ```
    pub fn map<F>(&self, mut f: F) -> Self
    where
        F: FnMut(&bool) -> bool,
    {
        match self {
            Polyform::Polyomino { dim, data } => {
                let mut new_squares = Vec::new();
                for i in 0..dim.0 {
                    for j in 0..dim.1 {
                        let old = data.iter().find(|square| square.x() == i as u32 && square.y() == j as u32).is_some();
                        let new = f(&old);
                        if new {
                            new_squares.push(prototile::Square::new(i as u32, j as u32));
                        }
                    }
                }
                Polyform::Polyomino { dim: *dim, data: new_squares }
            }
            Polyform::Hexomino { .. } => { todo!() }
        }
    }

    /// Fills the shape with the given value.
    ///
    /// # Arguments
    ///
    /// * `value`: the value to set all cells to
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    /// use ndarray::arr2;
    /// use puzzled_common::Shape;
    /// use puzzled_common::ShapeType;
    ///
    /// let mut shape = Shape::new(ShapeType::Square, arr2(&[[true, false], [false, true]]));
    /// shape.fill(false);
    /// let expected = Shape::new(ShapeType::Square, arr2(&[[false, false], [false, false]]));
    ///
    /// assert_eq!(shape, expected);
    /// ```
    pub fn fill(&mut self, value: bool) {
        match self {
            Polyform::Polyomino { dim, data } => {
                data.clear();
                if value {
                    for i in 0..dim.0 {
                        for j in 0..dim.1 {
                            data.push(prototile::Square::new(i as u32, j as u32));
                        }
                    }
                }
            }
            Polyform::Hexomino { .. } => { todo!() }
        }
    }

    /// Rotates the shape counterclockwise.
    /// This rotates by a different angle for each shape type, but is always the next viable
    /// angle.
    ///
    /// # Examples
    ///
    /// ```
    /// use ndarray::arr2;
    /// use puzzled_common::Shape;
    /// use puzzled_common::ShapeType;
    ///
    /// let mut shape = Shape::new(ShapeType::Square, arr2(&[[true, false], [false, true]]));
    /// shape.rotate_counterclockwise();
    /// let expected = Shape::new(ShapeType::Square, arr2(&[[false, true], [true, false]]));
    ///
    /// assert_eq!(shape, expected);
    /// ```
    pub fn rotate_counterclockwise(&mut self) {
        match self {
            Polyform::Polyomino { dim, data } => {
                let new_dim = (dim.1, dim.0);
                for square in data {
                    square.rotate_counterclockwise((dim.0 as u32, dim.1 as u32));
                }
                *dim = new_dim;
            }
            Polyform::Hexomino { .. } => { todo!() }
        }
    }

    /// Rotates the shape to landscape.
    /// # Examples
    ///
    /// ```
    /// use puzzled_common::shape::shape_square;
    /// use puzzled_common::Shape;
    /// use puzzled_common::ShapeType;
    ///
    /// let mut shape = shape_square(&[[true, false]]);
    /// shape.rotate_to_landscape();
    /// let expected = shape_square(&[[true], [false]]);
    /// assert_eq!(expected, shape);
    /// ```
    pub fn rotate_to_landscape(&mut self) {
        let dim = self.dim();
        if dim.0 < dim.1 {
            self.rotate_counterclockwise();
        }
    }

    /// Flips the shape by a default axis for the shape type.
    ///
    /// # Examples
    ///
    /// ```
    /// use puzzled_common::shape::shape_square;
    /// use puzzled_common::Shape;
    /// use puzzled_common::ShapeType;
    ///
    /// let mut shape = shape_square(&[[false, true], [false, false]]);
    /// shape.flip_default();
    /// let expected = shape_square(&[[false, false], [false, true]]);
    /// assert_eq!(expected, shape);
    /// ```
    pub fn flip_default(&mut self) {
        match self {
            Polyform::Polyomino { dim, data } => {
                for square in data {
                    square.flip_default((dim.0 as u32, dim.1 as u32));
                }
            }
            Polyform::Hexomino { .. } => { todo!() }
        }
    }

    /// Transposes the shape.
    ///
    /// # Examples
    ///
    /// ```
    /// use puzzled_common::shape::shape_square;
    /// use puzzled_common::Shape;
    /// use puzzled_common::ShapeType;
    ///
    /// let mut shape = shape_square(&[[false, true], [false, false]]);
    /// shape.transpose();
    /// let expected = shape_square(&[[false, false], [true, false]]);
    /// assert_eq!(expected, shape);
    /// ```
    pub fn transpose(&mut self) {
        match self {
            Polyform::Polyomino { dim, data } => {
                for square in data {
                    square.transpose((dim.0 as u32, dim.1 as u32));
                }
            }
            Polyform::Hexomino { .. } => { todo!() }
        }
    }

    pub fn transposed(&self) -> Self {
        let mut clone = self.clone();
        clone.transpose();
        clone
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
        if to_trim {
            self.map(|old| !old);
        }

        let trim_sides = match self {
            Polyform::Polyomino { dim, data } => {
                Self::polyomino_trim(dim, data)
            }
            Polyform::Hexomino { .. } => { todo!() }
        };

        if to_trim {
            self.map(|old| !old);
        }

        trim_sides
    }

    fn polyomino_trim(dim: &mut (usize, usize), data: &mut Vec<prototile::Square>) -> TrimSides {
        let mut trim_sides = TrimSides::default();
        if dim.0 == 0 || dim.1 == 0 {
            return trim_sides;
        }

        let min_y = data.iter().map(|square| square.y()).min().unwrap();
        data.iter_mut().for_each(|square| { square.set_y(square.y() - min_y) });
        dim.1 = dim.1 - min_y as usize;
        trim_sides.lower_y += min_y as usize;

        let max_y = data.iter().map(|square| square.y()).max().unwrap();
        dim.1 = max_y as usize;
        trim_sides.upper_y += max_y as usize;

        let min_x = data.iter().map(|square| square.x()).min().unwrap();
        data.iter_mut().for_each(|square| { square.set_x(square.x() - min_x) });
        dim.1 = dim.1 - min_x as usize;
        trim_sides.lower_x += min_x as usize;

        let max_x = data.iter().map(|square| square.x()).max().unwrap();
        dim.0 = max_x as usize;
        trim_sides.upper_x += max_x as usize;

        trim_sides
    }

    pub fn count_biggest_connected_area_of_cells_matching(&self, target_value: bool) -> usize {
        match self {
            Polyform::Polyomino { .. } => {
                self.polyomino_count_biggest_connected_area_of_cells_matching(target_value)
            }
            Polyform::Hexomino { .. } => { todo!() }
        }
    }

    fn polyomino_count_biggest_connected_area_of_cells_matching(&self, target_value: bool) -> usize {
        let mut visited = Array2::from_elem(self.dim(), false);
        let mut max_area = 0;

        for ((x, y), value) in self.indexed_iter() {
            if *value == target_value && !visited[[x, y]] {
                let mut area = 0;
                let mut stack = vec![(x, y)];

                while let Some((cx, cy)) = stack.pop() {
                    if cx < self.dim().0
                        && cy < self.dim().1
                        && !visited[[cx, cy]]
                        && self[(cx, cy)] == target_value
                    {
                        visited[[cx, cy]] = true;
                        area += 1;

                        // Add neighbors to the stack
                        if cx > 0 {
                            stack.push((cx - 1, cy));
                        }
                        if cx < self.dim().0 - 1 {
                            stack.push((cx + 1, cy));
                        }
                        if cy > 0 {
                            stack.push((cx, cy - 1));
                        }
                        if cy < self.dim().1 - 1 {
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
    pub fn or_at(&self, child: &Self, x_offset: isize, y_offset: isize) -> Self {
        let mut new_array = self.clone();
        let (child_xs, child_ys) = child.dim();

        for x in 0..child_xs {
            for y in 0..child_ys {
                let parent_x = x as isize + x_offset;
                let parent_y = y as isize + y_offset;
                if parent_x >= 0
                    && parent_x < child_xs as isize
                    && parent_y >= 0
                    && parent_y < child_ys as isize
                {
                    if child[(x, y)] {
                        new_array.ensure_presence((parent_x as usize, parent_y as usize));
                    }
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
    pub fn place_on_all_positions(&self, child: &Self) -> Vec<Self> {
        let mut placements = Vec::new();
        let (parent_rows, parent_cols) = self.dim();
        let (child_rows, child_cols) = child.dim();

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
                        if child[(r, c)] {
                            new_array.ensure_presence((row_offset + r, col_offset + c));
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

    /// Removes the `true` values from `self` wherever parent has `true` values.
    ///
    /// # Arguments
    ///
    /// * `parent`: The mutable reference to the parent shape to be removed from self.
    ///
    /// returns: ()
    pub fn remove_parent(&mut self, parent: &Self) {
        for row in 0..parent.dim().0 {
            for col in 0..parent.dim().1 {
                if parent[(row, col)] {
                    self.remove((row, col));
                }
            }
        }
    }

    fn remove(&mut self, index: (usize, usize)) {
        match self {
            Polyform::Polyomino { data, .. } => {
                let _ = data.extract_if(.., |square| square.x() == index.0 as u32 && square.y() == index.1 as u32);
            }
            Polyform::Hexomino { .. } => { todo!() }
        }
    }

    fn ensure_presence(&mut self, index: (usize, usize)) {
        let contains = self.get(index).is_some();
        if !contains {
            match self {
                Polyform::Polyomino { data, .. } => {
                    data.push(prototile::Square::new(index.0 as u32, index.1 as u32));
                }
                Polyform::Hexomino { .. } => { todo!() }
            }
        }
    }

    /// Prints a 2D boolean array to the debug log, using '#' for `true` and '-' for `false`.
    #[allow(dead_code)]
    pub fn debug_print(&self) {
        if cfg!(debug_assertions) {
            for i in 0..self.dim().0 {
                let mut row = String::new();
                for j in 0..self.dim().1 {
                    let char = if self.get((i, j)).is_some() {
                        '#'
                    } else {
                        '-'
                    };
                    row.push(char);
                }
                println!("{}", row);
            }
        }
    }
}

impl Index<(usize, usize)> for Polyform {
    type Output = bool;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.get(index).unwrap()
    }
}

impl Display for Polyform {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Polyform::Polyomino { dim, data } => {
                dim.fmt(f).and(data.fmt(f))
            }
            Polyform::Hexomino { .. } => { todo!() }
        }
    }
}

#[deprecated]
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

#[deprecated]
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
        let expected = shape_square(&[[false], [true]]);
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
            [false, true, true],
            [false, true, false],
            [true, true, true],
        ]);
        assert_eq!(expected, shape);
    }

    #[test]
    fn test_rotate_to_landscape_empty() {
        let mut shape = shape_square(&[[]]);
        shape.rotate_to_landscape();
        let expected = shape_square(&[[]]);
        assert_eq!(expected, shape);
    }

    #[test]
    fn test_rotate_to_landscape_one() {
        let mut shape = shape_square(&[[true]]);
        shape.rotate_to_landscape();
        let expected = shape_square(&[[true]]);
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
    fn test_count_biggest_connected_area_of_cells_matching_empty() {
        let shape = shape_square(&[[]]);

        let count_true = shape.count_biggest_connected_area_of_cells_matching(true);
        let count_false = shape.count_biggest_connected_area_of_cells_matching(false);

        assert_eq!(count_true, 0);
        assert_eq!(count_false, 0);
    }

    #[test]
    fn test_count_biggest_connected_area_of_cells_matching_true() {
        let shape = shape_square(&[[true]]);

        let count_true = shape.count_biggest_connected_area_of_cells_matching(true);
        let count_false = shape.count_biggest_connected_area_of_cells_matching(false);

        assert_eq!(count_true, 1);
        assert_eq!(count_false, 0);
    }

    #[test]
    fn test_count_biggest_connected_area_of_cells_matching_false() {
        let shape = shape_square(&[[false]]);

        let count_true = shape.count_biggest_connected_area_of_cells_matching(true);
        let count_false = shape.count_biggest_connected_area_of_cells_matching(false);

        assert_eq!(count_true, 0);
        assert_eq!(count_false, 1);
    }

    #[test]
    fn test_count_biggest_connected_area_of_cells_matching_ring() {
        let shape = shape_square(&[[true, true, true], [true, false, true], [true, true, true]]);

        let count_true = shape.count_biggest_connected_area_of_cells_matching(true);
        let count_false = shape.count_biggest_connected_area_of_cells_matching(false);

        assert_eq!(count_true, 8);
        assert_eq!(count_false, 1);
    }

    #[test]
    fn test_count_biggest_connected_area_of_cells_matching_complex() {
        let shape = shape_square(&[
            [true, true, true, false, true],
            [true, false, true, false, true],
            [true, true, true, false, true],
        ]);

        let count_true = shape.count_biggest_connected_area_of_cells_matching(true);
        let count_false = shape.count_biggest_connected_area_of_cells_matching(false);

        assert_eq!(count_true, 8);
        assert_eq!(count_false, 3);
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
        assert!(placements.contains(&shape_square(&[[true, false], [false, true], ])));
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
}
