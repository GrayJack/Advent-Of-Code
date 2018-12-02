use std::{
    error::Error,
    io::{ Read, stdin }
};

fn main() -> Result<(), Box<Error>> {
    let mut total = 0;

    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    for i in input.lines() {
        let num: i32 = i.parse()?;

        total += num;
    }

    println!("{}", total);

    Ok(())
}
