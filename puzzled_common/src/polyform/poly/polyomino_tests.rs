use crate::polyform::coord::regular::RegularCoord;
use crate::polyform::poly::polyomino::Polyomino;
use crate::polyform::poly::{Poly, TrimSides, UnitPoly};
use crate::polyform::prototile::{Square, SquareOrientation};

#[test]
fn test_default() {
    let polyform = Polyomino::<()>::default();
    assert_eq!(RegularCoord::new(0, 0), polyform.dim());
    assert!(polyform.cells().is_empty());
}

#[test]
fn test_from_bool_slice_empty() {
    let polyform = Polyomino::from_bool_slice::<0>(&[]);
    assert_eq!(RegularCoord::new(0, 0), polyform.dim());
    assert!(polyform.cells().is_empty());

    let polyform = Polyomino::from_bool_slice(&[[]]);
    assert_eq!(RegularCoord::new(0, 0), polyform.dim());
    assert!(polyform.cells().is_empty());
}

#[test]
fn test_from_bool_slice_false() {
    let polyform = Polyomino::from_bool_slice(&[[false]]);
    assert_eq!(RegularCoord::new(1, 1), polyform.dim());
    assert!(polyform.cells().is_empty());
}

#[test]
fn test_from_bool_slice_true() {
    let polyform = Polyomino::from_bool_slice(&[[true]]);
    assert_eq!(RegularCoord::new(1, 1), polyform.dim());
    assert_eq!(1, polyform.cells().len());
    assert!(polyform.cells().contains(&Square::new(
        RegularCoord::new(0, 0),
        SquareOrientation::OnSide,
        ()
    )));
}

#[test]
fn test_from_bool_slice_multiple() {
    let polyform = Polyomino::from_bool_slice(&[[true, false, true], [true, true, true]]);
    assert_eq!(RegularCoord::new(2, 3), polyform.dim());
    assert_eq!(5, polyform.cells().len());
    assert!(polyform.cells().contains(&Square::new(
        RegularCoord::new(0, 0),
        SquareOrientation::OnSide,
        ()
    )));
    assert!(polyform.cells().contains(&Square::new(
        RegularCoord::new(0, 2),
        SquareOrientation::OnSide,
        ()
    )));
    assert!(polyform.cells().contains(&Square::new(
        RegularCoord::new(1, 0),
        SquareOrientation::OnSide,
        ()
    )));
    assert!(polyform.cells().contains(&Square::new(
        RegularCoord::new(1, 1),
        SquareOrientation::OnSide,
        ()
    )));
    assert!(polyform.cells().contains(&Square::new(
        RegularCoord::new(1, 2),
        SquareOrientation::OnSide,
        ()
    )));
}

#[test]
fn test_sized() {
    let polyform = Polyomino::sized(RegularCoord::new(1, 2), 1);
    assert_eq!(RegularCoord::new(1, 2), polyform.dim());
    assert_eq!(2, polyform.cells().len());
    assert!(polyform.cells().contains(&Square::new(
        RegularCoord::new(0, 0),
        SquareOrientation::OnSide,
        1
    )));
    assert!(polyform.cells().contains(&Square::new(
        RegularCoord::new(0, 1),
        SquareOrientation::OnSide,
        1
    )));
}

#[test]
fn test_rotate_around_size_1x1() {
    let mut shape = Polyomino::from_bool_slice(&[[true]]);
    let movement_request = shape.rotate_around(RegularCoord::new(0, 0));
    let expected = Polyomino::from_bool_slice(&[[true]]);
    assert_eq!(expected, shape);
    assert_eq!(RegularCoord::new(0, 0), movement_request);
}

#[test]
fn test_rotate_around_size_1x2_a_left() {
    let mut shape = Polyomino::from_bool_slice(&[[true, false]]);
    let movement_request = shape.rotate_around(RegularCoord::new(0, 0));
    let expected = Polyomino::from_bool_slice(&[[false], [true]]);
    assert_eq!(expected, shape);
    assert_eq!(RegularCoord::new(-1, 0), movement_request);
}

#[test]
fn test_rotate_around_size_1x2_a_right() {
    let mut shape = Polyomino::from_bool_slice(&[[false, true]]);
    let movement_request = shape.rotate_around(RegularCoord::new(0, 1));
    let expected = Polyomino::from_bool_slice(&[[true], [false]]);
    assert_eq!(expected, shape);
    assert_eq!(RegularCoord::new(0, 1), movement_request);
}

