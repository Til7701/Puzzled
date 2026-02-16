use ndarray::Array2;

#[derive(Debug, Default)]
pub struct Contour {
    vertices_lists: Vec<Vec<(f32, f32)>>,
}

impl Contour {
    pub fn vertices_lists(&self) -> &Vec<Vec<(f32, f32)>> {
        &self.vertices_lists
    }
}

/// Extract contours (closed polygons) surrounding connected `true` cells.
/// Coordinates use the integer grid: a cell at `(x,y)` covers `[x,x+1] x [y,y+1]`.
pub fn contours_from_array(arr: &Array2<bool>) -> Contour {
    let mut contour = Contour::default();
    let mut visited = Array2::from_elem(arr.dim(), false);

    for x in 0..arr.dim().0 {
        for y in 0..arr.dim().1 {
            if arr[[x, y]] && !visited[[x, y]] {
                let vertices = trace_contour(arr, &mut visited, (x, y));
                contour.vertices_lists.push(vertices);
            }
        }
    }

    contour
}

fn trace_contour(
    arr: &Array2<bool>,
    visited: &mut Array2<bool>,
    start: (usize, usize),
) -> Vec<(f32, f32)> {
    let mut vertices = Vec::new();

    // TODO implement

    vertices
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::arr2;

    #[test]
    fn test_contours_simple() {
        let a = arr2(&[[true]]);
        let contour = contours_from_array(&a);
        assert_eq!(1, contour.vertices_lists().len());
        assert_eq!(
            vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
            contour.vertices_lists()[0]
        );
    }

    #[test]
    fn test_contours_hole() {
        let a = arr2(&[[true, true, true], [true, false, true], [true, true, true]]);
        let contour = contours_from_array(&a);
        assert_eq!(2, contour.vertices_lists().len());
        assert_eq!(
            vec![(0.0, 0.0), (3.0, 0.0), (3.0, 3.0), (0.0, 3.0)],
            contour.vertices_lists()[0]
        );
        // FIXME: define the hole contour in a way to tell the drawing function to draw it as a hole.
        assert_eq!(
            vec![(1.0, 1.0), (2.0, 1.0), (2.0, 2.0), (1.0, 2.0)],
            contour.vertices_lists()[1]
        );
    }

    #[test]
    fn test_contours_disconnected() {
        let a = arr2(&[
            [true, false, true],
            [false, false, false],
            [true, false, true],
        ]);
        let contour = contours_from_array(&a);
        assert_eq!(4, contour.vertices_lists().len());
        assert_eq!(
            vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
            contour.vertices_lists()[0]
        );
        assert_eq!(
            vec![(2.0, 0.0), (3.0, 0.0), (3.0, 1.0), (2.0, 1.0)],
            contour.vertices_lists()[1]
        );
        assert_eq!(
            vec![(0.0, 2.0), (1.0, 2.0), (1.0, 3.0), (0.0, 3.0)],
            contour.vertices_lists()[2]
        );
        assert_eq!(
            vec![(2.0, 2.0), (3.0, 2.0), (3.0, 3.0), (2.0, 3.0)],
            contour.vertices_lists()[3]
        );
    }
}
