use advent_of_code::helpers::{parse_to, PointU32};

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let (field, size) = parse_to::grid_2d(input, None);
    let size = PointU32::new(size.0 as u32, size.1 as u32);
    let mut rolls_removed = 0;

    for y in 0..size.y() {
        for x in 0..size.x() {
            if field[y as usize][x as usize] != "@" { continue; }

            let rolls = surrounding_rolls(&field, PointU32::new(x, y), &size);
            if rolls.len() < 4 {
                rolls_removed += 1;
            }
        }
    }

    Some(rolls_removed as u32)
}

fn surrounding_rolls(field: &Vec<Vec<String>>, pos: PointU32, size: &PointU32) -> Vec<PointU32> {
    let points = pos.get_neighbours(size, true);
    points.into_iter()
        .filter(|&coord| field[coord.y() as usize][coord.x() as usize] == "@")
        .collect::<Vec<PointU32>>()
}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut field, size) = parse_to::grid_2d(input, None);
    let size = PointU32::new(size.0 as u32, size.1 as u32);
    let mut rolls_removed = 0;

    loop {
        let removed = remove_rolls(&mut field, &size);
        if removed == 0 { break; }
        rolls_removed += removed;
    }

    Some(rolls_removed)
}

fn remove_rolls(field: &mut Vec<Vec<String>>, size: &PointU32) -> u32 {
    let mut rolls_removed = 0;

    for y in 0..size.y() {
        for x in 0..size.x() {
            if field[y as usize][x as usize] != "@" { continue; }

            let rolls = surrounding_rolls(&field, PointU32::new(x, y), &size);
            if rolls.len() < 4 {
                rolls_removed += 1;
                field[y as usize][x as usize] = "x".to_string();
            }
        }
    }
    rolls_removed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
