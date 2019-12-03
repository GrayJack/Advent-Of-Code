use std::{env, error::Error, fs::File, io::Read};
use sugars::cvec;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = env::args().collect();
    if args.len() == 2 {
        let mut file = File::open(&args[1])?;
        let mut input = String::new();
        file.read_to_string(&mut input)?;

        let mut vec: Vec<_> =
            input.trim().split(',').map(|e| e.parse::<i64>().ok().unwrap()).collect();
        let vec2 = vec.clone();

        vec[1] = 12;
        vec[2] = 2;

        println!("Part 1: {}", part1(&mut vec));
        println!("Part 2: {}", part2(&vec2));
        println!("Part 2 (right way): {}", part2_right(vec2, 19_690_720));
    } else {
        eprintln!("Usage: {} <INPUT_FILE>", &args[0]);
    }

    Ok(())
}

fn part1(vec: &mut [i64]) -> i64 {
    let mut index = 0;
    loop {
        let (pos1, pos2, to) =
            (vec[index + 1] as usize, vec[index + 2] as usize, vec[index + 3] as usize);
        match vec[index] {
            1 => vec[to] = vec[pos1] + vec[pos2],
            2 => vec[to] = vec[pos1] * vec[pos2],
            99 => break,
            _ => break,
        }
        index += 4;
    }

    vec[0]
}

fn part2(vec: &[i64]) -> i64 {
    let mut ans = 0;
    for noun in 0..100 {
        for verb in 0..100 {
            let mut memory = cvec![*x; x <- vec.iter()];
            memory[1] = noun;
            memory[2] = verb;
            let mut index = 0;
            loop {
                let (pos1, pos2, to) = (
                    memory[index + 1] as usize,
                    memory[index + 2] as usize,
                    memory[index + 3] as usize,
                );
                match memory[index] {
                    1 => memory[to] = memory[pos1] + memory[pos2],
                    2 => memory[to] = memory[pos1] * memory[pos2],
                    99 => break,
                    _ => break,
                }
                index += 4;

                if memory[0] == 19_690_720 {
                    ans = 100 * noun + verb;
                }
            }
        }
    }
    ans
}

fn part2_right(mem: Vec<i64>, wanted: i64) -> i64 {
    let mut program = mem;
    program[1] = 0;
    program[2] = 0;

    let offset = part1(&mut program.clone());
    let raw = wanted - offset;

    program[1] = 1;
    let multiplier = part1(&mut program) - offset;

    100 * (raw / multiplier) + (raw % multiplier)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(2, part1(&mut [1, 0, 0, 0, 99]));
        assert_eq!(2, part1(&mut [2, 3, 0, 3, 99]));
        assert_eq!(2, part1(&mut [2, 4, 4, 5, 99, 0]));
        assert_eq!(30, part1(&mut [1, 1, 1, 4, 99, 5, 6, 0, 99]));
    }
}
