use std::fs::read_to_string;

fn part_one(measurements: &[u32]) -> u32 {
    let mut count = 0;
    for i in 1..measurements.len() {
        if measurements[i] > measurements[i - 1] {
            count += 1;
        }
    }
    count
}

fn part_two(measurements: &[u32]) -> u32 {
    let mut sums: Vec<u32> = Vec::new();
    for i in 2..measurements.len() {
        sums.push(measurements[i - 2] + measurements[i - 1] + measurements[i]);
    }
    part_one(&sums)
}

fn main() {
    let input_string = read_to_string("./inputs/day1.txt").expect("Input file not found");
    let measurements = input_string
        .lines()
        .map(|i| i.parse::<u32>().expect("Failed to parse measurement value"))
        .collect::<Vec<_>>();
    println!("Part 1 solution: {}", part_one(&measurements));
    println!("Part 2 solution: {}", part_two(&measurements));
}
