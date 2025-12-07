use std::collections::{HashSet, VecDeque};
use advent_of_code::helpers::{parse_to, Point, PointU32};

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u32> {
    let ( field, size ) = parse_to::grid_2d(input, None);
    let mut splitters_encountered = 0;

    let mut frontier = VecDeque::new();
    let mut visited = HashSet::new();

    // Find the starting point
    let start = find_start(&field, &size, "S").expect("Could not find starting point 'S'.");
    frontier.push_back(start);

    while frontier.len() > 0 {
        let current = frontier.pop_front().unwrap();
        if visited.contains(&current) { continue; }
        visited.insert(current);

        // move down one, and check whether it is in bounds
        let new = current + PointU32::new(0, 1);
        if new.y() >= size.1 as u32 { continue; }

        // Check for a splitter
        match field[new.y_usize()][new.x_usize()].as_str() {
            "." => frontier.push_back(new),
            "^" => {
                // Create two new points
                if let Some(new_left) = new.checked_add_bounded_xy(&Point::new(-1, 0), size.0, size.1) {
                    if !visited.contains(&new_left) {
                        frontier.push_back(new_left);
                    }
                }
                if let Some(new_right) = new.checked_add_bounded_xy(&Point::new(1, 0), size.0, size.1) {
                    if !visited.contains(&new_right) {
                        frontier.push_back(new_right);
                    }
                }
                splitters_encountered += 1;
            },
            _ => {panic!("Unknown char found, {}", field[new.y_usize()][new.x_usize()])}
        }
    }

    Some(splitters_encountered)
}

fn find_start(field: &Vec<Vec<String>>, size: &(usize, usize), start_char: &str) -> Option<PointU32> {
    for y in 0..size.1 {
        for x in 0..size.0 {
            if field[y][x] == start_char {
                return Some(PointU32::new(x as u32, y as u32));
            }
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
