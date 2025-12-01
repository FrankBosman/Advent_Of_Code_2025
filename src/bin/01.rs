advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut times_zero = 0;
    let mut counter = 50;
    let max = 100;

    let lines = input.trim().lines();
    for line in lines {
        let first_char = line.get(0..1)?;
        let remainder = line.get(1..)?;
        let steps = remainder.parse::<i32>().ok()?;

        counter = match first_char {
            "L" => (counter - steps) % max,
            "R" => (counter + steps) % max,
            _ => {
                panic!("Invalid first char: {}", first_char);
            }
        };

        if counter == 0 {
            times_zero += 1;
        }
    }
    Some(times_zero)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut times_zero: i32 = 0;
    let mut current: i32 = 50;
    let max = 100;

    let lines = input.trim().lines();
    for line in lines {
        let first_char = line.get(0..1)?;
        let remainder = line.get(1..)?;
        let steps = remainder.parse::<i32>().ok()?;

        match first_char {
            "R" => {
                let current_boundaries = floor_div(current, max);
                current -= steps;
                let new_boundaries = floor_div(current, max);

                // Calculate how many "100 boundaries" we crossed going up
                times_zero += new_boundaries - current_boundaries;
            },
            "L" => {
                let current_boundaries = floor_div(current - 1, max);
                current -= steps;
                let new_boundaries = floor_div(current - 1, max);

                // Calculate how many "100 boundaries" we crossed going down, first 0 doesn't count
                times_zero += current_boundaries - new_boundaries;
            },
            _ => panic!("Invalid char"),
        };
    }

    Some(times_zero as u32)
}

fn floor_div(val: i32, div: i32) -> i32 {
    let result = val / div;
    let remainder = val % div;
    if remainder < 0 { result - 1 } else { result }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
