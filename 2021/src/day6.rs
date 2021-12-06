use std::fs::read_to_string;

fn fish_after_n_days(counters: &[u8], days: u32) -> u64 {
    let mut fish_count_at_each_state = (0..9)
        .map(|state| counters.iter().filter(|x| **x == state).count() as u64)
        .collect::<Vec<_>>();
    for _ in 0..days {
        // Shift left
        fish_count_at_each_state = [
            &fish_count_at_each_state[1..],
            &fish_count_at_each_state[..1],
        ]
        .concat();
        // New parents start at 6, babies at 8
        fish_count_at_each_state[6] += fish_count_at_each_state[8];
    }
    fish_count_at_each_state.iter().sum()
}

fn part_one(counters: &[u8]) -> u64 {
    fish_after_n_days(counters, 80)
}

fn part_two(counters: &[u8]) -> u64 {
    fish_after_n_days(counters, 256)
}

fn main() {
    let input_string = read_to_string("./inputs/day6.txt").expect("Input file not found");
    let counters = input_string
        .split(',')
        .map(|x| x.trim().parse::<u8>().unwrap())
        .collect::<Vec<_>>();
    println!("Part 1 solution: {}", part_one(&counters));
    println!("Part 2 solution: {}", part_two(&counters));
}
