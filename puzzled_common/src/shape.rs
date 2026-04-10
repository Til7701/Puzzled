use crate::ShapeType::Square;
use ndarray::iter::{IndexedIter, Iter};
use ndarray::{arr2, s, Array2, Axis, Ix2};
use std::ops::Index;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Shape {
    shape_type: ShapeType,
    data: Array2<bool>,
}

impl Shape {
    pub fn new(shape_type: ShapeType, data: Array2<bool>) -> Self {
        Self { shape_type, data }
    }

    pub fn shape_type(&self) -> ShapeType {
        self.shape_type
    }

    pub fn dim(&self) -> (usize, usize) {
        self.data.dim()
    }

    pub fn get(&self, index: (usize, usize)) -> Option<&bool> {
        self.data.get(index)
    }

    pub fn iter(&self) -> Iter<'_, bool, Ix2> {
        self.data.iter()
    }

    pub fn indexed_iter(&self) -> IndexedIter<'_, bool, Ix2> {
        self.data.indexed_iter()
    }

    pub fn rotate_clockwise(&mut self) {
        match self.shape_type {
            ShapeType::Square => {
                self.data.reverse_axes();
                self.data.invert_axis(Axis(1));
            }
            ShapeType::Triangle => {
                todo!()
            }
            ShapeType::Hexagon => {
                todo!()
            }
        };
    }

    pub fn flip_default(&mut self) {
        match self.shape_type {
            ShapeType::Square => {
                self.data.invert_axis(Axis(0));
            }
            ShapeType::Triangle => {
                todo!()
            }
            ShapeType::Hexagon => {
                todo!()
            }
        }
    }

    pub fn trim_matching(&mut self, to_trim: bool) -> TrimSides {
        let mut array = self.data.clone();
        let mut trim_sides = TrimSides::default();
        loop {
            if array.nrows() == 0 || array.ncols() == 0 {
                break;
            }

            let left_col_all_true = array.column(0).iter().all(|&cell| cell == to_trim);
            if left_col_all_true {
                array = array.slice(s![.., 1..]).to_owned();
                trim_sides.lower_y += 1;
                continue;
            }

            let right_col_all_true = array
                .column(array.ncols() - 1)
                .iter()
                .all(|&cell| cell == to_trim);
            if right_col_all_true {
                array = array.slice(s![.., ..array.ncols() - 1]).to_owned();
                trim_sides.upper_y += 1;
                continue;
            }

            let top_row_all_true = array.row(0).iter().all(|&cell| cell == to_trim);
            if top_row_all_true {
                array = array.slice(s![1.., ..]).to_owned();
                trim_sides.lower_x += 1;
                continue;
            }

            let bottom_row_all_true = array
                .row(array.nrows() - 1)
                .iter()
                .all(|&cell| cell == to_trim);
            if bottom_row_all_true {
                array = array.slice(s![..array.nrows() - 1, ..]).to_owned();
                trim_sides.upper_x += 1;
                continue;
            }

            break;
        }
        trim_sides
    }
}

impl Index<(usize, usize)> for Shape {
    type Output = bool;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index]
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
