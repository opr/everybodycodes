use std::cmp::{min};
use crate::input_reader::input_reader::read_input_for_day;

pub fn main() {
    println!("Day 1 part 1: {}", part1(read_input_for_day(2025, 1, false)));
    println!("Day 1 part 2: {}", part2(read_input_for_day(2025, "1.2", false)));
    println!("Day 1 part 3: {}", part3(read_input_for_day(2025, "1.3", false)));
}

pub fn part1( input: String ) -> String {
    let input_parts = input.split("\n").collect::<Vec<&str>>();
    let values = input_parts[0].split(',').collect::<Vec<&str>>();
    let instructions = input_parts[2].split(',').collect::<Vec<&str>>();

    let final_index = instructions.iter().fold( 0, | count: usize, current_instruction | {
        let split_instruction = current_instruction.chars().collect::<Vec<char>>();
        let direction = split_instruction[0];
        let value: usize = str::parse(split_instruction[1..].iter().collect::<String>().as_str()).unwrap();
        match direction {
            'L' => count.saturating_sub(value),
            'R' => min(values.len()-1,count+value),
            _ => count
        }
    });
    values[final_index].to_string()
}
pub fn part2( input: String ) -> String {
    let input_parts = input.split("\n").collect::<Vec<&str>>();
    let values = input_parts[0].split(',').collect::<Vec<&str>>();
    let instructions = input_parts[2].split(',').collect::<Vec<&str>>();
    let values_length = values.len() as isize;

    let final_index = instructions.iter().fold( 0, | count: isize, current_instruction | {
        let split_instruction = current_instruction.chars().collect::<Vec<char>>();
        let direction = split_instruction[0];
        let value: isize = str::parse(split_instruction[1..].iter().collect::<String>().as_str()).unwrap();
        match direction {
            'L' => (count + (values_length) - value) % (values_length),
            'R' => (count + value) % (values_length),
            _ => count
        }
    });
    values[final_index as usize].to_string()
}

pub fn part3( input: String ) -> String {
    let input_parts = input.split("\n").collect::<Vec<&str>>();
    let mut values = input_parts[0].split(',').collect::<Vec<&str>>();
    let instructions = input_parts[2].split(',').collect::<Vec<&str>>();
    let values_length = values.len();

    for instruction in instructions {
        let split_instruction = instruction.chars().collect::<Vec<char>>();
        let direction = split_instruction[0];
        let value: usize = str::parse(split_instruction[1..].iter().collect::<String>().as_str()).unwrap();
        let index_to_move = match direction {
            'L' => (0 - value as isize).rem_euclid(values_length as isize) as usize,
            'R' => (0 + value).rem_euclid(values_length),
            _ => 0
        };
        let temp = values[0];
        values[0] = values[index_to_move];
        values[index_to_move] = temp;
    }

    values[0].to_string()
}

#[cfg(test)]
mod tests {
    use crate::input_reader::input_reader::read_input_for_day;
    use crate::y2025::day1::{part1, part2, part3};

    #[test]
    fn test_part1_test() {
        let input = read_input_for_day(2025, 1, true);
        let result = part1(input);
        assert_eq!(result, "Fyrryn")
    }
    #[test]
    fn test_part2_test() {
        let input = read_input_for_day(2025, 1, true);
        let result = part2(input);
        assert_eq!(result, "Elarzris")
    }
    #[test]
    fn test_part3_test() {
        let input = read_input_for_day(2025, "1.3", true);
        let result = part3(input);
        assert_eq!(result, "Drakzyph")
    }

    #[test]
    fn test_part1_real() {
        let input = read_input_for_day(2025, 1, false);
        let result = part1(input);
        assert_eq!(result, "Vyrzar")
    }
    #[test]
    fn test_part2_real() {
        let input = read_input_for_day(2025, "1.2", false);
        let result = part2(input);
        assert_eq!(result, "Lornfelix")
    }
    #[test]
    fn test_part3_real() {
        let input = read_input_for_day(2025, "1.3", false);
        let result = part3(input);
        assert_eq!(result, "Rylarparth")
    }
}