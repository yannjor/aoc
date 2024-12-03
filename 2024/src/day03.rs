use std::fs::read_to_string;

fn part_one(input: &str) -> u32 {
    input
        .match_indices("mul(")
        .filter_map(|(i, _)| {
            input[i..].chars().position(|c| c == ')').and_then(|j| {
                let pattern = &input[i + 4..i + j];
                let nums: Vec<&str> = pattern.split(',').collect();
                if nums.len() == 2 {
                    nums[0].parse::<u32>().ok().zip(nums[1].parse::<u32>().ok())
                } else {
                    None
                }
            })
        })
        .map(|(n1, n2)| n1 * n2)
        .sum()
}

fn part_two(input: &str) -> u32 {
    let mut enabled = true;
    input
        .match_indices("mul(")
        .filter_map(|(i, _)| {
            input[i..]
                .chars()
                .position(|c| c == ')')
                .and_then(|j| {
                    let pattern = &input[i + 4..i + j];
                    let nums: Vec<&str> = pattern.split(',').collect();
                    if nums.len() == 2 {
                        nums[0].parse::<u32>().ok().zip(nums[1].parse::<u32>().ok())
                    } else {
                        None
                    }
                })
                .and_then(|(n1, n2)| {
                    let do_index = input[..i].rfind("do()");
                    let dont_index = input[..i].rfind("don't()");

                    enabled = if let (Some(do_idx), Some(dont_idx)) = (do_index, dont_index) {
                        do_idx > dont_idx
                    } else if do_index.is_some() {
                        true
                    } else {
                        dont_index.is_none()
                    };

                    if enabled {
                        Some(n1 * n2)
                    } else {
                        None
                    }
                })
        })
        .sum()
}

fn main() {
    let input_string = read_to_string("./inputs/day03.txt").expect("Input file not found");

    println!("Part 1 solution: {}", part_one(&input_string));
    println!("Part 2 solution: {}", part_two(&input_string));
}
