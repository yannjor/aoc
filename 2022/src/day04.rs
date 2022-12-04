use std::fs::read_to_string;
use std::ops::Range;

fn part_one(sections: &[(Range<u32>, Range<u32>)]) -> u32 {
    sections
        .iter()
        .filter(|s| {
            let mut i1 = s.0.start..s.0.end;
            let mut i2 = s.1.start..s.1.end;
            i1.all(|i| s.1.contains(&i)) || i2.all(|i| s.0.contains(&i))
        })
        .count() as u32
}

fn part_two(sections: &[(Range<u32>, Range<u32>)]) -> u32 {
    sections
        .iter()
        .filter(|s| {
            let mut i1 = s.0.start..s.0.end;
            let mut i2 = s.1.start..s.1.end;
            i1.any(|i| s.1.contains(&i)) || i2.any(|i| s.0.contains(&i))
        })
        .count() as u32
}

fn main() {
    let input_string = read_to_string("./inputs/day4.txt").expect("Input file not found");
    let sections = input_string
        .lines()
        .map(|l| {
            let p = l.split_once(',').expect("Failed to parse input");
            let section1 = p.0.split_once('-').expect("Failed to parse section");
            let section2 = p.1.split_once('-').expect("Failed to parse section");
            let interval1 =
                section1.0.parse::<u32>().unwrap()..section1.1.parse::<u32>().unwrap() + 1;
            let interval2 =
                section2.0.parse::<u32>().unwrap()..section2.1.parse::<u32>().unwrap() + 1;
            (interval1, interval2)
        })
        .collect::<Vec<_>>();
    println!("Part 1 solution: {}", part_one(&sections));
    println!("Part 2 solution: {}", part_two(&sections));
}
