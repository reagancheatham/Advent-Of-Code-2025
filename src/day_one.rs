use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Result;

pub fn run() -> Result<()> {
    let mut dial: i32 = 50;
    let mut zero_count = 0;
    let file_path = "input/day_one_input.txt";
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    println!("Dial start: {}", dial);

    for line in reader.lines() {
        let line = line?;
        let mut chars = line.chars();
        chars.next();
        let move_string = chars.as_str();

        let mut negate = false;

        if line.starts_with('L') {
            negate = true;
        }

        let mut movement = move_string.parse::<i32>()?;
        movement *= if negate { -1 } else { 1 };

        let total = dial + movement;

        let mut zero_touches = total.abs() / 100;

        if dial > 0 && total < 0 {
            zero_touches += 1;
        }

        dial = (dial + movement).rem_euclid(100);

        zero_count += zero_touches;

        if zero_touches == 0 && dial == 0 {
            zero_count += 1;
        }
    }

    println!("Answer: {}", zero_count);

    Ok(())
}
