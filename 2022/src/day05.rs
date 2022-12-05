use std::fs::read_to_string;

fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<Vec<usize>>) {
    let (stacks, instructions) = input.split_once("\n\n").expect("Failed to parse input");
    let n_stacks = ((stacks.lines().next().unwrap().len() - 1) as f32 / 4.0).ceil() as u32;
    let mut crate_stacks: Vec<Vec<char>> = (0..n_stacks).map(|_| vec![]).collect();
    for line in stacks.lines().rev() {
        for i in (1..line.len()).step_by(4) {
            let line_chars: Vec<char> = line.chars().collect();
            if line_chars[i].is_alphabetic() {
                crate_stacks[i / 4].push(line_chars[i]);
            }
        }
    }
    let instructions: Vec<Vec<usize>> = instructions
        .lines()
        .map(|l| {
            l.split_whitespace()
                .flat_map(|w| w.parse::<usize>())
                .collect()
        })
        .collect();

    (crate_stacks, instructions)
}

fn part_one(input: &str) -> String {
    let parsed = parse_input(input);
    let mut crate_stacks = parsed.0;
    let instructions = parsed.1;
    for instruction in instructions.iter() {
        let amount = instruction[0];
        let from = instruction[1] - 1;
        let to = instruction[2] - 1;
        (0..amount).for_each(|_| {
            let moved = crate_stacks[from].pop().expect("No element to move");
            crate_stacks[to].push(moved);
        });
    }
    crate_stacks.iter().map(|s| s[s.len() - 1]).collect()
}

fn part_two(input: &str) -> String {
    let parsed = parse_input(input);
    let mut crate_stacks = parsed.0;
    let instructions = parsed.1;
    for instruction in instructions.iter() {
        let amount = instruction[0];
        let from = instruction[1] - 1;
        let to = instruction[2] - 1;
        let from_crates = crate_stacks[from].len();
        let to_move = crate_stacks[from][from_crates - amount..from_crates].to_vec();
        for c in to_move.iter() {
            crate_stacks[to].push(*c);
            crate_stacks[from].pop();
        }
    }
    crate_stacks.iter().map(|s| s[s.len() - 1]).collect()
}

fn main() {
    let input_string = read_to_string("./inputs/day5.txt").expect("Input file not found");
    println!("Part 1 solution: {}", part_one(&input_string));
    println!("Part 2 solution: {}", part_two(&input_string));
}

#[cfg(test)]
mod tests {
    use super::*;
        const INPUT: &str = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;

    #[test]
    fn test_part_one() {
        let result = part_one(INPUT);
        assert_eq!(result, "CMZ");
    }
    #[test]
    fn test_part_two() {
        let result = part_two(INPUT);
        assert_eq!(result, "MCD");
    }
}
