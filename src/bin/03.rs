advent_of_code::solution!(3);

fn str_to_matrix(input: &str) -> Vec<Vec<char>> {
    let matrix = input.lines().fold(Vec::new(), |mut matrix, line| {
        matrix.push(line.chars().collect::<Vec<char>>());
        matrix
    });
    matrix
}

fn process_part_one(input: &str) -> usize {
    let matrix = str_to_matrix(input);
    let mut result: usize = 0;
    let mut num_str = String::new();
    let mut is_part_num = false;
    for i in 0..matrix.len() {
        for j in 0..matrix.len() {
            if matrix[i][j].is_ascii_digit() {
                num_str.push(matrix[i][j]);
                is_part_num = is_part_num || symbol_exist(&matrix, i as i32, j as i32);
            } else {
                if is_part_num {
                    // println!("{} {} => {}", i, j, num_str);
                    result += num_str.parse::<usize>().unwrap();
                }
                is_part_num = false;
                num_str = String::new();
            }
        }
        if is_part_num {
            result += num_str.parse::<usize>().unwrap();
        }

        is_part_num = false;
        num_str = String::new();
    }

    result
}

fn is_symbol(matrix: &Vec<Vec<char>>, i: i32, j: i32) -> bool {
    if i >= 0 && j >= 0 && i < matrix.len() as i32 && j < matrix[0].len() as i32 {
        let c = matrix[i as usize][j as usize];
        return !c.is_ascii_digit() && c != '.';
    }

    false
}

fn symbol_exist(matrix: &Vec<Vec<char>>, i: i32, j: i32) -> bool {
    is_symbol(matrix, i - 1, j - 1)
        || is_symbol(matrix, i - 1, j)
        || is_symbol(matrix, i - 1, j + 1)
        || is_symbol(matrix, i, j - 1)
        || is_symbol(matrix, i, j + 1)
        || is_symbol(matrix, i + 1, j - 1)
        || is_symbol(matrix, i + 1, j)
        || is_symbol(matrix, i + 1, j + 1)
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(process_part_one(input))
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
