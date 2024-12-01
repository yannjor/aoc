use std::fs::read_to_string;

fn part_one(nums: &[(u32, u32)]) -> u32 {
    let mut first_nums: Vec<u32> = nums.iter().map(|n| n.0).collect();
    let mut second_nums: Vec<u32> = nums.iter().map(|n| n.1).collect();
    first_nums.sort_unstable();
    second_nums.sort_unstable();
    (0..nums.len())
        .map(|i| first_nums[i].abs_diff(second_nums[i]))
        .sum()
}

fn part_two(nums: &[(u32, u32)]) -> u32 {
    let first_nums: Vec<u32> = nums.iter().map(|n| n.0).collect();
    let second_nums: Vec<u32> = nums.iter().map(|n| n.1).collect();
    first_nums
        .iter()
        .map(|&x| x * second_nums.iter().filter(|&&y| x == y).count() as u32)
        .sum()
}

fn main() {
    let input_string = read_to_string("./inputs/day01.txt").expect("Input file not found");
    let nums: Vec<(u32, u32)> = input_string
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let first = parts.next().unwrap().parse::<u32>().unwrap();
            let second = parts.next().unwrap().parse::<u32>().unwrap();
            (first, second)
        })
        .collect();

    println!("Part 1 solution: {}", part_one(&nums));
    println!("Part 2 solution: {}", part_two(&nums));
}
