use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs::read_to_string;

fn get_points_covered_by_diagonal_line(line: ((u32, u32), (u32, u32))) -> Vec<(u32, u32)> {
    let start_point = min(line.0, line.1);
    let end_point = max(line.0, line.1);
    let slope =
        (end_point.1 as f32 - start_point.1 as f32) / (end_point.0 as f32 - start_point.0 as f32);
    let mut point = start_point;
    let mut points = Vec::new();
    let mut i = 1;
    while point != end_point {
        points.push(point);
        if slope < 0.0 {
            point = (start_point.0 + i, start_point.1 - i);
        } else {
            point = (start_point.0 + i, start_point.1 + i);
        }
        i += 1;
    }
    points.push(end_point);
    points
}

fn get_points_covered_by_straight_line(line: ((u32, u32), (u32, u32))) -> Vec<(u32, u32)> {
    if line.0 < line.1 {
        if line.0 .0 == line.1 .0 {
            let points = (line.0 .1..line.1 .1 + 1).fold(Vec::new(), |mut v, x| {
                v.push((line.0 .0, x));
                v
            });
            return points;
        } else {
            let points = (line.0 .0..line.1 .0 + 1).fold(Vec::new(), |mut v, x| {
                v.push((x, line.0 .1));
                v
            });
            return points;
        }
    } else {
        if line.0 .0 == line.1 .0 {
            let points = (line.1 .1..line.0 .1 + 1).fold(Vec::new(), |mut v, x| {
                v.push((line.0 .0, x));
                v
            });
            return points;
        } else {
            let points = (line.1 .0..line.0 .0 + 1).fold(Vec::new(), |mut v, x| {
                v.push((x, line.0 .1));
                v
            });
            return points;
        }
    }
}

fn is_straight(line: ((u32, u32), (u32, u32))) -> bool {
    line.0 .0 == line.1 .0 || line.0 .1 == line.1 .1
}

fn count_overlaps(points: &[(u32, u32)]) -> u32 {
    let mut occurences = HashMap::new();
    for point in points {
        let counter = occurences.entry(point).or_insert(0);
        *counter += 1;
    }
    occurences.iter().filter(|(_, count)| **count >= 2).count() as u32
}

fn part_one(lines: &[((u32, u32), (u32, u32))]) -> u32 {
    let straight_lines = lines
        .iter()
        .filter(|l| is_straight(**l))
        .collect::<Vec<_>>();
    let points = straight_lines
        .iter()
        .map(|l| get_points_covered_by_straight_line(**l))
        .collect::<Vec<_>>()
        .concat();
    count_overlaps(&points)
}

fn part_two(lines: &[((u32, u32), (u32, u32))]) -> u32 {
    let points = lines
        .iter()
        .map(|l| {
            if is_straight(*l) {
                get_points_covered_by_straight_line(*l)
            } else {
                get_points_covered_by_diagonal_line(*l)
            }
        })
        .collect::<Vec<_>>()
        .concat();
    count_overlaps(&points)
}
fn main() {
    let input_string = read_to_string("./inputs/day5.txt").expect("Input file not found");
    let lines = input_string
        .lines()
        .map(|l| {
            let (xs, ys) = l.split_once(" -> ").unwrap();
            let (x1, x2) = xs.split_once(",").unwrap();
            let (y1, y2) = ys.split_once(",").unwrap();
            (
                (x1.parse::<u32>().unwrap(), x2.parse::<u32>().unwrap()),
                (y1.parse::<u32>().unwrap(), y2.parse::<u32>().unwrap()),
            )
        })
        .collect::<Vec<_>>();
    println!("Part 1 solution: {}", part_one(&lines));
    println!("Part 2 solution: {}", part_two(&lines));
}
