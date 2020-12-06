use std::iter::FusedIterator;

const INPUT: &str = include_str!("input.txt");


fn main() {
    let input: Vec<Vec<char>> = INPUT.lines().map(|s| s.chars().collect()).collect();

    let part1 = Walker::new(&input, 3, 1)
        .filter(|&block| block == BlockKind::Tree)
        .count();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2(&input));
}

fn part2(map: &[Vec<char>]) -> usize {
    let walkers = vec![
        Walker::new(map, 1, 1),
        Walker::new(map, 3, 1),
        Walker::new(map, 5, 1),
        Walker::new(map, 7, 1),
        Walker::new(map, 1, 2),
    ];

    walkers
        .into_iter()
        .map(|walker| walker.filter(|&block| block == BlockKind::Tree).count())
        .fold(1, |acc, item| acc * item)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
enum BlockKind {
    Open,
    Tree,
}

#[derive(Debug)]
struct Walker<'a> {
    map: &'a [Vec<char>],
    horizontal_step: usize,
    vertical_step: usize,
    x: usize,
    y: usize,
}

impl<'a> Walker<'a> {
    fn new(map: &'a [Vec<char>], horizontal_step: usize, vertical_step: usize) -> Self {
        Self {
            map,
            horizontal_step,
            vertical_step,
            x: 0,
            y: 0,
        }
    }
}

impl<'a> Iterator for Walker<'a> {
    type Item = BlockKind;

    fn next(&mut self) -> Option<Self::Item> {
        self.map.get(self.y).and_then(|row| {
            self.y += self.vertical_step;

            row.get(self.x % row.len()).and_then(|c| {
                self.x += self.horizontal_step;
                match c {
                    '.' => Some(BlockKind::Open),
                    '#' => Some(BlockKind::Tree),
                    _ => None,
                }
            })
        })
    }
}

impl FusedIterator for Walker<'_> {}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    #[test]
    fn it_works() {
        let input: Vec<Vec<char>> = EXAMPLE.lines().map(|s| s.chars().collect()).collect();

        let part1 = Walker::new(input.as_ref(), 3, 1)
            .filter(|&block| block == BlockKind::Tree)
            .count();

        assert_eq!(part1, 7);
    }
}