#[test]
fn test_rotate_around_size_1x2_b_left() {
    let mut shape = Polyomino::from_bool_slice(&[[true], [false]]);
    let movement_request = shape.rotate_around(RegularCoord::new(0, 0));
    let expected = Polyomino::from_bool_slice(&[[true, false]]);
    assert_eq!(expected, shape);
    assert_eq!(RegularCoord::new(0, 0), movement_request);
}

#[test]
fn test_rotate_around_size_1x2_b_right() {
    let mut shape = Polyomino::from_bool_slice(&[[false], [true]]);
    let movement_request = shape.rotate_around(RegularCoord::new(1, 0));
    let expected = Polyomino::from_bool_slice(&[[false, true]]);
    assert_eq!(expected, shape);
    assert_eq!(RegularCoord::new(1, -1), movement_request);
}

#[test]
fn test_rotate_around() {
    let mut shape = Polyomino::from_bool_slice(&[
        [true, false, false],
        [true, true, true],
        [true, false, true],
        [false, false, true],
    ]);
    let movement_request = shape.rotate_around(RegularCoord::new(0, 0));
    let expected = Polyomino::from_bool_slice(&[
        [false, true, true, true],
        [false, true, false, false],
        [true, true, true, false],
    ]);
    assert_eq!(expected, shape);
    assert_eq!(RegularCoord::new(-2, 0), movement_request);
}

#[test]
fn test_rotate_around_2() {
    let mut shape = Polyomino::from_bool_slice(&[
        [true, false, false],
        [true, true, true],
        [true, false, true],
        [false, false, true],
    ]);
    let movement_request = shape.rotate_around(RegularCoord::new(2, 1));
    let expected = Polyomino::from_bool_slice(&[
        [false, true, true, true],
        [false, true, false, false],
        [true, true, true, false],
    ]);
    assert_eq!(expected, shape);
    assert_eq!(RegularCoord::new(1, -1), movement_request);
}

#[test]
fn test_rotate_around_3() {
    let mut shape = Polyomino::from_bool_slice(&[
        [true, false, false],
        [true, true, true],
        [true, false, true],
        [false, false, true],
    ]);
    let movement_request = shape.rotate_around(RegularCoord::new(3, 2));
    let expected = Polyomino::from_bool_slice(&[
        [false, true, true, true],
        [false, true, false, false],
        [true, true, true, false],
    ]);
    assert_eq!(expected, shape);
    assert_eq!(RegularCoord::new(3, -1), movement_request);
}

#[test]
fn test_flip_size_1x1() {
    let mut shape = Polyomino::from_bool_slice(&[[true]]);
    let movement_request = shape.flip(RegularCoord::new(0, 0));
    let expected = Polyomino::from_bool_slice(&[[true]]);
    assert_eq!(expected, shape);
    assert_eq!(RegularCoord::new(0, 0), movement_request);
}

#[test]
fn test_flip_size_2x2_left() {
    let mut shape = Polyomino::from_bool_slice(&[[true, false], [false, true]]);
    let movement_request = shape.flip(RegularCoord::new(0, 0));
    let expected = Polyomino::from_bool_slice(&[[false, true], [true, false]]);
    assert_eq!(expected, shape);
    assert_eq!(RegularCoord::new(-1, 0), movement_request);
}

#[test]
fn test_flip_size_2x2_right() {
    let mut shape = Polyomino::from_bool_slice(&[[true, false], [false, true]]);
    let movement_request = shape.flip(RegularCoord::new(1, 0));
    let expected = Polyomino::from_bool_slice(&[[false, true], [true, false]]);
    assert_eq!(expected, shape);
    assert_eq!(RegularCoord::new(1, 0), movement_request);
}

#[test]
fn test_flip_left() {
    let mut shape = Polyomino::from_bool_slice(&[
        [true, false, false],
        [true, true, true],
        [true, false, true],
        [false, false, true],
    ]);
    let movement_request = shape.flip(RegularCoord::new(0, 0));
    let expected = Polyomino::from_bool_slice(&[
        [false, false, true],
        [true, false, true],
        [true, true, true],
        [true, false, false],
    ]);
    assert_eq!(expected, shape);
    assert_eq!(RegularCoord::new(-3, 0), movement_request);
}

