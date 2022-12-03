use std::fs::read_to_string;

fn get_unmarked_sum(drawn_numbers: &[u32], board: &[Vec<u32>]) -> u32 {
    board
        .concat()
        .iter()
        .filter(|x| !drawn_numbers.contains(x))
        .sum::<u32>()
}

fn get_score(drawn_numbers: &[u32], board: &[Vec<u32>]) -> u32 {
    // Check rows
    for row in board {
        let bingo = row.iter().all(|num| drawn_numbers.contains(num));
        if bingo {
            return drawn_numbers.last().unwrap() * get_unmarked_sum(drawn_numbers, board);
        }
    }
    // Check columns
    let mut columns: Vec<Vec<u32>> = Vec::new();
    board.iter().enumerate().for_each(|(i, _)| {
        columns.push(Vec::new());
        board[i]
            .iter()
            .enumerate()
            .for_each(|(j, _)| columns[i].push(board[j][i]))
    });
    for col in columns {
        let bingo = col.iter().all(|num| drawn_numbers.contains(num));
        if bingo {
            return drawn_numbers.last().unwrap() * get_unmarked_sum(drawn_numbers, board);
        }
    }
    0
}

fn part_one(bingo_numbers: &[u32], boards: &[Vec<Vec<u32>>]) -> u32 {
    let mut drawn_numbers = Vec::new();
    for num in bingo_numbers {
        drawn_numbers.push(*num);
        for board in boards {
            let score = get_score(&drawn_numbers, board);
            if score != 0 {
                return score;
            }
        }
    }
    0
}

fn part_two(bingo_numbers: &[u32], boards: &[Vec<Vec<u32>>]) -> u32 {
    let mut drawn_numbers = Vec::new();
    let mut already_won = Vec::new();
    for num in bingo_numbers {
        drawn_numbers.push(*num);
        for (i, board) in boards.iter().enumerate() {
            let score = get_score(&drawn_numbers, board);
            if score != 0 && !already_won.contains(&i) {
                already_won.push(i);
            }
            if already_won.len() == boards.len() {
                return score;
            }
        }
    }
    0
}

fn main() {
    let input_string = read_to_string("./inputs/day4.txt").expect("Input file not found");
    let mut iter = input_string.split("\n\n");
    let bingo_numbers = iter
        .next()
        .unwrap()
        .split(',')
        .map(|num| num.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let boards = iter
        .map(|board| {
            board
                .lines()
                .map(|row| {
                    row.split_whitespace()
                        .map(|num| num.parse::<u32>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!("Part 1 solution: {}", part_one(&bingo_numbers, &boards));
    println!("Part 2 solution: {}", part_two(&bingo_numbers, &boards));
}
