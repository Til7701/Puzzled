use crate::polyform::coord::regular::RegularCoord;
use crate::polyform::poly::polyomino::Polyomino;
use crate::polyform::poly::{Poly, TrimSides, UnitPoly};

#[test]
fn test_rotate_counterclockwise_size_1() {
    let mut shape = Polyomino::from_bool_slice(&[[true]]);
    shape.rotate_around(RegularCoord::new(0, 0));
    let expected = Polyomino::from_bool_slice(&[[true]]);
    assert_eq!(expected, shape);
}

#[test]
fn test_rotate_counterclockwise_size_2() {
    let mut shape = Polyomino::from_bool_slice(&[[true, false]]);
    shape.rotate_around(RegularCoord::new(0, 0));
    let expected = Polyomino::from_bool_slice(&[[false], [true]]);
    assert_eq!(expected, shape);
}

#[test]
fn test_rotate_counterclockwise() {
    let mut shape = Polyomino::from_bool_slice(&[
        [true, false, false],
        [true, true, true],
        [true, false, true],
    ]);
    shape.rotate_around(RegularCoord::new(0, 0));
    let expected = Polyomino::from_bool_slice(&[
        [false, true, true],
        [false, true, false],
        [true, true, true],
    ]);
    assert_eq!(expected, shape);
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
fn test_trim_sides_empty() {
    let mut array = Polyomino::from_bool_slice(&[[true]]);
    let trim_sides = array.trim();
    let expected = Polyomino::from_bool_slice(&[[true]]);
    assert_eq!(expected, array);
    let expected_trim_sides = TrimSides {
        lower: RegularCoord::new(0, 0),
        upper: RegularCoord::new(0, 0),
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
        lower: RegularCoord::new(0, 1),
        upper: RegularCoord::new(0, 0),
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
        lower: RegularCoord::new(0, 1),
        upper: RegularCoord::new(0, 1),
    };
    assert_eq!(expected_trim_sides, trim_sides);
}

#[test]
fn test_trim_sides_lower_x_upper_x() {
    let mut array = Polyomino::from_bool_slice(&[
        [true, true, true, true],
        [false, true, false, false],
        [true, true, true, true],
    ]);
    let trim_sides = array.trim();
    let expected = Polyomino::from_bool_slice(&[[false, true, false, false]]);
    assert_eq!(expected, array);
    let expected_trim_sides = TrimSides {
        lower: RegularCoord::new(1, 0),
        upper: RegularCoord::new(1, 0),
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
        lower: RegularCoord::new(1, 1),
        upper: RegularCoord::new(1, 1),
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
        lower: RegularCoord::new(0, 1),
        upper: RegularCoord::new(0, 0),
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
        lower: RegularCoord::new(0, 0),
        upper: RegularCoord::new(0, 1),
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
        lower: RegularCoord::new(1, 0),
        upper: RegularCoord::new(0, 0),
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
        lower: RegularCoord::new(0, 0),
        upper: RegularCoord::new(1, 0),
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
        lower: RegularCoord::new(0, 0),
        upper: RegularCoord::new(1, 0),
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

// #[test]
// fn test_or_arrays_at() {
//     let parent = Polyomino::from_bool_slice(&[
//         [false, false, false],
//         [false, false, false],
//         [false, false, false],
//     ]);
//     let child = Polyomino::from_bool_slice(&[[true, false], [false, true]]);
//     let result = parent.or_at(&child, 1, 1);
//     let expected = Polyomino::from_bool_slice(&[
//         [false, false, false],
//         [false, true, false],
//         [false, false, true],
//     ]);
//     assert_eq!(expected, result);
// }
//
// #[test]
// fn test_or_arrays_at_empty_child() {
//     let parent = Polyomino::from_bool_slice(&[
//         [false, false, false],
//         [false, false, false],
//         [false, false, false],
//     ]);
//     let child = Polyomino::from_bool_slice(&[[]]);
//     let result = parent.or_at(&child, 1, 1);
//     let expected = Polyomino::from_bool_slice(&[
//         [false, false, false],
//         [false, false, false],
//         [false, false, false],
//     ]);
//     assert_eq!(expected, result);
// }
//
// #[test]
// fn test_or_arrays_at_child_1x1() {
//     let parent = Polyomino::from_bool_slice(&[
//         [false, false, false],
//         [false, false, false],
//         [false, false, false],
//     ]);
//     let child = Polyomino::from_bool_slice(&[[true]]);
//     let result = parent.or_at(&child, 1, 1);
//     let expected = Polyomino::from_bool_slice(&[
//         [false, false, false],
//         [false, true, false],
//         [false, false, false],
//     ]);
//     assert_eq!(expected, result);
// }
//
// #[test]
// fn test_or_arrays_at_child_off_parent() {
//     let parent = Polyomino::from_bool_slice(&[
//         [false, false, false],
//         [false, false, false],
//         [false, false, false],
//     ]);
//     let child = Polyomino::from_bool_slice(&[[true, true], [true, true]]);
//     let result = parent.or_at(&child, 2, 2);
//     let expected = Polyomino::from_bool_slice(&[
//         [false, false, false],
//         [false, false, false],
//         [false, false, true],
//     ]);
//     assert_eq!(expected, result);
// }
//
// #[test]
// fn test_or_arrays_at_true_parent() {
//     let parent = Polyomino::from_bool_slice(&[[true, true, true], [true, true, true], [true, true, true]]);
//     let child = Polyomino::from_bool_slice(&[[true, false], [true, false]]);
//     let result = parent.or_at(&child, 1, 1);
//     let expected = Polyomino::from_bool_slice(&[[true, true, true], [true, true, true], [true, true, true]]);
//     assert_eq!(expected, result);
// }
//
// #[test]
// fn test_or_arrays_at_smaller_parent() {
//     let parent = Polyomino::from_bool_slice(&[[false, false], [false, false]]);
//     let child = Polyomino::from_bool_slice(&[[true, true, true], [true, true, true], [true, true, true]]);
//     let result = parent.or_at(&child, 0, 0);
//     let expected = Polyomino::from_bool_slice(&[[true, true], [true, true]]);
//     assert_eq!(expected, result);
// }

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
#[should_panic]
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
