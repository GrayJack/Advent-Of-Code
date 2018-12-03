use std::{
    collections::HashSet,
    error::Error,
    io::{ Read, stdin }
};

fn first_part(input: &str) -> Result<(), Box<Error>> {
    let mut total = 0;

    for i in input.lines() {
        let num: i32 = i.parse()?;

        total += num;
    }

    println!("{}", total);

    Ok(())
}

fn second_part(input: &str) -> Result<(), Box<Error>> {
    let mut total = 0;

    let mut seen = HashSet::new();
    seen.insert(0);

    loop {
        for i in input.lines() {
            let num: i32 = i.parse()?;

            total += num;

            if seen.contains(&total) {
                println!("{}", total);
                return Ok(());
            }

            seen.insert(total);
        }
    }
}

fn main() -> Result<(), Box<Error>> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    first_part(&input)?;
    second_part(&input)?;

    Ok(())
}
