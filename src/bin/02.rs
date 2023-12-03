advent_of_code::solution!(2);

fn validate_play_step(play_step: &str, max_red: u32, max_green: u32, max_blue: u32) -> bool {
    let parts = play_step.split(", ").collect::<Vec<&str>>();
    parts.iter().all(|part| {
        let part = part.split(' ').collect::<Vec<&str>>();
        let value = part[0].parse::<u32>().unwrap();
        match part[1] {
            "red" => value <= max_red,
            "green" => value <= max_green,
            "blue" => value <= max_blue,
            _ => false,
        }
    })
}
fn process_part_one(input: &str, max_red: u32, max_green: u32, max_blue: u32) -> u32 {
    input
        .lines()
        .flat_map(|line| {
            let parts = line.split(':').collect::<Vec<&str>>();
            let game = parts[0].split("Game ").collect::<Vec<&str>>()[1]
                .parse::<u32>()
                .unwrap();
            let play_steps = parts[1].trim().split("; ").collect::<Vec<&str>>();
            Some(
                if play_steps
                    .iter()
                    .all(|play_step| validate_play_step(play_step, max_red, max_green, max_blue))
                {
                    game
                } else {
                    0
                },
            )
        })
        .sum()
}

fn process_part_two(input: &str) -> u32 {
    input
        .lines()
        .flat_map(|line| {
            let play_steps: Vec<&str> = line.split(':').collect();
            if let Some(play_steps) = play_steps.get(1) {
                let max_values: (u32, u32, u32) = play_steps
                    .trim()
                    .split("; ")
                    .flat_map(|play_step| {
                        play_step.split(", ").flat_map(|part| {
                            let part: Vec<&str> = part.split(' ').collect();
                            let value = part[0].parse::<u32>().unwrap_or(0);
                            let color = part.get(1).unwrap_or(&"").to_lowercase();
                            match color.as_str() {
                                "red" => Some((value, 0, 0)),
                                "green" => Some((0, value, 0)),
                                "blue" => Some((0, 0, value)),
                                _ => None,
                            }
                        })
                    })
                    .fold((0, 0, 0), |acc, (r, g, b)| {
                        (acc.0.max(r), acc.1.max(g), acc.2.max(b))
                    });
                Some(max_values.0 * max_values.1 * max_values.2)
            } else {
                None
            }
        })
        .sum()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(process_part_one(input, 12, 13, 14))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(process_part_two(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
