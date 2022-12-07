use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day4)]
fn parse_input(input: &str) -> Vec<Vec<i8>> {
    input.lines().map(|l| {
        l.split(",")
            .map(|s| s.split("-"))
            .flatten()
            .map(|i| i.parse::<i8>().unwrap())
            .collect()
    }).collect()
}

#[aoc(day4, part1)]
fn part1(input: &[Vec<i8>]) -> usize {
    input.iter().filter(|x| {
        let (l1, r1, l2, r2) = (x[0], x[1], x[2], x[3]);
        (l1 <= l2 && r1 >= r2) || (l2 <= l1 && r2 >= r1)
    }).count()
}

#[aoc(day4, part2)]
fn part2(input: &[Vec<i8>]) -> usize {
    input.iter().filter(|x| {
        let (l1, r1, l2, r2) = (x[0], x[1], x[2], x[3]);
        (l1 >= l2 && l1 <= r2) || (l2 >= l1 && l2 <= r1)
    }).count()
}
