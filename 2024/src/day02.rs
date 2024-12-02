use std::fs::read_to_string;

fn is_safe(report: &[u32]) -> bool {
    let sorted = report.windows(2).all(|w| w[0] < w[1]) || report.windows(2).all(|w| w[0] > w[1]);
    sorted && report.windows(2).all(|w| w[0].abs_diff(w[1]) <= 3)
}

fn part_one(reports: &[Vec<u32>]) -> u32 {
    reports.iter().filter(|&report| is_safe(report)).count() as u32
}

fn part_two(reports: &[Vec<u32>]) -> u32 {
    reports
        .iter()
        .filter(|&report| {
            is_safe(report)
                || (0..report.len()).any(|i| {
                    let mut report_trimmed = report.clone();
                    report_trimmed.remove(i);
                    is_safe(&report_trimmed)
                })
        })
        .count() as u32
}

fn main() {
    let input_string = read_to_string("./inputs/day02.txt").expect("Input file not found");
    let reports: Vec<Vec<u32>> = input_string
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();

    println!("Part 1 solution: {}", part_one(&reports));
    println!("Part 2 solution: {}", part_two(&reports));
}
