use aoc_runner_derive::{aoc, aoc_generator};

fn priority(ch: char) -> u32 {
    let code = ch as u32;
    if code <= 90 {
        code - 38
    } else {
        code - 96
    }
}

#[aoc_generator(day3, part1)]
fn parse_input1(input: &str) -> Vec<u32> {
    input.lines().map(|l| {
        let (left, right) = l.split_at(l.len() / 2);
        let mut items: [u8; 123] = [0; 123];
        for ch in left.chars() {
            items[ch as usize] = 1
        }
        let mut prio: u32 = 0;
        for ch in right.chars() {
            if items[ch as usize] == 1 {
                prio = priority(ch);
                break
            }
        }
        prio
    }).collect()
}

#[aoc(day3, part1)]
fn part1(input: &[u32]) -> u32 {
    input.iter().sum()
}

#[aoc_generator(day3, part2)]
fn parse_input2(input: &str) -> Vec<u32> {
    let lines: Vec<_> = input.lines().collect();
    let mut res = vec![0; lines.len()/3];
    let mut j = 0;
    for i in (0..lines.len()).step_by(3) {
        let mut items1: [u8; 123] = [0; 123];
        for ch in lines[i].chars() {
            items1[ch as usize] = 1
        }
        let mut items2: [u8; 123] = [0; 123];
        for ch in lines[i+1].chars() {
            items2[ch as usize] = 1
        }
        for ch in lines[i+2].chars() {
            if items1[ch as usize] == 1 && items2[ch as usize] == 1  {
                res[j] = priority(ch);
                j += 1;
                break
            }
        }
    }
    res
}

#[aoc(day3, part2)]
fn part2(input: &[u32]) -> u32 {
    input.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_input1_test() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(
            parse_input1(input),
            [16,38,42,22,20,19]
        );
    }

    #[test]
    fn parse_input2_test() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(
            parse_input2(input),
            [18, 52]
        );
    }
}
