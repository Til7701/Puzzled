use crate::app::components::polyform_drawing::drawable::Drawable;
use puzzled_polyform::{Coord, Square};
use std::hash::Hash;

impl<T, C> Drawable for Square<T, C>
where
    T: Clone + Hash,
    C: Coord,
{
    fn shape_path(&self) -> &[(f64, f64)] {
        static PATH: [(f64, f64); 4] = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
        &PATH
    }

    fn outline_path(&self, width: f64) -> &[(f64, f64)] {
        todo!()
    }

    fn text_box(&self, dead_zone: f64) -> ((f64, f64), (f64, f64)) {
        todo!()
    }
}
