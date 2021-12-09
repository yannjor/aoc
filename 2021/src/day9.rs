use std::collections::{HashSet, VecDeque};
use std::fs::read_to_string;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Point {
    row: usize,
    col: usize,
    height: u32,
}

fn get_adjacent_points(height_map: &[Vec<u32>], row: usize, col: usize) -> [Point; 4] {
    let out_of_bounds = Point {
        row: 0,
        col: 0,
        height: u32::MAX,
    };
    let left = if col == 0 {
        out_of_bounds
    } else {
        Point {
            row,
            col: col - 1,
            height: height_map[row][col - 1],
        }
    };
    let top = if row == 0 {
        out_of_bounds
    } else {
        Point {
            row: row - 1,
            col,
            height: height_map[row - 1][col],
        }
    };
    let right = if col == height_map[0].len() - 1 {
        out_of_bounds
    } else {
        Point {
            row,
            col: col + 1,
            height: height_map[row][col + 1],
        }
    };
    let bottom = if row == height_map.len() - 1 {
        out_of_bounds
    } else {
        Point {
            row: row + 1,
            col,
            height: height_map[row + 1][col],
        }
    };
    [left, top, right, bottom]
}

fn get_low_points(height_map: &[Vec<u32>]) -> Vec<Point> {
    let mut low_points = Vec::new();
    for (i, row) in height_map.iter().enumerate() {
        for (j, height) in row.iter().enumerate() {
            if get_adjacent_points(height_map, i, j)
                .iter()
                .all(|p| p.height > *height)
            {
                low_points.push(Point {
                    row: i,
                    col: j,
                    height: *height,
                });
            }
        }
    }
    low_points
}

fn get_basin_size(height_map: &[Vec<u32>], low_point: Point) -> u32 {
    // BFS from low_point
    let mut queue = VecDeque::from([low_point]);
    let mut visited = HashSet::new();
    visited.insert(low_point);
    while !queue.is_empty() {
        let current_point = queue.pop_front().unwrap();
        for p in get_adjacent_points(height_map, current_point.row, current_point.col) {
            if p.height > current_point.height && p.height < 9 && !visited.contains(&p) {
                queue.push_back(p);
                visited.insert(p);
            }
        }
    }
    visited.len() as u32
}

fn part_one(height_map: &[Vec<u32>]) -> u32 {
    get_low_points(height_map)
        .iter()
        .map(|p| p.height + 1)
        .sum()
}

fn part_two(height_map: &[Vec<u32>]) -> u32 {
    let low_points = get_low_points(height_map);
    let mut basin_sizes: Vec<u32> = low_points
        .iter()
        .map(|p| get_basin_size(height_map, *p))
        .collect();
    basin_sizes.sort_unstable_by(|a, b| b.cmp(a));
    basin_sizes[..3].iter().product::<u32>()
}

fn main() {
    let input_string = read_to_string("./inputs/day9.txt").expect("Input file not found");
    let height_map = input_string
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    println!("Part 1 solution: {}", part_one(&height_map));
    println!("Part 2 solution: {}", part_two(&height_map));
}
