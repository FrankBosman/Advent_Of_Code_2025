advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let mut max_joltage = 0;

    for bank in input.trim().lines() {
        let length = bank.len();

        let (first_digit, first_index) = find_largest_digit(0, length - 1, bank);
        let (second_digit, _) = find_largest_digit(first_index + 1, length, bank);

        max_joltage += first_digit * 10 + second_digit;
    }

    Some(max_joltage)
}

fn find_largest_digit(from: usize, to: usize, bank: &str) -> (u32, usize) {
    let mut max_value = get_digit_at(bank, from);
    let mut max_index = from;
    if max_value == 9 { return (max_value, max_index); }

    for i in (from + 1)..to {
        // Get the current value for instance i = 2: 12x456.
        let val = get_digit_at(bank, i);
        if val > max_value {
            max_value = val;
            max_index = i;
            if max_value == 9 { break; }
        }
    }
    (max_value, max_index)
}

fn get_digit_at(bank: &str, i: usize) -> u32 {
    bank.chars().nth(i).unwrap()
        .to_digit(10).unwrap()
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut max_joltage = 0u64;

    for bank in input.trim().lines() {
        let length = bank.len();
        let mut joltage = 0u64;
        let mut next_index = 0;

        for i in 0..12 {
            let (first_digit, first_index) = find_largest_digit(next_index, length - (11 - i), bank);
            next_index = first_index + 1;
            joltage += (first_digit as u64) * 10u64.pow((11 - i) as u32);
        }

        max_joltage += joltage;
    }

    Some(max_joltage)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
