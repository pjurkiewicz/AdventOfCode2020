use std::collections::HashSet;
use std::fs;
use std::ops::BitXor;


fn main() {
    let input_data = get_input_data("input.txt");
    let part1_result = solve_part1(&input_data);
    println!("part1: {}", part1_result);

    let part2_result = solve_part2(&input_data);
    println!("part1: {}", part2_result);
}

fn get_input_data(input_file_mame: &str) -> HashSet<String> {
    let file_content =
        fs::read_to_string(input_file_mame).expect("something went wrong reading file");
    let values: HashSet<String> = file_content.lines().map(String::from).collect();

    return values.clone();
}

fn solve_part1(input_data: &HashSet<String>) -> u32 {
    let mut result = 0;
    for line in input_data {
        let splitted_line: Vec<&str> = line.split_whitespace().collect();
        let (min, max) = parse_letter_count(splitted_line[0]);
        let required_character = parse_required_character(splitted_line[1]);
        let required_character_count = splitted_line[2]
            .chars()
            .filter(|l| l == &required_character)
            .count() as u32;
        if required_character_count >= min && required_character_count <= max {
            result += 1;
        }
    }

    result
}

fn solve_part2(input_data: &HashSet<String>) -> u32 {
    let mut result = 0;
    for line in input_data {
        let splitted_line: Vec<&str> = line.split_whitespace().collect();
        let (fist_pos, second_pos) = parse_letter_count(splitted_line[0]);
        let required_character = parse_required_character(splitted_line[1]);
        let word: Vec<char> = splitted_line[2].chars().collect();
        let fixed_first_pos = (fist_pos - 1) as usize;
        let fixed_second_pos = (second_pos - 1) as usize;

        if word.len() <= fixed_second_pos || word.len() <= fixed_first_pos {
            continue;
        }

        if (word[fixed_first_pos] == required_character) ^ (word[fixed_second_pos] == required_character) {
            result += 1;
        }
    }

    result
}

fn parse_required_character(value: &str) -> char {
    value.chars().next().unwrap()
}

fn parse_letter_count(value: &str) -> (u32, u32) {
    let splitter_min_max: Vec<u32> = value.split("-").map(|f| f.parse().unwrap()).collect();

    (splitter_min_max[0], splitter_min_max[1])
}

#[test]
fn solve_part1_works() {
    let input_data: HashSet<String> = ["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"]
        .iter()
        .cloned()
        .map(String::from)
        .collect();

    let result = solve_part1(&input_data);

    assert_eq!(result, 2);
}

#[test]
fn solve_part2_works() {
    let input_data: HashSet<String> = ["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"]
        .iter()
        .cloned()
        .map(String::from)
        .collect();

    let result = solve_part2(&input_data);

    assert_eq!(result, 1);
}
