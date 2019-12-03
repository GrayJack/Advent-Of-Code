#![feature(hash_set_entry)]
#![allow(dead_code)]
use std::{collections::{HashSet, HashMap}, env, error::Error, fs::File, io::Read, process, str::FromStr};

use crate::Direction::*;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = env::args().collect();
    if args.len() == 2 {
        let mut file = File::open(&args[1])?;
        let mut input = String::new();
        file.read_to_string(&mut input)?;

        let vec: Vec<_> = input.split_whitespace().collect();
        let in1: Vec<Direction> = vec[0].split(',').map(|e| e.parse().unwrap()).collect();
        let in2: Vec<Direction> = vec[1].split(',').map(|e| e.parse().unwrap()).collect();

        // Sub-optimal performance
        // println!("Part 1: {}", part1(&in1, &in2));
        // println!("Part 2: {}", part2(&in1, &in2));

        // More close to optimal performance
        let (p1, p2) = optimal(&in1, &in2);
        println!("Part 1: {}", p1);
        println!("Part 2: {}", p2);
    } else {
        eprintln!("Usage {} <INPUT_FILE>", args[0]);
        process::exit(1);
    }

    Ok(())
}

fn part1(ds1: &[Direction], ds2: &[Direction]) -> i64 {
    let w1 = create_wire(ds1);
    let w2 = create_wire(ds2);

    // Minor point
    let (a, b) = w1.intersection(&w2).min_by_key(|(a, b)| a.abs() + b.abs()).unwrap();
    a.abs() + b.abs()
}

fn part2(ds1: &[Direction], ds2: &[Direction]) -> i64 {
    let w1 = create_wire_save_path(ds1);
    let w2 = create_wire_save_path(ds2);
    let mut ans = std::i64::MAX;

    for (pos1, sum1) in &w1 {
        if let Some(sum2) = w2.get(pos1) {
            let possible_ans = sum1 + sum2;

            if ans > possible_ans {
                ans = possible_ans;
            }
        }
    }

    ans
}

fn optimal(ds1: &[Direction], ds2: &[Direction]) -> (i64, i64) {
    let w1 = create_wire_save_path(ds1);
    let w2 = create_wire_save_path(ds2);
    let mut ans_dist = std::i64::MAX;
    let mut ans_sum = std::i64::MAX;

    for (pos1, sum1) in &w1 {
        if let Some(sum2) = w2.get(pos1) {
            let possible_ans_dist = pos1.0.abs() + pos1.1.abs();

            if ans_dist > possible_ans_dist {
                ans_dist = possible_ans_dist;
            }

            let possible_ans_sum = sum1 + sum2;
            if ans_sum > possible_ans_sum {
                ans_sum = possible_ans_sum;
            }
        }
    }

    (ans_dist, ans_sum)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    Up(usize),
    Left(usize),
    Right(usize),
    Down(usize),
}

impl FromStr for Direction {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dir_letter = &s[0..1];
        let num = &s[1..];

        match dir_letter {
            "U" | "u" => Ok(Self::Up(num.parse()?)),
            "R" | "r" => Ok(Self::Right(num.parse()?)),
            "L" | "l" => Ok(Self::Left(num.parse()?)),
            "D" | "d" => Ok(Self::Down(num.parse()?)),
            _ => Err(Box::new(std::io::Error::last_os_error())),
        }
    }
}

fn create_wire(ds: &[Direction]) -> HashSet<(i64, i64)> {
    let mut set = HashSet::new();
    let mut pos = (0, 0);

    for dir in ds {
        match dir {
            Up(x) => {
                for i in 1..=*x {
                    set.get_or_insert((pos.0, pos.1 + i as i64));
                }
                pos.1 += *x as i64;
            },
            Right(x) => {
                for i in 1..=*x {
                    set.get_or_insert((pos.0 + i as i64, pos.1));
                }
                pos.0 += *x as i64;
            },
            Down(x) => {
                for i in 1..=*x {
                    set.get_or_insert((pos.0, pos.1 - i as i64));
                }
                pos.1 -= *x as i64;
            },
            Left(x) => {
                for i in 1..=*x {
                    set.get_or_insert((pos.0 - i as i64, pos.1));
                }
                pos.0 -= *x as i64;
            },
        }
    }

    set
}

fn create_wire_save_path(ds: &[Direction]) -> HashMap<(i64, i64), i64> {
    let mut map = HashMap::new();
    let mut pos = (0, 0);
    let mut dis = 0;

    for dir in ds {
        match dir {
            Up(x) => {
                for i in 1..=*x {
                    dis += 1;
                    map.entry((pos.0, pos.1 + i as i64)).or_insert(dis);
                }
                pos.1 += *x as i64;
            },
            Right(x) => {
                for i in 1..=*x {
                    dis += 1;
                    map.entry((pos.0 + i as i64, pos.1)).or_insert(dis);
                }
                pos.0 += *x as i64;
            },
            Down(x) => {
                for i in 1..=*x {
                    dis += 1;
                    map.entry((pos.0, pos.1 - i as i64)).or_insert(dis);
                }
                pos.1 -= *x as i64;
            },
            Left(x) => {
                for i in 1..=*x {
                    dis += 1;
                    map.entry((pos.0 - i as i64, pos.1)).or_insert(dis);
                }
                pos.0 -= *x as i64;
            },
        }
    }

    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            6,
            part1(&[Right(8), Up(5), Left(5), Down(3)], &[Up(7), Right(6), Down(4), Left(4)])
        );
        assert_eq!(
            159,
            part1(
                &[
                    Right(75),
                    Down(30),
                    Right(83),
                    Up(83),
                    Left(12),
                    Down(49),
                    Right(71),
                    Up(7),
                    Left(72)
                ],
                &[Up(62), Right(66), Up(55), Right(34), Down(71), Right(55), Down(58), Right(83)]
            )
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            30,
            part2(&[Right(8), Up(5), Left(5), Down(3)], &[Up(7), Right(6), Down(4), Left(4)])
        );
        assert_eq!(
            610,
            part2(
                &[
                    Right(75),
                    Down(30),
                    Right(83),
                    Up(83),
                    Left(12),
                    Down(49),
                    Right(71),
                    Up(7),
                    Left(72)
                ],
                &[Up(62), Right(66), Up(55), Right(34), Down(71), Right(55), Down(58), Right(83)]
            )
        );
    }

    #[test]
    fn test_optimal() {
        assert_eq!(
            (6, 30),
            optimal(&[Right(8), Up(5), Left(5), Down(3)], &[Up(7), Right(6), Down(4), Left(4)])
        );
        assert_eq!(
            (159, 610),
            optimal(
                &[
                    Right(75),
                    Down(30),
                    Right(83),
                    Up(83),
                    Left(12),
                    Down(49),
                    Right(71),
                    Up(7),
                    Left(72)
                ],
                &[Up(62), Right(66), Up(55), Right(34), Down(71), Right(55), Down(58), Right(83)]
            )
        );
    }
}
