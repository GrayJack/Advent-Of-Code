#![feature(str_split_once)]
use std::{collections::HashMap, str::FromStr};

const INPUT: &str = include_str!("input.txt");

static REQUIRED_KEYS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

fn main() {
    let iter = INPUT.split("\n\n").map(|s| s.parse::<Passport>());
    println!("Part 1: {}", part1(iter.clone()));
    println!("Part 2: {}", part2(iter));
}

fn part1<I>(iter: I) -> usize
where I: Iterator<Item = Result<Passport, PassportParseError>> {
    iter.filter(|r| {
        matches!(
            r,
            Ok(_)
                | Err(PassportParseError::InvalidValue(_))
                | Err(PassportParseError::InvalidUnit(_))
        )
    })
    .count()
}

fn part2<I>(iter: I) -> usize
where I: Iterator<Item = Result<Passport, PassportParseError>> {
    dbg!(iter.filter(Result::is_ok).collect::<Vec<_>>())
        .iter()
        .count()

    // iter.filter(Result::is_ok).count()
}

#[derive(Debug)]
enum PassportParseError {
    InvalidKey,
    InvalidValue(String),
    InvalidUnit(String),
    MissingKey,
}

#[derive(Debug)]
struct ParseEyeColorError;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum EyeColor {
    Ambar,
    Blue,
    Brown,
    Gray,
    Green,
    Hazel,
    Other,
}

impl Default for EyeColor {
    fn default() -> Self {
        Self::Other
    }
}

impl FromStr for EyeColor {
    type Err = ParseEyeColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "amb" => Self::Ambar,
            "blu" => Self::Blue,
            "brn" => Self::Brown,
            "gry" => Self::Gray,
            "grn" => Self::Green,
            "hzl" => Self::Hazel,
            "oth" => Self::Other,
            _ => return Err(ParseEyeColorError),
        })
    }
}

#[derive(Debug, Default, PartialEq, PartialOrd)]
struct Passport {
    birth_year: usize,
    issue_year: usize,
    expiration_year: usize,
    /// height in centimeters
    height: f64,
    hair_color: String,
    eye_color: EyeColor,
    passport_id: usize,
    country_id: Option<usize>,
}

impl FromStr for Passport {
    type Err = PassportParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut passport = Passport::default();
        let map: HashMap<_, _> = s
            .split_whitespace()
            .filter_map(|a| a.split_once(':'))
            .collect();

        if !REQUIRED_KEYS
            .iter()
            .all(|req_key| map.contains_key(req_key))
        {
            return Err(PassportParseError::MissingKey);
        }

        for (key, value) in map {
            match key {
                "byr" => {
                    let byr = value.parse().map_err(|_err| {
                        PassportParseError::InvalidValue(format!("{}:{}", key, value))
                    })?;
                    if !(1920..=2002).contains(&byr) {
                        return Err(PassportParseError::InvalidValue(format!(
                            "{}:{}",
                            key, value
                        )));
                    }
                    passport.birth_year = byr
                },
                "iyr" => {
                    let iyr = value.parse().map_err(|_err| {
                        PassportParseError::InvalidValue(format!("{}:{}", key, value))
                    })?;
                    if !(2010..=2020).contains(&iyr) {
                        return Err(PassportParseError::InvalidValue(format!(
                            "{}:{}",
                            key, value
                        )));
                    }
                    passport.issue_year = iyr
                },
                "eyr" => {
                    let eyr = value.parse().map_err(|_err| {
                        PassportParseError::InvalidValue(format!("{}:{}", key, value))
                    })?;
                    if !(2020..=2030).contains(&eyr) {
                        return Err(PassportParseError::InvalidValue(format!(
                            "{}:{}",
                            key, value
                        )));
                    }
                    passport.expiration_year = eyr
                },
                "hgt" => {
                    if value.ends_with("in") {
                        if let Some((num, _)) = value.split_once("in") {
                            let inches: u32 = num.parse().map_err(|_err| {
                                PassportParseError::InvalidValue(format!("{}:{}", key, value))
                            })?;
                            if !(59..=76).contains(&inches) {
                                return Err(PassportParseError::InvalidValue(format!(
                                    "{}:{}",
                                    key, value
                                )));
                            }
                            passport.height = inches as f64 / 0.39370f64;
                        }
                    } else if value.ends_with("cm") {
                        if let Some((num, _)) = value.split_once("cm") {
                            let cm: u32 = num.parse().map_err(|_err| {
                                PassportParseError::InvalidValue(format!("{}:{}", key, value))
                            })?;
                            if !(150..=193).contains(&cm) {
                                return Err(PassportParseError::InvalidValue(format!(
                                    "{}:{}",
                                    key, value
                                )));
                            }
                            passport.height = cm as f64
                        }
                    } else {
                        // passport.height = value.parse().map_err(|_err| {
                        //     PassportParseError::InvalidValue(format!("{}:{}", key, value))
                        // })?;
                        return Err(PassportParseError::InvalidUnit(format!(
                            "{}:{}",
                            key, value
                        )));
                    }
                },
                "hcl" => {
                    if !(value.is_ascii()
                        && value.starts_with('#')
                        && value.len() == 7
                        && value.chars().skip(1).all(|c| c.is_ascii_hexdigit()))
                    {
                        return Err(PassportParseError::InvalidValue(format!(
                            "{}:{}",
                            key, value
                        )));
                    }
                    passport.hair_color = value.to_string();
                },
                "ecl" => {
                    passport.eye_color = value.parse().map_err(|_err| {
                        PassportParseError::InvalidValue(format!("{}:{}", key, value))
                    })?;
                },
                "pid" => {
                    if !(value.is_ascii() && value.len() == 9) {
                        return Err(PassportParseError::InvalidValue(format!(
                            "{}:{}",
                            key, value
                        )));
                    }
                    passport.passport_id = value.parse().map_err(|_err| {
                        PassportParseError::InvalidValue(format!("{}:{}", key, value))
                    })?;
                },
                "cid" => {
                    passport.country_id = Some(value.parse().map_err(|_err| {
                        PassportParseError::InvalidValue(format!("{}:{}", key, value))
                    })?);
                },
                _ => return Err(PassportParseError::InvalidKey),
            }
        }

        Ok(passport)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    const EXAMPLE2: &str = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007

pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

    #[test]
    fn part1_works() {
        let iter = EXAMPLE1.split("\n\n").map(|s| s.parse::<Passport>());

        let num_valid = part1(iter);
        assert_eq!(num_valid, 2);
    }

    #[test]
    fn part2_works() {
        let iter = EXAMPLE2.split("\n\n").map(|s| s.parse::<Passport>());

        let num_valid = part2(iter);
        assert_eq!(num_valid, 4);
    }
}
