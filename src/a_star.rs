use bevy::prelude::*;
use crate::grid::{Grid, TileState};

const MAX_SEARCH_DEPTH: usize = 1000;
const WALKABLE_TILE_STATES: [TileState; 3] = [TileState::Empty, TileState::InteractionPoint, TileState::Robot];



/// very basic A* function
pub fn a_star(start: IVec2, end: IVec2, grid: &Grid, _: usize) -> Option<Vec<IVec2>> {
    let mut open_list = vec![(start.distance_squared(end), start)];
    let mut closed_list = Vec::new();

    for _ in 0..MAX_SEARCH_DEPTH {
        // println!("\n{:?}", open_list);
        // println!("{:?}", closed_list);
        if open_list.is_empty() {return None;}

        let (_, current_pos) = open_list.remove(0); // grab smallest f on open list
        closed_list.push(current_pos);

        // found goal
        if current_pos == end {
            return Some(trim_a_star(closed_list));
        }

        // add children
        let children = get_available_children(current_pos, grid);
        for child_location in children {
            if closed_list.contains(&child_location) {continue;}
            let child_point = (heuristic(start, end, child_location), child_location);
            if !open_list.contains(&child_point) {open_list.push(child_point)}
            // println!("{:?}", open_list[open_list.len() - 1]);
        }

        open_list = quicksort(open_list);

        
    }
    // println!("\n{:?}", open_list);
    // println!("{:?}", closed_list);
    return None;
}

/// sorts the given values list by their accosiated f32 in ascending order
fn quicksort<T, P: PartialOrd + Copy>(list: Vec<(P, T)>) -> Vec<(P, T)>{
    if list.len() <= 1 {return list;}

    let mut less = Vec::new();
    let mut equal = Vec::new();
    let mut more = Vec::new();

    let target_val = list[list.len() / 2].0;

    for item in list {
        if item.0 < target_val {less.push(item)}
        else if item.0 > target_val {more.push(item)}
        else {equal.push(item)}
    }

    let mut sorted = quicksort(less);
    sorted.append(&mut equal);
    sorted.append(&mut quicksort(more));
    sorted
}

fn get_available_children(current: IVec2, grid: &Grid) -> Vec<IVec2> {
    let mut result = Vec::new();

    if WALKABLE_TILE_STATES.contains(&grid[current + IVec2::X]) {result.push(current + IVec2::X)};
    if WALKABLE_TILE_STATES.contains(&grid[current - IVec2::X]) {result.push(current - IVec2::X)};
    if WALKABLE_TILE_STATES.contains(&grid[current + IVec2::Y]) {result.push(current + IVec2::Y)};
    if WALKABLE_TILE_STATES.contains(&grid[current - IVec2::Y]) {result.push(current - IVec2::Y)};

    result
}


fn heuristic(_: IVec2, end: IVec2, current: IVec2) -> i32 {
    current.distance_squared(end)
}


fn trim_a_star(mut path: Vec<IVec2>) -> Vec<IVec2>{
    if path.len() <= 2 {
        return path;
    }

    let mut i = 0;
    let mut trimmed_path = vec![path.pop().unwrap()];
    path.reverse();

    loop {
        if path.len() <= i {break;}

        if trimmed_path.last().unwrap().distance_squared(path[i]) == 1 {
            trimmed_path.push(path.remove(i));
        } else {
            i += 1;
        }
    }


    trimmed_path
}