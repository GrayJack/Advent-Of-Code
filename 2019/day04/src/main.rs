#![feature(is_sorted)]
use std::error::Error;

use itertools::Itertools;

fn main() -> Result<(), Box<dyn Error>> {
    let input = "128392-643281";
    let nums: Vec<u64> = input.trim().split('-').map(|e| e.parse::<u64>().unwrap()).collect();

    let range = nums[0]..=nums[1];

    println!("Part 1: {}", part1(range.clone()));
    println!("Part 2: {}", part2(range));

    Ok(())
}

fn part1(input: impl Iterator<Item = u64>) -> usize {
    input.filter(|e| digits_of(*e).is_sorted()).filter(|e| contain_double_adjacent(*e)).count()
}

fn part2(input: impl Iterator<Item = u64>) -> usize {
    input
        .filter(|e| digits_of(*e).is_sorted())
        .filter(|e| contain_double_adjacent(*e))
        .filter(|e| contain_triple_adjacent(*e))
        .count()
}

fn digits_of(num: u64) -> impl Iterator<Item = u64> + Clone {
    (0..6).map(move |exp| num / 10_u64.pow(exp) % 10).rev()
}

fn contain_double_adjacent(num: u64) -> bool {
    digits_of(num).tuple_windows().any(|(d1, d2)| d1 == d2)
}

fn contain_triple_adjacent(num: u64) -> bool {
    count_double_adjacent(num) > count_triple_adjacent(num)
}

fn count_double_adjacent(num: u64) -> usize {
    digits_of(num)
        .tuple_windows()
        .filter_map(|(d1, d2)| if d1 == d2 { Some(d1) } else { None })
        .unique()
        .count()
}

fn count_triple_adjacent(num: u64) -> usize {
    digits_of(num)
        .tuple_windows()
        .filter_map(|(d1, d2, d3)| if d1 == d2 && d1 == d3 { Some(d1) } else { None })
        .unique()
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(1, part1(111_111..=111_111));
        assert_eq!(0, part1(223_450..=223_450));
        assert_eq!(0, part1(123_789..=123_789));
    }

    #[test]
    fn test_part2() {
        assert_eq!(1, part2(112_233..=112_233));
        assert_eq!(0, part2(123_444..=123_444));
        assert_eq!(1, part2(111_122..=111_122));
    }
}
