use crate::Shape;
use ndarray::iter::{IndexedIter, Iter};
use ndarray::Ix2;

impl Shape {
    /// Iterates over the values of the shape.
    pub fn iter(&self) -> Iter<'_, bool, Ix2> {
        self.data.iter()
    }

    /// Iterates over the values of the shape while also being given the index of the value.
    pub fn indexed_iter(&self) -> IndexedIter<'_, bool, Ix2> {
        self.data.indexed_iter()
    }

    /// Iterates over all rotations of the shape.
    /// Duplicates are not removed.
    pub fn rotations_flips_iter(&self) -> ShapeRotationIterator {
        ShapeRotationIterator::new(self.clone())
    }
}

pub struct ShapeRotationIterator {
    current: Shape,
    iteration: u8,
}

impl ShapeRotationIterator {
    fn new(shape: Shape) -> Self {
        Self {
            current: shape,
            iteration: 0,
        }
    }
}

impl Iterator for ShapeRotationIterator {
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

#[cfg(test)]
mod test {
    use crate::shape::shape_square;

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
            Some(shape_square(&[[false, false], [true, false]]))
        );
        assert_eq!(
            iter.next(),
            Some(shape_square(&[[false, false], [false, true]]))
        );
        assert_eq!(
            iter.next(),
            Some(shape_square(&[[false, true], [false, false]]))
        );

        assert_eq!(
            iter.next(),
            Some(shape_square(&[[true, false], [false, false]]))
        );
        assert_eq!(
            iter.next(),
            Some(shape_square(&[[false, false], [true, false]]))
        );
        assert_eq!(
            iter.next(),
            Some(shape_square(&[[false, false], [false, true]]))
        );
        assert_eq!(
            iter.next(),
            Some(shape_square(&[[false, true], [false, false]]))
        );
        assert_eq!(iter.next(), None);
    }
}
