use std::fs::read_to_string;

fn parse_instruction(instruction: &str) -> (&str, u32) {
    let instruction_tuple = instruction
        .split_once(" ")
        .expect("Failed to parse instruction");
    (
        instruction_tuple.0,
        instruction_tuple
            .1
            .parse::<u32>()
            .expect("Failed to parse instruction value"),
    )
}

fn part_one(instructions: &[(&str, u32)]) -> u32 {
    let mut depth = 0;
    let mut position = 0;
    for instruction in instructions {
        match instruction {
            ("forward", val) => position += val,
            ("down", val) => depth += val,
            ("up", val) => depth -= val,
            (ins, val) => eprintln!("Unknown instruction: {} {}", ins, val),
        }
    }
    depth * position
}

fn part_two(instructions: &[(&str, u32)]) -> u32 {
    let mut depth = 0;
    let mut position = 0;
    let mut aim = 0;
    for instruction in instructions {
        match instruction {
            ("forward", val) => {
                position += val;
                depth += aim * val;
            }
            ("down", val) => aim += val,
            ("up", val) => aim -= val,
            (ins, val) => eprintln!("Unknown instruction: {} {}", ins, val),
        }
    }
    depth * position
}

fn main() {
    let input_string = read_to_string("./inputs/day2.txt").expect("Input file not found");
    let instructions = input_string
        .lines()
        .map(|i| parse_instruction(i))
        .collect::<Vec<_>>();
    println!("Part 1 solution: {}", part_one(&instructions));
    println!("Part 2 solution: {}", part_two(&instructions));
}
