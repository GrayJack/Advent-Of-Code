use std::{error::Error, fs::File, io::Read, env};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = env::args().collect();
    if args.len() == 2 {
        let mut file = File::open(&args[1])?;
        let mut input = String::new();
        file.read_to_string(&mut input)?;

        println!("Part 1: {}", part1(&input));
        println!("Part 2: {}", part2(&input));
    } else {
        eprintln!("Usage {} <INPUT_FILE>", args[0]);
    }

    Ok(())
}

fn part1(str: &str) -> i128 {
    str.split_whitespace().map(|e| e.parse::<i128>().unwrap()).map(|e| (e / 3) - 2).sum()
}

fn part2(str: &str) -> i128 {
    str.split_whitespace()
        .map(|e| e.parse::<i128>().unwrap())
        .map(|e| (e / 3) - 2)
        .map(|mut e| {
            let mut ans = e;
            while ans >= 0 {
                ans = (ans / 3) - 2;
                if ans > 0 {
                    e += ans;
                }
            }
            e
        })
        .sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(2, part1("12"));
        assert_eq!(2, part1("14"));
        assert_eq!(654, part1("1969"));
        assert_eq!(33583, part1("100756"));
    }

    #[test]
    fn test_part2() {
        assert_eq!(2, part2("12"));
        assert_eq!(2, part2("14"));
        assert_eq!(966, part2("1969"));
        assert_eq!(50346, part2("100756"));
    }
}
