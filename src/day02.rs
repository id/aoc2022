use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2, part1)]
fn parse_input1(input: &str) -> Vec<u32> {
    input.lines().map(|l| match l {
        "A X" => 4, // 1 + (R / R) = 4
        "A Y" => 8, // 2 + (R / P) = 8
        "A Z" => 3, // 3 + (R / S) = 3
        "B X" => 1, // 1 + (P / R) = 1
        "B Y" => 5, // 2 + (P / P) = 5
        "B Z" => 9, // 3 + (P / S) = 9
        "C X" => 7, // 1 + (S / R) = 7
        "C Y" => 2, // 2 + (S / P) = 2
        "C Z" => 6, // 3 + (S / S) = 6
        _ => 0
    }).collect()
}

#[aoc(day2, part1)]
fn solve_part1(input: &[u32]) -> u32 {
    input.iter().sum()
}

#[aoc_generator(day2, part2)]
fn parse_input2(input: &str) -> Vec<u32> {
    input.lines().map(|l| match l {
        "A X" => 3, // 0 + 3
        "A Y" => 4, // 3 + 1
        "A Z" => 8, // 6 + 2
        "B X" => 1, // 0 + 1
        "B Y" => 5, // 3 + 2
        "B Z" => 9, // 6 + 3
        "C X" => 2, // 0 + 2
        "C Y" => 6, // 3 + 3
        "C Z" => 7, // 6 + 1
        _ => 0
    }).collect()
}

#[aoc(day2, part2)]
fn solve_part2(input: &[u32]) -> u32 {
    input.iter().sum()
}
