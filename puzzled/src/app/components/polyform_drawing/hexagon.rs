use crate::app::components::polyform_drawing::PolyformDrawingData;
use crate::app::components::polyform_drawing::drawable::Drawable;
use puzzled_polyform::{Coord, Hexagon, HexagonOrientation, Prototile};
use std::f64::consts::PI;

impl<C> Drawable for Hexagon<PolyformDrawingData, C>
where
    C: Coord,
{
    fn normalized_shape_path(&self) -> Vec<(f64, f64)> {
        match self.orientation() {
            HexagonOrientation::OnSide => {
                let mut v = Vec::with_capacity(6);
                for i in 0..6 {
                    let angle = i as f64 * PI / 3.0;
                    v.push((0.5 + (angle.cos() / 2.0), 0.5 + (angle.sin() / 2.0)));
                }
                v
            }
            HexagonOrientation::OnCorner => {
                let mut v = Vec::with_capacity(6);
                for i in 0..6 {
                    let angle = (i as f64 * PI / 3.0) + PI / 6.0;
                    v.push((0.5 + (angle.cos() / 2.0), 0.5 + (angle.sin() / 2.0)));
                }
                v
            }
        }
    }

    fn normalized_text_box(&self, dead_zone: f64) -> ((f64, f64), (f64, f64)) {
        match self.orientation() {
            HexagonOrientation::OnSide => (
                (dead_zone, dead_zone),
                (1.0 - dead_zone - dead_zone, 1.0 - dead_zone - dead_zone),
            ),
            HexagonOrientation::OnCorner => (
                (dead_zone, dead_zone),
                (1.0 - dead_zone - dead_zone, 1.0 - dead_zone - dead_zone),
            ),
        }
    }

    fn pixel_line_paths(&self, size: f64) -> Vec<Vec<(f64, f64)>> {
        vec![]
    }

    fn drawing_data(&self) -> &PolyformDrawingData {
        self.data()
    }
}
