use std::{
    error::Error,
    io::{ Read, stdin }
};

fn first_part(input: &str) -> Result<(), Box<Error>> {
    let (mut twice, mut thrice) = (0, 0);

    for line in input.lines() {
        let mut freqs = [0u8; 256];

        for b in line.as_bytes().iter().map(|&x| x as usize) {
            freqs[b] += 1;
        }

        if freqs.iter().any(|&x| x == 2) {
            twice += 1;
        }
        if freqs.iter().any(|&x| x == 3) {
            thrice += 1;
        }
    }
    println!("{}", twice * thrice);

    Ok(())
}

fn second_part(input: &str) -> Result<(), Box<Error>> {
    let ids: Vec<&str> = input.lines().collect();

    for i in ids.iter() {
        for j in ids.iter() {
            if let Some(common) = find_common_letters(&i, &j) {
                println!("{}", common);
                return Ok(());
            }
        }
    }

    Err(From::from("Didn't found the boxes"))
}

fn find_common_letters(id1: &str, id2: &str) -> Option<String> {
    if id1 == id2 || id1.len() != id2.len() {
        return None;
    }

    let mut found_once = false;
    for (c1, c2) in id1.chars().zip(id2.chars()) {
        if c1 != c2 {
            if found_once {
                return None;
            }

            found_once = true;
        }
    }

    Some(
        id1.chars().zip(id2.chars())
            .filter(|&(c1, c2)| c1 == c2)
            .map(|(c, _)| c)
            .collect()
    )
}

fn main() -> Result<(), Box<Error>> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    first_part(&input)?;
    second_part(&input)?;

    Ok(())
}
