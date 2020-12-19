use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");

type BoxResult<T> = Result<T, Box<dyn std::error::Error>>;

fn main() -> BoxResult<()> {
    let input: Result<Vec<_>, _> = INPUT.lines().map(|v| v.parse::<i64>()).collect();
    let input = input?;

    println!("Part1: {:?}", part1(&input));
    println!("Part2: {:?}", part2(&input));

    Ok(())
}

fn part1(input: &[i64]) -> Option<i64> {
    input
        .iter()
        .cartesian_product(input.iter().skip(1))
        .find(|&(x, y)| (x + y) == 2020)
        .map(|(x, y)| x * y)
}

fn part2(input: &[i64]) -> Option<i64> {
    input
        .iter()
        .cartesian_product(input.iter().skip(1))
        .cartesian_product(input.iter().skip(2))
        .find(|&((x, y), z)| (x + y + z) == 2020)
        .map(|((x, y), z)| x * y * z)
}
