use std::collections::HashSet;
use std::fs;

const result_to_comapre: u32 = 2020;

fn main() {
    let input_data: HashSet<u32> = get_input_data("input.txt");

    let part1_result = solve_part1(&input_data);
    println!("part 1 {}", part1_result);

    let part2_result = solve_part2(&input_data);
    println!("part 2 {}", part2_result);
}

fn get_input_data(input_file_mame: &str) -> HashSet<u32> {
    let file_content =
        fs::read_to_string(input_file_mame).expect("something went wrong reading file");
    let values: HashSet<u32> = file_content
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    values
}

fn solve_part1(input_data: &HashSet<u32>) -> u32 {
    for input in input_data {
        let second_entry = result_to_comapre - input;
        if input_data.contains(&second_entry) {
            return input * second_entry;
        }
    }

    0
}

fn solve_part2(input_data: &HashSet<u32>) -> u32 {
    for input in input_data {
        if input > &result_to_comapre {
            continue;
        }

        for second_entry in input_data {
            let sum = input + second_entry;
            if sum > result_to_comapre {
                continue;
            }

            let third_entry = result_to_comapre - second_entry - input;
            if input_data.contains(&third_entry) {
                return input * second_entry * third_entry;
            }
        }
    }

    0
}

// tests

#[test]
fn solve_part1_works() {
    let input_data: HashSet<u32> = [10, 2010, 12].iter().cloned().collect();

    let result = solve_part1(&input_data);

    assert_eq!(result, 20100);
}

#[test]
fn solve_part2_works() {
    let input_data: HashSet<u32> = [10, 20, 1990, 12].iter().cloned().collect();

    let result = solve_part2(&input_data);

    assert_eq!(result, 398000);
}
