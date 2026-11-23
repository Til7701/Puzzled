use crate::app::components::polyform_drawing::PolyformDrawingData;
use crate::app::components::polyform_drawing::drawable::Drawable;
use puzzled_polyform::{Prototile, PrototileRef};

impl<'a> Drawable for PrototileRef<'a, PolyformDrawingData> {
    fn normalized_shape_path(&self) -> Vec<(f64, f64)> {
        match self {
            PrototileRef::Polyomino(p) => p.normalized_shape_path(),
            PrototileRef::Polyhex(p) => p.normalized_shape_path(),
        }
    }

    fn normalized_text_box(&self, dead_zone: f64) -> ((f64, f64), (f64, f64)) {
        match self {
            PrototileRef::Polyomino(p) => p.normalized_text_box(dead_zone),
            PrototileRef::Polyhex(p) => p.normalized_text_box(dead_zone),
        }
    }

    fn pixel_line_paths(&self, size: f64) -> Vec<Vec<(f64, f64)>> {
        match self {
            PrototileRef::Polyomino(p) => p.pixel_line_paths(size),
            PrototileRef::Polyhex(p) => p.pixel_line_paths(size),
        }
    }

    fn drawing_data(&self) -> &PolyformDrawingData {
        self.data()
    }
}
