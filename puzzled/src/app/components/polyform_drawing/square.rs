use crate::app::components::polyform_drawing::PolyformDrawingData;
use crate::app::components::polyform_drawing::drawable::Drawable;
use puzzled_polyform::{Coord, Prototile, Square, SquareOrientation};

impl<C> Drawable for Square<PolyformDrawingData, C>
where
    C: Coord,
{
    fn normalized_shape_path(&self) -> Vec<(f64, f64)> {
        match self.orientation() {
            SquareOrientation::OnSide => vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
            SquareOrientation::OnCorner => {
                todo!()
            }
        }
    }

    fn normalized_text_box(&self, dead_zone: f64) -> ((f64, f64), (f64, f64)) {
        match self.orientation() {
            SquareOrientation::OnSide => (
                (dead_zone, dead_zone),
                (1.0 - dead_zone - dead_zone, 1.0 - dead_zone - dead_zone),
            ),
            SquareOrientation::OnCorner => {
                todo!()
            }
        }
    }

    fn drawing_data(&self) -> &PolyformDrawingData {
        self.data()
    }
}
