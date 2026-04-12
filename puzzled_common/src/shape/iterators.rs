use crate::Shape;
use ndarray::iter::{IndexedIter, Iter};
use ndarray::Ix2;

impl Shape {
    pub fn iter(&self) -> Iter<'_, bool, Ix2> {
        self.data.iter()
    }

    pub fn indexed_iter(&self) -> IndexedIter<'_, bool, Ix2> {
        self.data.indexed_iter()
    }

    pub fn rotations_flips_iter(&self) -> TileRotationIterator {
        TileRotationIterator::new(self.clone())
    }
}

pub struct TileRotationIterator {
    current: Shape,
    iteration: u8,
}

impl TileRotationIterator {
    fn new(tile: Shape) -> Self {
        Self {
            current: tile,
            iteration: 0,
        }
    }
}

impl Iterator for TileRotationIterator {
    type Item = Shape;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iteration >= 8 {
            return None;
        }
        if self.iteration == 4 {
            self.current.transpose();
        }
        let current = self.current.clone();
        self.current.rotate_counterclockwise();
        self.iteration += 1;
        Some(current)
    }
}
