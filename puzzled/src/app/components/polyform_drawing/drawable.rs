use crate::app::components::polyform_drawing::PolyformDrawingData;

pub trait Drawable {
    /// The bounding outline of the shape to draw as a path.
    /// Coordinates must be between 0 and 1
    fn normalized_shape_path(&self) -> Vec<(f64, f64)>;

    /// The bounding box for text to draw into this shape
    fn normalized_text_box(&self, dead_zone: f64) -> ((f64, f64), (f64, f64));

    fn drawing_data(&self) -> &PolyformDrawingData;
}