#[test]
fn test_flip_middle() {
    let mut shape = Polyomino::from_bool_slice(&[
        [true, false, false],
        [true, false, true],
        [false, false, true],
    ]);
    let movement_request = shape.flip(RegularCoord::new(1, 2));
    let expected = Polyomino::from_bool_slice(&[
        [false, false, true],
        [true, false, true],
        [true, false, false],
    ]);
    assert_eq!(expected, shape);
    assert_eq!(RegularCoord::new(0, 0), movement_request);
}

#[test]
fn test_flip_right() {
    let mut shape = Polyomino::from_bool_slice(&[
        [true, false, false],
        [true, true, true],
        [true, false, true],
        [false, false, true],
    ]);
    let movement_request = shape.flip(RegularCoord::new(3, 0));
    let expected = Polyomino::from_bool_slice(&[
        [false, false, true],
        [true, false, true],
        [true, true, true],
        [true, false, false],
    ]);
    assert_eq!(expected, shape);
    assert_eq!(RegularCoord::new(3, 0), movement_request);
}

#[test]
fn test_rotate_to_landscape_empty() {
    let mut shape = Polyomino::from_bool_slice(&[[]]);
    shape.rotate_to_landscape();
    let expected = Polyomino::from_bool_slice(&[[]]);
    assert_eq!(expected, shape);
}

#[test]
fn test_rotate_to_landscape_one() {
    let mut shape = Polyomino::from_bool_slice(&[[true]]);
    shape.rotate_to_landscape();
    let expected = Polyomino::from_bool_slice(&[[true]]);
    assert_eq!(expected, shape);
}

#[test]
fn test_rotate_to_landscape() {
    let mut shape = Polyomino::from_bool_slice(&[[true, false, true, true]]);
    shape.rotate_to_landscape();
    let expected = Polyomino::from_bool_slice(&[[true], [false], [true], [true]]);
    assert_eq!(expected, shape);
}

#[test]
fn test_trim_sides_empty() {
    let mut array = Polyomino::from_bool_slice(&[[]]);
    let trim_sides = array.trim();
    let expected = Polyomino::from_bool_slice(&[[]]);
    assert_eq!(expected, array);
    let expected_trim_sides = TrimSides {
        negative: RegularCoord::new(0, 0),
        positive: RegularCoord::new(0, 0),
    };
    assert_eq!(expected_trim_sides, trim_sides);
}

#[test]
fn test_transpose() {
    let mut polyform = Polyomino::from_bool_slice(&[[true, false, true], [false, true, false]]);
    polyform.transpose();
    let expected = Polyomino::from_bool_slice(&[[true, false], [false, true], [true, false]]);
    assert_eq!(expected, polyform);
}

#[test]
fn test_mapped() {
    let polyform = Polyomino::from_vec(
        &[vec![true, false, true], vec![false, true, false]],
        &|v, _| Some(*v),
    );
    let mapped = polyform.mapped(&|v| !*v);
    let expected = Polyomino::from_vec(
        &[vec![false, true, false], vec![true, false, true]],
        &|v, _| Some(*v),
    );
    assert_eq!(expected, mapped);
}

#[test]
fn test_mapped_indexed() {
    let polyform = Polyomino::from_vec(
        &[vec![true, false, true], vec![false, true, false]],
        &|v, _| Some(*v),
    );
    let mapped = polyform.mapped_indexed(&|v, _| !*v);
    let expected = Polyomino::from_vec(
        &[vec![false, true, false], vec![true, false, true]],
        &|v, _| Some(*v),
    );
    assert_eq!(expected, mapped);
}

#[test]
fn test_filter_mapped() {
    let polyform = Polyomino::from_vec(&[vec![1, 2, 3], vec![4, 5, 6]], &|v, c| Some(*v));
    let mapped = polyform.filter_mapped(&|v| if *v % 2 == 0 { Some(*v) } else { None });
    let expected = Polyomino::from_vec(&[vec![0, 2, 0], vec![4, 0, 6]], &|v, c| {
        if *v != 0 { Some(*v) } else { None }
    });
    assert_eq!(expected, mapped);
}

#[test]
fn test_extend_adjacent() {
    let mut polyform =
        Polyomino::from_bool_slice(&[[true, false, true], [false, true, false]]).mapped(&|_| 1);
    polyform.extend_adjacent(2);
    let expected = Polyomino::from_vec(
        &[
            vec![0, 2, 0, 2, 0],
            vec![2, 1, 2, 1, 2],
            vec![0, 2, 1, 2, 0],
            vec![0, 0, 2, 0, 0],
        ],
        &|v, _| if *v != 0 { Some(*v) } else { None },
    );
    assert_eq!(expected, polyform);
}

