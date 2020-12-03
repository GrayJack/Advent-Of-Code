const INPUT: &str = include_str!("input.txt");

fn main() {
    let input = INPUT.lines().map(Password::new);
    let in2 = input.clone();

    println!("Part 1: {:?}", part1(input));
    println!("Part 2: {:?}", part2(in2));
}

#[derive(Debug)]
struct Password {
    min:    u8,
    max:    u8,
    ch:     char,
    passwd: String,
}

impl Password {
    fn new(string: impl AsRef<str>) -> Self {
        let mut split = string.as_ref().split_whitespace();

        let range = split
            .next()
            .and_then(|s| {
                s.split('-')
                    .map(|val| val.parse::<u8>().ok())
                    .collect::<Option<Vec<_>>>()
            })
            .unwrap();
        let ch = split
            .next()
            .and_then(|val| val.strip_suffix(':')?.parse::<char>().ok())
            .unwrap();
        let passwd = split.next().unwrap().to_owned();

        Self {
            min: range[0],
            max: range[1],
            ch,
            passwd,
        }
    }

    fn is_valid_wrong_rule(&self) -> bool {
        let num_ch = self
            .passwd
            .chars()
            .fold(0, |acc, c| if c == self.ch { acc + 1 } else { acc });

        self.min <= num_ch && num_ch <= self.max
    }

    fn is_valid(&self) -> bool {
        let chars: Vec<_> = self.passwd.chars().collect();
        match (
            chars.get((self.min - 1) as usize),
            chars.get((self.max - 1) as usize),
        ) {
            (None, None) => false,
            (None, Some(&c)) | (Some(&c), None) => c == self.ch,
            (Some(&c1), Some(&c2)) => {
                if c1 == self.ch && c2 == self.ch {
                    false
                } else {
                    !(c1 != self.ch && c2 != self.ch)
                }
            },
        }
    }
}

fn part1(input: impl Iterator<Item = Password>) -> usize {
    input.filter(Password::is_valid_wrong_rule).count()
}

fn part2(input: impl Iterator<Item = Password>) -> usize {
    input.filter(|s| s.is_valid()).count()
}
