use std::fs::read_to_string;

fn part_one(rounds: &[(&str, &str)]) -> u32 {
    rounds
        .iter()
        .map(|(a, b)| match (*a, *b) {
            ("A", "X") => 4,
            ("A", "Y") => 8,
            ("A", "Z") => 3,
            ("B", "X") => 1,
            ("B", "Y") => 5,
            ("B", "Z") => 9,
            ("C", "X") => 7,
            ("C", "Y") => 2,
            ("C", "Z") => 6,
            _ => panic!("Illegal move"),
        })
        .sum()
}

fn part_two(rounds: &[(&str, &str)]) -> u32 {
    rounds
        .iter()
        .map(|(a, b)| match (*a, *b) {
            ("A", "X") => 3,
            ("A", "Y") => 4,
            ("A", "Z") => 8,
            ("B", "X") => 1,
            ("B", "Y") => 5,
            ("B", "Z") => 9,
            ("C", "X") => 2,
            ("C", "Y") => 6,
            ("C", "Z") => 7,
            _ => panic!("Illegal move"),
        })
        .sum()
}

fn main() {
    let input_string = read_to_string("./inputs/day2.txt").expect("Input file not found");
    let rounds = input_string
        .lines()
        .map(|r| r.split_once(' ').expect("Failed to parse round strategy"))
        .collect::<Vec<_>>();
    println!("Part 1 solution: {}", part_one(&rounds));
    println!("Part 2 solution: {}", part_two(&rounds));
}