#[test]
fn test_trim_sides_dont() {
    let mut array = Polyomino::from_bool_slice(&[[true]]);
    let trim_sides = array.trim();
    let expected = Polyomino::from_bool_slice(&[[true]]);
    assert_eq!(expected, array);
    let expected_trim_sides = TrimSides {
        negative: RegularCoord::new(0, 0),
        positive: RegularCoord::new(0, 0),
    };
    assert_eq!(expected_trim_sides, trim_sides);
}

#[test]
fn test_trim_sides_1() {
    let mut array = Polyomino::from_bool_slice(&[[false]]);
    let trim_sides = array.trim();
    let expected = Polyomino::from_bool_slice(&[[]]);
    assert_eq!(expected, array);
    let expected_trim_sides = TrimSides {
        negative: RegularCoord::new(0, 1),
        positive: RegularCoord::new(0, 0),
    };
    assert_eq!(expected_trim_sides, trim_sides);
}

#[test]
fn test_trim_sides_lower_y_upper_y() {
    let mut array = Polyomino::from_bool_slice(&[
        [false, false, true, false],
        [false, true, true, false],
        [false, false, true, false],
    ]);
    let trim_sides = array.trim();
    let expected = Polyomino::from_bool_slice(&[[false, true], [true, true], [false, true]]);
    assert_eq!(expected, array);
    let expected_trim_sides = TrimSides {
        negative: RegularCoord::new(0, 1),
        positive: RegularCoord::new(0, 1),
    };
    assert_eq!(expected_trim_sides, trim_sides);
}

#[test]
fn test_trim_sides_lower_x_upper_x() {
    let mut array = Polyomino::from_bool_slice(&[
        [false, false, false, false],
        [true, false, true, true],
        [false, false, false, false],
    ]);
    let trim_sides = array.trim();
    let expected = Polyomino::from_bool_slice(&[[true, false, true, true]]);
    assert_eq!(expected, array);
    let expected_trim_sides = TrimSides {
        negative: RegularCoord::new(1, 0),
        positive: RegularCoord::new(1, 0),
    };
    assert_eq!(expected_trim_sides, trim_sides);
}

#[test]
fn test_trim_sides_all_sides() {
    let mut array = Polyomino::from_bool_slice(&[
        [false, false, false, false, false],
        [false, false, true, true, false],
        [false, true, false, true, false],
        [false, false, false, false, false],
    ]);
    let trim_sides = array.trim();
    let expected = Polyomino::from_bool_slice(&[[false, true, true], [true, false, true]]);
    assert_eq!(expected, array);
    let expected_trim_sides = TrimSides {
        negative: RegularCoord::new(1, 1),
        positive: RegularCoord::new(1, 1),
    };
    assert_eq!(expected_trim_sides, trim_sides);
}

#[test]
fn test_trim_sides_from_lower_y() {
    let mut array =
        Polyomino::from_bool_slice(&[[false, false, true, true], [false, true, false, true]]);
    let trim_sides = array.trim();
    let expected = Polyomino::from_bool_slice(&[[false, true, true], [true, false, true]]);
    assert_eq!(expected, array);
    let expected_trim_sides = TrimSides {
        negative: RegularCoord::new(0, 1),
        positive: RegularCoord::new(0, 0),
    };
    assert_eq!(expected_trim_sides, trim_sides);
}

#[test]
fn test_trim_sides_from_upper_y() {
    let mut array =
        Polyomino::from_bool_slice(&[[true, true, false, false], [true, false, true, false]]);
    let trim_sides = array.trim();
    let expected = Polyomino::from_bool_slice(&[[true, true, false], [true, false, true]]);
    assert_eq!(expected, array);
    let expected_trim_sides = TrimSides {
        negative: RegularCoord::new(0, 0),
        positive: RegularCoord::new(0, 1),
    };
    assert_eq!(expected_trim_sides, trim_sides);
}

