use std::{
    collections::HashMap,
    error::Error,
    io::{stdin, Read},
    str::FromStr,
};

use lazy_static::lazy_static;
use regex::Regex;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

#[derive(Debug)]
struct Claim {
    id:       u32,
    x_inches: u32,
    y_inches: u32,
    x_size:   u32,
    y_size:   u32,
}

impl Claim {
    fn new(id: u32, x_inches: u32, y_inches: u32, x_size: u32, y_size: u32) -> Claim {
        Claim { id, x_inches, y_inches, x_size, y_size }
    }

    fn iter_points(&self) -> IterPoints {
        IterPoints { claim: self, pt_x: self.x_inches, pt_y: self.y_inches }
    }
}

impl FromStr for Claim {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Claim, Box<dyn Error>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(?x)
                \#
                (?P<id>[0-9]+)
                \s+@\s+
                (?P<x>[0-9]+),(?P<y>[0-9]+):
                \s+
                (?P<width>[0-9]+)x(?P<height>[0-9]+)
            "
            )
            .unwrap();
        }

        let caps = match RE.captures(s) {
            None => return err!("unrecognized claim"),
            Some(caps) => caps,
        };

        Ok(Claim::new(
            caps["id"].parse()?,
            caps["x"].parse()?,
            caps["y"].parse()?,
            caps["width"].parse()?,
            caps["height"].parse()?,
        ))
    }
}

struct IterPoints<'a> {
    claim: &'a Claim,
    pt_x:  u32,
    pt_y:  u32,
}

impl<'a> Iterator for IterPoints<'a> {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<(u32, u32)> {
        if self.pt_y >= self.claim.y_inches + self.claim.y_size {
            self.pt_y = self.claim.y_inches;
            self.pt_x += 1;
        }

        if self.pt_x >= self.claim.x_inches + self.claim.x_size {
            return None;
        }

        let (px, py) = (self.pt_x, self.pt_y);
        self.pt_y += 1;
        Some((px, py))
    }
}

fn first_part(grid: &HashMap<(u32, u32), u32>) -> Result<(), Box<dyn Error>> {
    let count = grid.values().filter(|&&v| v > 1).count();
    println!("{}", count);
    Ok(())
}

fn second_part(claims: &[Claim], grid: &HashMap<(u32, u32), u32>) -> Result<(), Box<dyn Error>> {
    for claim in claims {
        if claim.iter_points().all(|pt| grid[&pt] == 1) {
            println!("Uncontested claim: {}", claim.id);
            return Ok(());
        }
    }
    err!("No uncontested claim")
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let mut claims: Vec<Claim> = Vec::new();
    for line in input.lines() {
        let claim = line.parse().or_else(|err| err!("failed to parse '{:?}': {}", line, err))?;
        claims.push(claim);
    }

    let mut grid: HashMap<(u32, u32), u32> = HashMap::new();
    for claim in &claims {
        for (x, y) in claim.iter_points() {
            *grid.entry((x, y)).or_default() += 1;
        }
    }

    first_part(&grid)?;
    second_part(&claims, &grid)?;

    Ok(())
}
