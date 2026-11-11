use crate::Polyform;
use crate::coord::hex::HexCoord;
use crate::coord::regular::RegularCoord;
use crate::poly::Poly;
use crate::prototile::{Hexagon, PrototileMutRef, PrototileRef, Square};
use std::hash::Hash;
use std::slice::IterMut;

impl<T> Polyform<T>
where
    T: Clone + Hash,
{
    pub fn iter(&self) -> PolyformIter<'_, T> {
        PolyformIter {
            index: 0,
            polyform: self,
        }
    }

    pub fn iter_mut(&mut self) -> PolyformIterMut<'_, T> {
        match self {
            Polyform::Polyomino(polyomino) => {
                PolyformIterMut::Polyomino(polyomino.cells_mut().iter_mut())
            }
            Polyform::Hexomino(hexomino) => {
                PolyformIterMut::Hexomino(hexomino.cells_mut().iter_mut())
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
    T: Clone + Hash,
{
    index: usize,
    polyform: &'a Polyform<T>,
}

impl<'a, T> Iterator for PolyformIter<'a, T>
where
    T: Clone + Hash,
{
    type Item = PrototileRef<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        let v = match self.polyform {
            Polyform::Polyomino(polyomino) => polyomino.cells().get(self.index).map(|p| p.into()),
            Polyform::Hexomino(hexomino) => hexomino.cells().get(self.index).map(|p| p.into()),
        };
        self.index += 1;
        v
    }
}

pub enum PolyformIterMut<'a, T>
where
    T: Clone + Hash,
{
    Polyomino(IterMut<'a, Square<T, RegularCoord>>),
    Hexomino(IterMut<'a, Hexagon<T, HexCoord>>),
}

impl<'a, T> Iterator for PolyformIterMut<'a, T>
where
    T: Clone + Hash,
{
    type Item = PrototileMutRef<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Polyomino(iter) => iter.next().map(|s| s.into()),
            Self::Hexomino(iter) => iter.next().map(|h| h.into()),
        }
    }
}

pub struct PolyformRotationFlipsIter<T>
where
    T: Clone + Hash,
{
    index: usize,
    polyform: Polyform<T>,
}

impl<T> Iterator for PolyformRotationFlipsIter<T>
where
    T: Clone + Hash,
{
    type Item = Polyform<T>;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.polyform {
            Polyform::Polyomino(_) => {
                let polyform = &mut self.polyform;
                let v = if self.index >= 8 {
                    None
                } else if self.index == 4 {
                    polyform.flip(RegularCoord::new(0, 0).into());
                    Some(polyform.clone())
                } else {
                    polyform.rotate_around(RegularCoord::new(0, 0).into());
                    Some(polyform.clone())
                };
                self.index += 1;
                v
            }
            Polyform::Hexomino(_) => {
                todo!()
            }
        }
    }
}
