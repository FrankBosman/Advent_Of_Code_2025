advent_of_code::solution!(5);

struct FreshRange {
    begin: u64,
    end: u64
}

impl FreshRange {
    fn new(input: &str) -> FreshRange {
        let (begin, end) = input.split_once("-").unwrap();
        Self{
            begin: begin.parse::<u64>().unwrap(),
            end: end.parse::<u64>().unwrap()
        }
    }

    fn in_range(&self, num: u64) -> bool {
        self.begin <= num && num <= self.end
    }

    pub(crate) fn clone(&self) -> FreshRange {
        Self {
            begin: self.begin,
            end: self.end
        }
    }
    pub(crate) fn increase_end(&mut self, new_end: u64) {
        self.end = new_end;
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (fresh, available) = input.split_once("\n\n")?;

    let mut fresh_ranges = Vec::<FreshRange>::new();
    for row in fresh.lines() {
        fresh_ranges.push(FreshRange::new(row));
    }

    // From part 2
    let merged = merge_range(fresh_ranges);

    let mut fresh_count = 0;
    for row in available.lines() {
        if merged.iter().any(|range| range.in_range(row.parse::<u64>().unwrap())) {
            fresh_count += 1;
        }
    }

    Some(fresh_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (fresh, _) = input.split_once("\n\n")?;

    let mut fresh_ranges = Vec::<FreshRange>::new();
    for row in fresh.lines() {
        fresh_ranges.push(FreshRange::new(row));
    }

    let merged = merge_range(fresh_ranges);

    let mut total = 0;
    for range in merged {
        total += range.end - range.begin + 1;  // plus one as it is in inclusive, 1 <= x <= 5.
    }
    Some(total)
}
fn merge_range(mut ranges: Vec<FreshRange>) -> Vec<FreshRange> {
    ranges.sort_unstable_by_key(|range| range.begin);
    let mut new_ranges = Vec::<FreshRange>::new();
    new_ranges.push(ranges[0].clone());
    let mut last = &new_ranges[0];

    for range in &ranges[1..] {
        // If the ranges do not overlap
        if last.end < range.begin {
            new_ranges.push(range.clone());
            last = &new_ranges[new_ranges.len() - 1];
            continue;
        }

        // If they overlap completely, skip
        if last.end >= range.end {
            continue;
        }

        // If they partially overlap, expand the previous range
        let mut last_range = new_ranges.pop().unwrap();
        last_range.increase_end(range.end);
        new_ranges.push(last_range);
        last = &new_ranges[new_ranges.len() - 1];
    }

    new_ranges
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
        assert_eq!(result, Some(14));
    }
}
