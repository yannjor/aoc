use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(PartialOrd, PartialEq, Clone, Eq, Hash, Copy, Debug, Ord)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Copy, Clone, Debug)]
struct Line {
    start: Point,
    end: Point,
}

fn get_points_covered_by_diagonal_line(line: Line) -> Vec<Point> {
    let start_point = min(line.start, line.end);
    let end_point = max(line.start, line.end);
    let slope =
        (end_point.y as f32 - start_point.y as f32) / (end_point.x as f32 - start_point.x as f32);
    let mut point = start_point;
    let mut points = Vec::new();
    let mut i = 1;
    while point != end_point {
        points.push(point);
        if slope < 0_f32 {
            point = Point {
                x: start_point.x + i,
                y: start_point.y - i,
            };
        } else {
            point = Point {
                x: start_point.x + i,
                y: start_point.y + i,
            };
        }
        i += 1;
    }
    points.push(end_point);
    points
}

fn get_points_covered_by_straight_line(line: Line) -> Vec<Point> {
    if line.start < line.end {
        if line.start.x == line.end.x {
            (line.start.y..line.end.y + 1)
                .map(|i| Point {
                    x: line.start.x,
                    y: i,
                })
                .collect::<Vec<_>>()
        } else {
            (line.start.x..line.end.x + 1)
                .map(|i| Point {
                    x: i,
                    y: line.start.y,
                })
                .collect::<Vec<_>>()
        }
    } else if line.start.x == line.end.x {
        (line.end.y..line.start.y + 1)
            .map(|i| Point {
                x: line.start.x,
                y: i,
            })
            .collect::<Vec<_>>()
    } else {
        (line.end.x..line.start.x + 1)
            .map(|i| Point {
                x: i,
                y: line.start.y,
            })
            .collect::<Vec<_>>()
    }
}

fn is_straight(line: Line) -> bool {
    line.start.x == line.end.x || line.start.y == line.end.y
}

fn count_overlaps(points: &[Point]) -> u32 {
    let mut occurences = HashMap::new();
    for point in points {
        let counter = occurences.entry(point).or_insert(0);
        *counter += 1;
    }
    occurences.iter().filter(|(_, count)| **count >= 2).count() as u32
}

fn part_one(lines: &[Line]) -> u32 {
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

fn part_two(lines: &[Line]) -> u32 {
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
            let (p1, p2) = l.split_once(" -> ").unwrap();
            let (x1, y1) = p1.split_once(",").unwrap();
            let (x2, y2) = p2.split_once(",").unwrap();
            Line {
                start: Point {
                    x: x1.parse::<u32>().unwrap(),
                    y: y1.parse::<u32>().unwrap(),
                },
                end: Point {
                    x: x2.parse::<u32>().unwrap(),
                    y: y2.parse::<u32>().unwrap(),
                },
            }
        })
        .collect::<Vec<_>>();
    println!("Part 1 solution: {}", part_one(&lines));
    println!("Part 2 solution: {}", part_two(&lines));
}
