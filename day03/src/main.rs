use std::fs;

fn main() {
    let input_data: Vec<String> = get_input_data("input.txt");
    let part1_result = solve_part1(&input_data);
    println!("part1: {}", part1_result);

    let part2_result = solve_part2(&input_data);
    println!("part2: {}", part2_result);
}

fn get_input_data(input_file_mame: &str) -> Vec<String> {
    let file_content =
        fs::read_to_string(input_file_mame).expect("something went wrong reading file");
    let values: Vec<String> = file_content.lines().map(String::from).collect();

    return values;
}

fn solve_part1(input_data: &Vec<String>) -> u32 {
    tree_finder(input_data, 3, 1)
}

fn solve_part2(input_data: &Vec<String>) -> usize {
    let trees: Vec<u32> = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|a| tree_finder(input_data, a.0, a.1))
        .collect();

    let mut result: usize = 1;
    for elem in trees {
        result *= elem as usize;
    }

    result
}

fn tree_finder(
    input_data: &Vec<String>,
    horizontal_steps_count: usize,
    vertical_steps_count: usize,
) -> u32 {
    let mut row_pos: usize = 0;

    let mut line_number: usize = 0;
    let mut tree_count: u32 = 0;
    for line in input_data {
        if line_number % vertical_steps_count > 0 {
            line_number +=1;
            continue;
        }

        let line_chars: Vec<char> = line.chars().collect();
        if line_chars.len() <= row_pos {
            row_pos = row_pos - line_chars.len();
        }
        if line_chars[row_pos] == '#' {
            tree_count += 1;
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

#[test]
fn solve_part2_works() {
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

    let result = solve_part2(&input_data);

    assert_eq!(result, 336);
}
