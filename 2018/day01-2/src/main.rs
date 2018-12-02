use std::{
    collections::HashSet,
    error::Error,
    io::{ Read, stdin }
};

fn main() -> Result<(), Box<Error>> {
    let mut total = 0;

    let mut seen = HashSet::new();
    seen.insert(0);

    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

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