#[test]
fn test_trim_sides_from_lower_x() {
    let mut array = Polyomino::from_bool_slice(&[
        [false, false, false],
        [true, false, true],
        [false, true, false],
    ]);
    let trim_sides = array.trim();
    let expected = Polyomino::from_bool_slice(&[[true, false, true], [false, true, false]]);
    assert_eq!(expected, array);
    let expected_trim_sides = TrimSides {
        negative: RegularCoord::new(1, 0),
        positive: RegularCoord::new(0, 0),
    };
    assert_eq!(expected_trim_sides, trim_sides);
}

#[test]
fn test_trim_sides_from_upper_x() {
    let mut array = Polyomino::from_bool_slice(&[
        [true, false, true],
        [false, true, false],
        [false, false, false],
    ]);
    let trim_sides = array.trim();
    let expected = Polyomino::from_bool_slice(&[[true, false, true], [false, true, false]]);
    assert_eq!(expected, array);
    let expected_trim_sides = TrimSides {
        negative: RegularCoord::new(0, 0),
        positive: RegularCoord::new(1, 0),
    };
    assert_eq!(expected_trim_sides, trim_sides);
}

#[test]
fn test_trim_sides_rows_cols_test() {
    let mut array = Polyomino::from_bool_slice(&[
        [true, true, true, true],
        [true, true, true, true],
        [false, false, false, false],
        [true, false, true, false],
        [false, false, false, false],
    ]);
    let trim_sides = array.trim();
    let expected = Polyomino::from_bool_slice(&[
        [true, true, true, true],
        [true, true, true, true],
        [false, false, false, false],
        [true, false, true, false],
    ]);
    assert_eq!(expected, array);
    let expected_trim_sides = TrimSides {
        negative: RegularCoord::new(0, 0),
        positive: RegularCoord::new(1, 0),
    };
    assert_eq!(expected_trim_sides, trim_sides);
}

#[test]
fn test_count_biggest_connected_area_of_cells_matching_empty() {
    let shape = Polyomino::from_bool_slice(&[[]]);

    let count_true = shape.count_biggest_connected_area_of_cells_matching(true);
    let count_false = shape.count_biggest_connected_area_of_cells_matching(false);

    assert_eq!(count_true, 0);
    assert_eq!(count_false, 0);
}

#[test]
fn test_count_biggest_connected_area_of_cells_matching_true() {
    let shape = Polyomino::from_bool_slice(&[[true]]);

    let count_true = shape.count_biggest_connected_area_of_cells_matching(true);
    let count_false = shape.count_biggest_connected_area_of_cells_matching(false);

    assert_eq!(count_true, 1);
    assert_eq!(count_false, 0);
}

#[test]
fn test_count_biggest_connected_area_of_cells_matching_false() {
    let shape = Polyomino::from_bool_slice(&[[false]]);

    let count_true = shape.count_biggest_connected_area_of_cells_matching(true);
    let count_false = shape.count_biggest_connected_area_of_cells_matching(false);

    assert_eq!(count_true, 0);
    assert_eq!(count_false, 1);
}

#[test]
fn test_count_biggest_connected_area_of_cells_matching_ring() {
    let shape =
        Polyomino::from_bool_slice(&[[true, true, true], [true, false, true], [true, true, true]]);

    let count_true = shape.count_biggest_connected_area_of_cells_matching(true);
    let count_false = shape.count_biggest_connected_area_of_cells_matching(false);

    assert_eq!(count_true, 8);
    assert_eq!(count_false, 1);
}

#[test]
fn test_count_biggest_connected_area_of_cells_matching_complex() {
    let shape = Polyomino::from_bool_slice(&[
        [true, true, true, false, true],
        [true, false, true, false, true],
        [true, true, true, false, true],
    ]);

    let count_true = shape.count_biggest_connected_area_of_cells_matching(true);
    let count_false = shape.count_biggest_connected_area_of_cells_matching(false);

    assert_eq!(count_true, 8);
    assert_eq!(count_false, 3);
}

#[test]
fn test_place_on_all_positions() {
    let parent = Polyomino::from_bool_slice(&[
        [false, false, false],
        [false, false, false],
        [false, false, false],
    ]);
    let child = Polyomino::from_bool_slice(&[[true, false], [false, true]]);
    let placements = parent.place_on_all_positions(&child);
    assert_eq!(placements.len(), 4);
    assert!(placements.contains(&Polyomino::from_bool_slice(&[
        [true, false, false],
        [false, true, false],
        [false, false, false],
    ])));
    assert!(placements.contains(&Polyomino::from_bool_slice(&[
        [false, true, false],
        [false, false, true],
        [false, false, false],
    ])));
    assert!(placements.contains(&Polyomino::from_bool_slice(&[
        [false, false, false],
        [true, false, false],
        [false, true, false],
    ])));
    assert!(placements.contains(&Polyomino::from_bool_slice(&[
        [false, false, false],
        [false, true, false],
        [false, false, true],
    ])));
}

