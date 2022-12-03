use std::fs::read_to_string;

fn part_one(elf_calories: &[Vec<u32>]) -> u32 {
    elf_calories
        .iter()
        .map(|calories| calories.iter().sum())
        .max()
        .unwrap()
}

fn part_two(elf_calories: &[Vec<u32>]) -> u32 {
    let mut sums: Vec<u32> = elf_calories
        .iter()
        .map(|calories| calories.iter().sum())
        .collect();
    sums.sort_unstable();
    sums[sums.len() - 3..sums.len()].iter().sum()
}

fn main() {
    let input_string = read_to_string("./inputs/day1.txt").expect("Input file not found");
    let elf_calories = input_string
        .split("\n\n")
        .map(|calories| {
            calories
                .lines()
                .map(|c| c.parse::<u32>().expect("Failed to parse calorie value"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    println!("Part 1 solution: {}", part_one(&elf_calories));
    println!("Part 2 solution: {}", part_two(&elf_calories));
}
