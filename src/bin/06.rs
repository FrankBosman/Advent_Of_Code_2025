advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let mut problems: Vec<Vec<&str>> = Vec::new();

    for line in input.trim().lines() {
        for (i, val) in line.trim().split_whitespace().enumerate() {
            if i >= problems.len() {
                problems.push(Vec::new());
            }
            problems[i].push(val);
        }
    }

    let mut grand_total = 0;
    for problem in problems {
        let mut problem_total = problem[0].parse::<u64>().unwrap();
        let operator = problem[problem.len() -1];
        for &digit in problem[1..(problem.len() - 1)].iter() {
            match operator {
                "+" => { problem_total += digit.parse::<u64>().unwrap() }
                "*" => { problem_total *= digit.parse::<u64>().unwrap() }
                _ => { panic!("Unknown operator: {}", operator) }
            }
        }
        grand_total += problem_total;
    }
    Some(grand_total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut problems: Vec<Vec<String>> = Vec::new();

    let lines = input.lines().collect::<Vec<_>>();
    let mut operators = Vec::new();
    let mut lengths = Vec::new();

    let mut current_length = 0;
    operators.push(lines[lines.len() - 1].chars().next().unwrap());
    for char in lines[lines.len() - 1].chars() {
        if char != ' ' && current_length != 0 {
            lengths.push(current_length - 1);
            current_length = 1;
            operators.push(char);
        } else {
            current_length += 1;
        }
    }
    lengths.push(current_length);

    // Parses the input to the form:
    // [["123", "045", "006"], ["328", "640", "980"], ["051", "387", "215"], ["640", "230", "314"]]
    for &line in &lines[..(lines.len() - 1)] {
        let prefixed = " ".to_owned() + line;
        let mut remainder = prefixed.as_str();

        let mut i = 0;
        while remainder.len() > 0 {
            let part: &str;
            (part, remainder) = remainder[1..].split_at(lengths[i] as usize);

            if i >= problems.len() {
                problems.push(Vec::new());
            }
            problems[i].push(part.replace(" ", "0"));
            i += 1;
        }
    }

    let mut grand_total = 0;
    for ((problem, operator), &length) in problems.iter().zip(operators.iter()).zip(lengths.iter()) {
        let mut problem_total = if operator.eq(&'+') {0} else {1};

        for i in 0..length{
            let mut term = 0;
            for val in problem.iter() {
                let digit =  get_digit_at(val, val.len() - 1  - i);
                if digit == 0 { continue; }
                term = term * 10 + digit;
            }
            match operator {
                '+' => { problem_total += term }
                '*' => { problem_total *= term }
                _ => { panic!("Unknown operator: {}", operator) }
            }
        }
        grand_total += problem_total;
    }
    Some(grand_total)
}

pub fn get_digit_at(val: &str, i: usize) -> u64 {
    (val.as_bytes()[i] - b'0') as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
