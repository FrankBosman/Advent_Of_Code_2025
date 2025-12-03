// use advent_of_code::helpers::string_helpers::get_digit_at;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let mut max_joltage = 0;

    for bank in input.trim().lines() {
        //// Original approach which relied on finding the high number in series
        // let length = bank.len();
        //
        // let (first_digit, first_index) = find_largest_digit(0, length - 1, bank);
        // let (second_digit, _) = find_largest_digit(first_index + 1, length, bank);
        //
        // max_joltage += first_digit * 10 + second_digit;

        max_joltage += find_joltage::<2>(bank) as u32;
    }

    Some(max_joltage)
}

pub fn part_two(input: &str) -> Option<u64> {
    let max_joltage: u64 = input
        .trim()
        .lines()
        .map(|bank| find_joltage::<12>(bank))
        .sum();

    Some(max_joltage)
}

pub fn get_digit_at(bank: &str, i: usize) -> u32 {
    (bank.as_bytes()[i] - b'0') as u32
}

/// New approach, it keeps a buffer of found numbers and if a new number is found
/// the entire buffer is checked as long as th new number is higher (up to the remaining digits).
/// The older values are then cleared.
/// This prevents us from looking over the entire string multiple times.
fn find_joltage<const BATTERY_LENGTH: usize>(bank: &str) -> u64 {
    let mut buffer= [0; BATTERY_LENGTH];  // The buffer stores the highest digits in order
    let mut latest_pointer = 0;
    buffer[0] = get_digit_at(bank, latest_pointer);

    for i in 1..bank.len() {
        let val = get_digit_at(bank, i);

        let mut ii = latest_pointer + 1;
        while ii > 0 && val > buffer[ii - 1] && (BATTERY_LENGTH - ii) < bank.len() - i {
            buffer[ii - 1] = 0;
            ii -= 1;
        }
        if ii < BATTERY_LENGTH {
            buffer[ii] = val;
            latest_pointer = ii;
        }
    }

    buffer.iter().fold(0, |acc, &digit| acc * 10 + digit as u64)
}

/// Original approach, which relied on finding the highest number
fn _find_largest_digit(from: usize, to: usize, bank: &str) -> (u32, usize) {
    let mut max_value = get_digit_at(bank, from);
    let mut max_index = from;
    if max_value == 9 { return (max_value, max_index); }

    for i in (from + 1)..to {
        let val = get_digit_at(bank, i);
        if val > max_value {
            max_value = val;
            max_index = i;
            if max_value == 9 { break; }
        }
    }
    (max_value, max_index)
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
