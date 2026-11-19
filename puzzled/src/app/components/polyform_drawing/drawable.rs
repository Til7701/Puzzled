pub trait Drawable {
    /// The bounding outline of the shape to draw as a path.
    /// Coordinates must be between 0 and 1
    fn shape_path(&self) -> &[(f64, f64)];

    /// A path for drawing an outline of the shape to draw as a path.
    /// Coordinates must be between 0 and 1
    fn outline_path(&self, width: f64) -> &[(f64, f64)];

    /// The bounding box for text to draw into this shape
    fn text_box(&self, dead_zone: f64) -> ((f64, f64), (f64, f64));
}
