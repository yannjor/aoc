use std::collections::HashSet;
use std::fs::read_to_string;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Octopus {
    row: usize,
    col: usize,
    energy_level: u8,
}

fn get_adjacent(row: usize, col: usize) -> Vec<(usize, usize)> {
    let deltas: [(i8, i8); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let mut adjacent = Vec::new();
    for (dx, dy) in deltas {
        let cur_x = row as i8 + dx;
        let cur_y = col as i8 + dy;
        if cur_x < 0 || cur_y < 0 || cur_x > 9 || cur_y > 9 {
            continue;
        }
        adjacent.push((cur_x as usize, cur_y as usize));
    }
    adjacent
}

fn step(octopuses: &mut [Vec<Octopus>]) -> u32 {
    let mut ready_to_flash = Vec::new();
    let mut visited = HashSet::new();
    let mut flashes = 0;

    octopuses.iter_mut().for_each(|r| {
        r.iter_mut().for_each(|o| {
            o.energy_level += 1;
            if o.energy_level == 10 {
                ready_to_flash.push(*o);
                visited.insert(*o);
            }
        })
    });

    while !ready_to_flash.is_empty() {
        let current = ready_to_flash.pop().unwrap();
        for (i, j) in get_adjacent(current.row, current.col) {
            let adjacent = &mut octopuses[i][j];
            adjacent.energy_level += 1;
            if adjacent.energy_level == 10 && !visited.contains(adjacent) {
                ready_to_flash.push(*adjacent);
                visited.insert(*adjacent);
            }
        }
    }
    for row in octopuses.iter_mut() {
        for octopus in row.iter_mut() {
            if octopus.energy_level >= 10 {
                octopus.energy_level = 0;
                flashes += 1;
            }
        }
    }
    flashes
}

fn part_one(octopuses: &[Vec<Octopus>]) -> u32 {
    let mut octopuses_mut = octopuses.to_owned();
    (0..100).map(|_| step(&mut octopuses_mut)).sum()
}

fn part_two(octopuses: &[Vec<Octopus>]) -> u32 {
    let mut octopuses_mut = octopuses.to_owned();
    let mut i = 1;
    while step(&mut octopuses_mut) != 100 {
        i += 1;
    }
    i
}

fn main() {
    let input_string = read_to_string("./inputs/day11.txt").expect("Input file not found");
    let octopuses = input_string
        .lines()
        .enumerate()
        .map(|(i, l)| {
            l.chars()
                .enumerate()
                .map(|(j, c)| Octopus {
                    row: i,
                    col: j,
                    energy_level: c.to_digit(10).unwrap() as u8,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    println!("Part 1 solution: {}", part_one(&octopuses));
    println!("Part 2 solution: {}", part_two(&octopuses));
}
