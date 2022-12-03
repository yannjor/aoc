use std::collections::HashSet;
use std::fs::read_to_string;

fn get_item_value(item: char) -> u32 {
    if item.is_lowercase() {
        item as u32 - 96
    } else {
        item as u32 - 38
    }
}

fn part_one(rucksacks: &[&str]) -> u32 {
    rucksacks
        .iter()
        .map(|rucksack| {
            let compartment_1 = &rucksack[0..rucksack.len() / 2];
            let compartment_2 = &rucksack[rucksack.len() / 2..rucksack.len()];
            let unique_a = compartment_1.chars().collect::<HashSet<_>>();
            let unique_b = compartment_2.chars().collect::<HashSet<_>>();
            let shared_item = *unique_a
                .intersection(&unique_b)
                .next()
                .expect("Found no shared items between compartments");
            get_item_value(shared_item)
        })
        .sum()
}

fn part_two(rucksacks: &[&str]) -> u32 {
    let mut groups = Vec::new();
    for i in (0..rucksacks.len() - 2).step_by(3) {
        groups.push(&rucksacks[i..i + 3]);
    }
    groups
        .iter()
        .map(|group| {
            if let [bag_1, bag_2, bag_3] = group {
                let unique_a = bag_1.chars().collect::<HashSet<_>>();
                let unique_b = bag_2.chars().collect::<HashSet<_>>();
                let unique_c = bag_3.chars().collect::<HashSet<_>>();
                let shared_ab = unique_a
                    .intersection(&unique_b)
                    .copied()
                    .collect::<HashSet<_>>();
                let shared_item = *shared_ab
                    .intersection(&unique_c)
                    .next()
                    .expect("Found no shared items between group");
                get_item_value(shared_item)
            } else {
                panic!("Invalid input")
            }
        })
        .sum()
}

fn main() {
    let input_string = read_to_string("./inputs/day3.txt").expect("Input file not found");
    let rucksacks = input_string.lines().collect::<Vec<_>>();
    println!("Part 1 solution: {}", part_one(&rucksacks));
    println!("Part 2 solution: {}", part_two(&rucksacks));
}
