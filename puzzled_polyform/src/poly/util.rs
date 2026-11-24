use crate::{Coord, Poly, Prototile};
use std::collections::HashSet;
use std::hash::Hash;
use std::ops::Add;

pub fn place_on_if_possible<C, P>(
    parent: &P,
    child: &P,
    positions: impl Iterator<Item = C>,
) -> Vec<Vec<C>>
where
    C: Coord + Add<Output = C>,
    P: Poly<(), C>,
{
    let mut placements = Vec::new();

    for position in positions {
        let mut valid = true;
        let mut placed_cells = Vec::new();
        for child_coord in child.dim().contained_by_span() {
            let child_cell = child.get(child_coord);
            let parent_coord = position + child_coord;
            let parent_cell = parent.get(parent_coord);
            match (parent_cell, child_cell) {
                (Some(_), Some(_)) => {
                    valid = false;
                    break;
                }
                (Some(_), None) => {
                    placed_cells.push(parent_coord);
                }
                (None, Some(_)) => {
                    placed_cells.push(parent_coord);
                }
                (None, None) => {}
            }
        }
        if valid {
            placements.push(placed_cells);
        }
    }

    placements
}

pub fn inverted_cells<C, F>(dim: C, contains: F, size_hint: usize) -> Vec<C>
where
    C: Coord,
    F: Fn(C) -> bool,
{
    let mut new_cells = Vec::with_capacity(size_hint);
    for coord in dim.contained_by_span() {
        if !contains(coord) {
            new_cells.push(coord);
        }
    }
    new_cells
}

pub fn count_biggest_connected_area_of_cells<'a, P, T, C, O, F>(
    cells: impl Iterator<Item = &'a P>,
    contains: F,
) -> usize
where
    P: Prototile<T, C, O> + 'a,
    C: Coord,
    T: Clone + Hash,
    F: Fn(C) -> bool,
{
    let mut visited_cells = HashSet::new();
    let mut max_area = 0;

    for prototile in cells {
        let coord = prototile.coord();
        if !visited_cells.contains(&coord) {
            let area = count_connected_cells(coord, &contains, &mut visited_cells);
            max_area = max_area.max(area);
        }
    }

    max_area
}

pub fn count_smallest_connected_area_of_cells<'a, P, T, C, O, F>(
    cells: impl Iterator<Item = &'a P>,
    contains: F,
) -> usize
where
    P: Prototile<T, C, O> + 'a,
    C: Coord,
    T: Clone + Hash,
    F: Fn(C) -> bool,
{
    let mut visited_cells = HashSet::new();
    let mut min_area = usize::MAX;

    for prototile in cells {
        let coord = prototile.coord();
        if !visited_cells.contains(&coord) {
            let area = count_connected_cells(coord, &contains, &mut visited_cells);
            min_area = min_area.min(area);
        }
    }

    min_area
}

fn count_connected_cells<C, F>(coord: C, contains: F, visited_cells: &mut HashSet<C>) -> usize
where
    C: Coord,
    F: Fn(C) -> bool,
{
    let mut area = 0;
    let mut stack = vec![coord];

    while let Some(c) = stack.pop() {
        if !visited_cells.contains(&c) && contains(c) {
            visited_cells.insert(c);
            area += 1;

            // Add neighbors to the stack
            for neighbour in c.neighbours() {
                if !visited_cells.contains(&neighbour) {
                    stack.push(neighbour);
                }
            }
        }
    }

    area
}
