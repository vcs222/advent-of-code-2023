advent_of_code::solution!(1);

fn extract_calibration_value(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            // print!("{}: ", line);
            let mut calibration: u32 = 10;
            for c in line.chars() {
                if c.is_digit(10) {
                    // print!("{} + ", c);
                    calibration *= c.to_digit(10).unwrap();
                    break;
                }
            }
            for c in line.chars().rev() {
                if c.is_digit(10) {
                    // print!("{}", c);
                    calibration += c.to_digit(10).unwrap();
                    break;
                }
            }
            // println!(" = {}", calibration);
            Some(calibration)
        })
        .flatten()
        .sum()
}

fn replace_numbers_in_line(line: &str) -> String {
    let replacements = [
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "t3e"),
        ("four", "f4r"),
        ("five", "f5e"),
        ("six", "s6x"),
        ("seven", "s7n"),
        ("eight", "e8t"),
        ("nine", "n9e"),
    ];

    let mut transformed_line = line.to_string();

    for (word, replacement) in replacements.iter() {
        transformed_line = transformed_line.replace(word, replacement);
    }

    transformed_line
}

fn extract_calibration_value_two(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            // print!("{}: ", line);
            let line = replace_numbers_in_line(line);
            Some(extract_calibration_value(line.as_str()))
        })
        .flatten()
        .sum()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(extract_calibration_value(input))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(extract_calibration_value_two(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