#[test]
fn test_place_on_all_positions_same_size() {
    let parent = Polyomino::from_bool_slice(&[[false, false], [false, false]]);
    let child = Polyomino::from_bool_slice(&[[true, false], [false, true]]);
    let placements = parent.place_on_all_positions(&child);
    assert_eq!(placements.len(), 1);
    assert!(placements.contains(&Polyomino::from_bool_slice(
        &[[true, false], [false, true],]
    )));
}

#[test]
fn test_place_on_all_positions_smaller_parent() {
    let parent = Polyomino::from_bool_slice(&[[false, false], [false, false]]);
    let child = Polyomino::from_bool_slice(&[[true, false, true], [false, true, false]]);
    let placements = parent.place_on_all_positions(&child);
    assert_eq!(placements.len(), 0);
}

#[test]
fn test_place_on_all_positions_with_blocking() {
    let parent = Polyomino::from_bool_slice(&[
        [false, false, false],
        [false, true, false],
        [false, false, false],
    ]);
    let child = Polyomino::from_bool_slice(&[[true, false], [false, true]]);
    let placements = parent.place_on_all_positions(&child);
    assert_eq!(placements.len(), 2);
    assert!(placements.contains(&Polyomino::from_bool_slice(&[
        [false, true, false],
        [false, true, true],
        [false, false, false],
    ])));
    assert!(placements.contains(&Polyomino::from_bool_slice(&[
        [false, false, false],
        [true, true, false],
        [false, true, false],
    ])));
}

#[test]
fn test_remove_parent() {
    let parent = Polyomino::from_bool_slice(&[
        [true, false, true],
        [false, true, false],
        [true, true, true],
    ]);
    let mut child =
        Polyomino::from_bool_slice(&[[true, true, true], [true, true, true], [true, true, true]]);
    child.remove_parent(&parent);
    let expected = Polyomino::from_bool_slice(&[
        [false, true, false],
        [true, false, true],
        [false, false, false],
    ]);
    assert_eq!(expected, child);
}

#[test]
fn test_remove_parent_smaller_parent() {
    let parent = Polyomino::from_bool_slice(&[[true, false], [false, true]]);
    let mut child =
        Polyomino::from_bool_slice(&[[true, true, true], [true, true, true], [true, true, true]]);
    child.remove_parent(&parent);
    let expected =
        Polyomino::from_bool_slice(&[[false, true, true], [true, false, true], [true, true, true]]);
    assert_eq!(expected, child);
}

#[test]
fn test_remove_parent_bigger_parent_panic() {
    let parent = Polyomino::from_bool_slice(&[
        [true, false, true],
        [false, true, false],
        [true, true, true],
    ]);
    let mut child = Polyomino::from_bool_slice(&[[true, true], [true, true]]);
    child.remove_parent(&parent);
}

#[test]
fn test_inverted() {
    let polyform = Polyomino::from_bool_slice(&[[true, false], [false, true]]);
    let inverted = polyform.inverted();
    let expected = Polyomino::from_bool_slice(&[[false, true], [true, false]]);
    assert_eq!(expected, inverted);
}

#[test]
fn test_count_biggest_connected_area_of_cells_matching() {
    let array = Polyomino::from_bool_slice(&[
        [true, false, true],
        [false, true, false],
        [true, true, true],
    ]);
    let count_true = array.count_biggest_connected_area_of_cells_matching(true);
    let count_false = array.count_biggest_connected_area_of_cells_matching(false);
    assert_eq!(count_true, 4);
    assert_eq!(count_false, 1);
}

#[test]
fn test_count_smallest_connected_area_of_cells_matching() {
    let array = Polyomino::from_bool_slice(&[
        [true, false, false],
        [true, false, true],
        [false, true, true],
    ]);
    let count_true = array.count_smallest_connected_area_of_cells_matching(true);
    let count_false = array.count_smallest_connected_area_of_cells_matching(false);
    assert_eq!(count_true, 2);
    assert_eq!(count_false, 1);
}
