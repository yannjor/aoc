use std::fs::read_to_string;

fn part_one(output_values: &[Vec<&str>]) -> u32 {
    output_values
        .concat()
        .iter()
        .filter(|x| {
            let len = x.chars().count();
            len == 2 || len == 3 || len == 4 || len == 7
        })
        .count() as u32
}

fn part_two(input_values: &[Vec<&str>], output_values: &[Vec<&str>]) -> u32 {
    let mut numbers_decoded = Vec::new();
    for (i, entry) in output_values.iter().enumerate() {
        let mut chars = String::new();
        let one_chars: Vec<char> = input_values[i]
            .iter()
            .find(|p| p.chars().count() == 2)
            .unwrap()
            .chars()
            .collect();
        let four_chars: Vec<char> = input_values[i]
            .iter()
            .find(|p| p.chars().count() == 4)
            .unwrap()
            .chars()
            .collect();
        for pattern in entry {
            let pattern_chars: Vec<char> = pattern.chars().collect();
            let digit = match (
                pattern_chars.len(),
                one_chars
                    .iter()
                    .filter(|c| pattern_chars.contains(c))
                    .count(),
                four_chars
                    .iter()
                    .filter(|c| pattern_chars.contains(c))
                    .count(),
            ) {
                (2, 2, 2) => '1',
                (3, 2, 2) => '7',
                (4, 2, 4) => '4',
                (5, 1, 2) => '2',
                (5, 1, 3) => '5',
                (5, 2, 3) => '3',
                (6, 1, 3) => '6',
                (6, 2, 3) => '0',
                (6, 2, 4) => '9',
                (7, 2, 4) => '8',
                _ => panic!("Failed to decode digit {}", pattern),
            };
            chars.push(digit);
        }
        numbers_decoded.push(chars.parse::<u32>().unwrap());
    }
    numbers_decoded.iter().sum::<u32>()
}

fn main() {
    let input_string = read_to_string("./inputs/day8.txt").expect("Input file not found");
    let input_values = input_string
        .lines()
        .map(|l| {
            l.split_once(" | ")
                .unwrap()
                .0
                .split_whitespace()
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let output_values = input_string
        .lines()
        .map(|l| {
            l.split_once(" | ")
                .unwrap()
                .1
                .split_whitespace()
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    println!("Part 1 solution: {}", part_one(&output_values));
    println!(
        "Part 2 solution: {}",
        part_two(&input_values, &output_values)
    );
}
