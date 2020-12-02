use std::collections::HashSet;
use std::fs;

fn main() {
    let input_data: HashSet<u32> = get_input_data("input.txt");

    let part1_result = solve_part1(&input_data);
    println!("{}", part1_result);
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
        let second_entry = 2020 - input;
        if input_data.contains(&second_entry) {
            return input * second_entry;
        }
    }

    0
}


// tests


#[test]
fn solve_part1_works() {
    let input_data: HashSet<u32> =[10, 2010, 12].iter().cloned().collect();

    let result = solve_part1(&input_data);

    assert_eq!(result, 20100);
}