use std::collections::HashMap;
use std::fs::read_to_string;

fn solve(lines: &[&str]) -> (u32, u64) {
    let pairs = HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);
    let points_p1 = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    let points_p2 = HashMap::from([('(', 1), ('[', 2), ('{', 3), ('<', 4)]);

    let mut score_p1 = 0;
    let mut scores_p2 = Vec::new();

    for line in lines {
        let mut stack: Vec<char> = Vec::new();
        let mut corrupt = false;
        for c in line.chars() {
            if pairs.contains_key(&c) {
                stack.push(c);
            // Wrong closing bracket
            } else if *pairs.get(&stack[stack.len() - 1]).unwrap() != c {
                score_p1 += points_p1.get(&c).unwrap();
                corrupt = true;
                break;
            } else {
                stack.pop();
            }
        }
        if !corrupt {
            stack.reverse();
            let mut score: u64 = 0;
            for c in &stack {
                score *= 5;
                score += points_p2[c];
            }
            scores_p2.push(score);
        }
    }
    scores_p2.sort_unstable();
    (score_p1, scores_p2[scores_p2.len() / 2])
}

fn main() {
    let input_string = read_to_string("./inputs/day10.txt").expect("Input file not found");
    let lines = input_string.lines().collect::<Vec<_>>();
    let (p1, p2) = solve(&lines);
    println!("Part 1 solution: {}", p1);
    println!("Part 2 solution: {}", p2);
}
