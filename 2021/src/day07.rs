use std::fs::read_to_string;

fn part_one(postitions: &[i32]) -> i32 {
    let median = postitions[postitions.len() / 2];
    postitions.iter().map(|x| (x - median).abs()).sum()
}

fn part_two(postitions: &[i32]) -> i32 {
    let median = postitions[postitions.len() / 2];
    let mut prev_cost = i32::MAX;
    for i in median..postitions[postitions.len() - 1] {
        let cost = postitions
            .iter()
            .map(|x| {
                let diff = (x - i).abs();
                (1..diff + 1).sum::<i32>()
            })
            .sum::<i32>();
        if cost > prev_cost {
            return prev_cost;
        }
        prev_cost = cost;
    }
    -1
}

fn main() {
    let input_string = read_to_string("./inputs/day7.txt").expect("Input file not found");
    let mut postitions = input_string
        .split(',')
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    postitions.sort_unstable();
    println!("Part 1 solution: {}", part_one(&postitions));
    println!("Part 2 solution: {}", part_two(&postitions));
}
