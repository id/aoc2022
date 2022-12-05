use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input.splitn(9999, "\n\n").map(|s| s.lines().map(|l| l.parse::<u32>().unwrap()).collect()).collect()
}

#[aoc(day1, part1)]
fn solve_part1(input: &[Vec<u32>]) -> u32 {
    let mut max = 0;
    for elf in input {
        let tmp = elf.iter().sum();
        if tmp > max {
            max = tmp;
        }
    }
    max
}

#[aoc(day1, part2)]
fn solve_part2(input: &[Vec<u32>]) -> u32 {
    let n = input.len();
    let mut top = input.iter().map(|elf| elf.iter().sum::<u32>()).collect::<Vec<u32>>();
    top.sort();
    top[n-1] + top[n-2] + top[n-3]
}
