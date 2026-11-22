use crate::app::components::polyform_drawing::drawable::Drawable;
use crate::app::components::polyform_drawing::{PolyformDrawingData, PolyformDrawingDataSpecial};
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

    fn pixel_line_paths(&self, size: f64) -> Vec<Vec<(f64, f64)>> {
        let special = match &self.data().special {
            PolyformDrawingDataSpecial::Polyomino(polyomino_special) => polyomino_special,
            _ => unreachable!(),
        };

        let mut paths = Vec::with_capacity(2);
        if special.x_pos_neighbour {
            paths.push(vec![
                (size - 1.0, 0.0),
                (size - 1.0, size),
                (size + 1.0, size),
                (size + 1.0, 0.0),
            ]);
        }
        if special.y_pos_neighbour {
            paths.push(vec![
                (0.0, size - 1.0),
                (size, size - 1.0),
                (size, size + 1.0),
                (0.0, size + 1.0),
            ]);
        };
        paths
    }

    fn drawing_data(&self) -> &PolyformDrawingData {
        self.data()
    }
}
