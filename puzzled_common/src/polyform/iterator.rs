use crate::polyform::Polyform;
use crate::polyform::prototile::{Hexagon, PrototileMutRef, PrototileRef, Square};
use std::slice::IterMut;

impl<T> Polyform<T>
where
    T: Clone,
{
    pub fn iter(&self) -> PolyformIter<'_, T> {
        PolyformIter {
            index: 0,
            polyform: self,
        }
    }

    pub fn iter_mut(&mut self) -> PolyformIterMut<'_, T> {
        match self {
            Polyform::Polyomino { cells, .. } => {
                PolyformIterMut::Squares(cells.iter_mut())
            }
            Polyform::Hexomino { cells, .. } => {
                PolyformIterMut::Hexagons(cells.iter_mut())
            }
        }
    }

    pub fn rotations_flips_iter(&self) -> PolyformRotationFlipsIter<T> {
        PolyformRotationFlipsIter {
            index: 0,
            polyform: self.clone(),
        }
    }
}

pub struct PolyformIter<'a, T>
where
    T: Clone,
{
    index: usize,
    polyform: &'a Polyform<T>,
}

impl<'a, T> Iterator for PolyformIter<'a, T>
where
    T: Clone,
{
    type Item = PrototileRef<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.polyform {
            Polyform::Polyomino { cells, .. } => {
                cells.get(self.index).map(|p| p.into())
            }
            Polyform::Hexomino { cells, .. } => {
                cells.get(self.index).map(|p| p.into())
            }
        }
    }
}

pub enum PolyformIterMut<'a, T>
where
    T: Clone,
{
    Squares(IterMut<'a, Square<T>>),
    Hexagons(IterMut<'a, Hexagon<T>>),
}

impl<'a, T> Iterator for PolyformIterMut<'a, T>
where
    T: Clone,
{
    type Item = PrototileMutRef<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Squares(iter) => iter.next().map(|s| s.into()),
            Self::Hexagons(iter) => iter.next().map(|h| h.into()),
        }
    }
}

pub struct PolyformRotationFlipsIter<T>
where
    T: Clone,
{
    index: usize,
    polyform: Polyform<T>,
}

impl<T> Iterator for PolyformRotationFlipsIter<T>
where
    T: Clone,
{
    type Item = Polyform<T>;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.polyform {
            Polyform::Polyomino { .. } => {
                let polyform = &mut self.polyform;
                if self.index >= 8 {
                    None
                } else if self.index == 4 {
                    polyform.flip_horizontally();
                    Some(polyform.clone())
                } else {
                    polyform.rotate_clockwise();
                    Some(polyform.clone())
                }
            }
            Polyform::Hexomino { .. } => {
                todo!()
            }
        }
    }
}
