use std::collections::HashSet;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let mut sum = 0u64;

    for range in input.split(",") {
        let (begin, end) = range.split_once( "-")
            .expect("Invalid range, it does not contain an '-'.");
        let begin= begin.trim().parse::<u64>().expect("Couldn't parse range begin");
        let end = end.trim().parse::<u64>().expect("Couldn't parse range end");

        sum += check_range_part_1_optimized(begin, end);
    }

    Some(sum)
}

fn _check_range_part_1(start_range: u64, end_range: u64) -> u64 {
    let mut sum = 0u64;
    for num in start_range..=end_range {
        let length = num.checked_ilog10().unwrap_or(0) + 1;
        if (length % 2) != 0 {
            continue;
        }

        let start = num / 10u64.pow(length / 2);
        let end = num % 10u64.pow(length / 2);
        if start == end {
            sum += num;
        }
    }

    sum
}

fn check_range_part_1_optimized(start_range: u64, end_range: u64) -> u64 {
    let mut sum = 0u64;

    // Repeating numbers are in the form: x * (10^k + 1), where x is the first part
    // Example: 1212: 12 * 100 + 12 = 1200 + 12 = 1212
    // The maximum length of a number in u64 is ~1e19. half of that is 9
    for half_len in 1..=9 {
        // Get the range of digits with this length, ie 10 - 99
        let half_start = 10u64.pow(half_len - 1);
        let half_end = 10u64.pow(half_len) - 1;
        let multiplier = 10u64.pow(half_len) + 1;  // (10^k + 1)

        // Calculate bounds for 'x', range_start <= x * multiplier <= range_end
        let min_x_target = start_range.div_ceil(multiplier);
        let max_x_target = end_range / multiplier;

        // Intersect the valid digit range (10..99) with the target range
        let actual_min = u64::max(half_start, min_x_target);
        let actual_max = u64::min(half_end, max_x_target);

        if actual_min <= actual_max {
            // score of this range:     (actual_min * multiplier) + ... + (actual_max * multiplier)
            // This factors to:         (actual_min + ... + actual_max) * multiplier
            // Using arithmetic series: (count * (first + last) / 2)    * multiplier

            let count = actual_max - actual_min + 1;
            let sub_sum = (actual_min + actual_max) * count / 2;
            sum += sub_sum * multiplier;
        }
    }
    sum
}

pub fn _part_two_original(input: &str) -> Option<u64> {
    let mut sum = 0u64;
    let ranges = input.split(",").collect::<Vec<&str>>();

    for range in ranges {
        let (begin, end) = range.split_once( "-")
            .expect("Invalid range, it does not contain an '-'.");
        let begin= begin.trim().parse::<u64>().expect("Couldn't parse range begin");
        let end = end.trim().parse::<u64>().expect("Couldn't parse range end");

        for num in begin..=end {
            if !is_valid_part_2(num) {
                sum += num;
            }
        }
    }

    Some(sum)
}

fn is_valid_part_2(num: u64) -> bool {
    let string = num.to_string();

    for i in 1..=(string.len() / 2) {
        if string.len() % i != 0 { continue; }

        let (start, remainder) = string.split_at(i);
        if remainder.find(start).is_some() {
            let count = remainder.matches(start).count();
            if count == remainder.len() / start.len() {
                return false;
            }
        }
    }

    true
}


// Optimized version of part 2
pub fn part_two(input: &str) -> Option<u64> {
    let mut total_sum = 0u64;

    for range in input.split(',') {
        let (begin_str, end_str) = range.split_once('-')
            .expect("Invalid range format, no '-' found.");

        let range_start: u64 = begin_str.trim().parse().expect("Invalid start");
        let range_end: u64 = end_str.trim().parse().expect("Invalid end");

        // Set to deal with duplicate numbers (1 4x = 1111 is also 11 2x = 1111)
        let mut valid_numbers = HashSet::new();

        // Get the range of digits with this length, ie 10 - 99
        let min_len = range_start.checked_ilog10().unwrap_or(0) + 1;
        let max_len = range_end.checked_ilog10().unwrap_or(0) + 1;

        // Iterate through all possible total lengths (e.g., 6 digits: 121212)
        for digit_length in min_len..=max_len {


            // If digit length is 6, chunks can be len 1 (repeat 6x), 2 (repeat 3x), or 3 (repeat 2x)
            for pattern_length in 1..=(digit_length / 2) {
                if digit_length % pattern_length != 0 {
                    continue;
                }

                // Multiplier, take pattern of length 2, total length 6 this means three times
                // Pattern: XX XX XX, multiplier then is: 10^4 + 10^2 + 10^0 = 10101
                let reps = digit_length / pattern_length;
                let mut multiplier = 0u64;
                for i in 0..reps {
                    multiplier += 10u64.pow(i * pattern_length);
                }

                // Calculate bounds for the pattern number
                let min_root_digits = 10u64.pow(pattern_length - 1);
                let max_root_digits = 10u64.pow(pattern_length) - 1;

                // Determine which roots result in a number inside the range
                let min_root_target = range_start.div_ceil(multiplier);
                let max_root_target = range_end / multiplier;

                // Intersect the valid digit bounds with the target bounds
                let start = u64::max(min_root_digits,min_root_target);
                let end = u64::min(max_root_digits, max_root_target);

                // Loop through all te patterns in the range and add them to the set
                if start <= end {
                    for root in start..=end {
                        valid_numbers.insert(root * multiplier);
                    }
                }
            }
        }

        // Finally sum the set for this range
        total_sum += valid_numbers.iter().sum::<u64>();
    }

    Some(total_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
