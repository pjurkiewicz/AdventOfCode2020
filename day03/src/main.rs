use std::collections::HashSet;
use std::fs;

fn main() {
    let  input_data : Vec<String> = get_input_data("input.txt");
    let part1_result = solve_part1(&input_data);
    println!("part1: {}", part1_result);
}

fn get_input_data(input_file_mame: &str) -> Vec<String> {
    let mut file_content =
        fs::read_to_string(input_file_mame).expect("something went wrong reading file");
    let values: Vec<String> = file_content.lines().map(String::from).collect();

    return values;
}

fn solve_part1(input_data: &Vec<String>) -> u32 {
    let mut row_pos: usize = 0;
    const horizontal_steps_count: usize = 3;

    let mut line_number: usize = 0;
    let mut tree_count: u32 = 0;
    for line in input_data {
        let line_chars: Vec<char> = line.chars().collect();
        if line_chars.len() <= row_pos {
            row_pos = row_pos - line_chars.len();
        }
        if line_chars[row_pos] == '#' {
            tree_count += 1;
            println!("lien {} pos {}", line_number, row_pos);
        }

        row_pos += horizontal_steps_count;
        line_number += 1;
    }

    tree_count
}

// tests

#[test]
fn solve_part1_works() {
    let input_data: Vec<String> = [
        "..##.........##.........##.........##.........##.........##.......",
        "#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..",
        ".#....#..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.",
        "..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#",
        ".#...##..#..#...##..#..#...##..#..#...##..#..#...##..#..#...##..#.",
        "..#.##.......#.##.......#.##.......#.##.......#.##.......#.##.....",
        ".#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#",
        ".#........#.#........#.#........#.#........#.#........#.#........#",
        "#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...",
        "#...##....##...##....##...##....##...##....##...##....##...##....#",
        ".#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#",
    ]
    .iter()
    .cloned()
    .map(String::from)
    .collect();

    let result = solve_part1(&input_data);

    assert_eq!(result, 7);
}
