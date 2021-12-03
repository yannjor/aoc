use std::cmp::Ordering;
use std::fs::read_to_string;

fn part_one(diagnostic_data: &[&[u8]]) -> u32 {
    let mut gamma_rate = String::new();
    let mut epsilon_rate = String::new();
    for i in 0..diagnostic_data[0].len() {
        let mut zero_count = 0;
        let mut one_count = 0;
        for binary_str in diagnostic_data {
            match binary_str[i] {
                48 => zero_count += 1, // '0' as bytes
                49 => one_count += 1,  // '1' as bytes
                _ => eprintln!("Failed to parse binary string bit"),
            }
        }
        if zero_count < one_count {
            gamma_rate.push('1');
            epsilon_rate.push('0');
        } else {
            gamma_rate.push('0');
            epsilon_rate.push('1');
        }
    }
    let gamma_rate = u32::from_str_radix(&gamma_rate, 2).unwrap();
    let epsilon_rate = u32::from_str_radix(&epsilon_rate, 2).unwrap();
    gamma_rate * epsilon_rate
}

fn get_rating(diagnostic_data: &[&[u8]], molecule: &str) -> u32 {
    let mut remaining = diagnostic_data.to_vec();
    for i in 0..diagnostic_data[0].len() {
        let mut zero_count = 0;
        let mut one_count = 0;
        for binary_str in &remaining {
            match binary_str[i] {
                48 => zero_count += 1, // '0' as bytes
                49 => one_count += 1,  // '1' as bytes
                _ => eprintln!("Failed to parse binary string bit"),
            }
        }
        let bit = match zero_count.cmp(&one_count) {
            Ordering::Less => match molecule {
                "oxygen" => 49,
                "co2" => 48,
                _ => panic!("Unknown molecule"),
            },
            Ordering::Greater => match molecule {
                "oxygen" => 48,
                "co2" => 49,
                _ => panic!("Unknown molecule"),
            },
            Ordering::Equal => match molecule {
                "oxygen" => 49,
                "co2" => 48,
                _ => panic!("Unknown molecule"),
            },
        };
        remaining.retain(|x| x[i] == bit);
        if remaining.len() == 1 {
            break;
        }
    }
    u32::from_str_radix(&String::from_utf8_lossy(remaining[0]), 2).unwrap()
}

fn part_two(diagnostic_data: &[&[u8]]) -> u32 {
    get_rating(diagnostic_data, "oxygen") * get_rating(diagnostic_data, "co2")
}

fn main() {
    let input_string = read_to_string("./inputs/day3.txt").expect("Input file not found");
    let diagnostic_data = input_string
        .lines()
        .map(|i| i.as_bytes())
        .collect::<Vec<&[u8]>>();
    println!("Part 1 solution: {}", part_one(&diagnostic_data));
    println!("Part 2 solution: {}", part_two(&diagnostic_data));
}
