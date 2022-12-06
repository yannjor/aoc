use std::collections::HashSet;
use std::fs::read_to_string;

fn solve(stream: &str, distinct_chars: usize) -> u32 {
    let chars: Vec<char> = stream.chars().collect();
    for i in 0..chars.len() - distinct_chars {
        let seq: HashSet<_> = chars[i..i + distinct_chars].iter().collect();
        if seq.len() == distinct_chars {
            return (i + distinct_chars) as u32;
        }
    }
    unreachable!("Could not find start of packet");
}

fn main() {
    let input_string = read_to_string("./inputs/day6.txt").expect("Input file not found");
    println!("Part 1 solution: {}", solve(&input_string, 4));
    println!("Part 2 solution: {}", solve(&input_string, 14));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(solve("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4), 7);
        assert_eq!(solve("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
        assert_eq!(solve("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), 10);
        assert_eq!(solve("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), 11);
    }
    #[test]
    fn test_part_two() {
        assert_eq!(solve("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), 19);
        assert_eq!(solve("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23);
        assert_eq!(solve("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14), 29);
        assert_eq!(solve("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14), 26);
    }
}
