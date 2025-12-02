advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let mut sum = 0u64;
    let ranges = input.split(",").collect::<Vec<&str>>();

    for range in ranges {
        let (begin, end) = range.split_once( "-").expect(&format!("Invalid range, it does not contain an '-'. Range: {}", range));
        let (begin, end) = (begin.trim().parse::<u64>().expect("Couldn't parse range begin"), end.trim().parse::<u64>().expect("Couldn't parse range end"));

        for num in begin..=end {
            if !is_valid_part_1(num) {
                sum += num;
            }
        }
    }

    Some(sum)
}

fn is_valid_part_1(num: u64) -> bool {
    let string = num.to_string();
    if (string.len() % 2) != 0 { return true; }

    let (begin, end) = string.split_at(string.len() / 2);

    begin != end
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut sum = 0u64;
    let ranges = input.split(",").collect::<Vec<&str>>();

    for range in ranges {
        let (begin, end) = range.split_once( "-").expect(&format!("Invalid range, it does not contain an '-'. Range: {}", range));
        let (begin, end) = (begin.trim().parse::<u64>().expect("Couldn't parse range begin"), end.trim().parse::<u64>().expect("Couldn't parse range end"));

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
