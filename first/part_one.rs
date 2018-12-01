use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let buffer = BufReader::new(file);

    let mut result: i32 = 0;

    for line in buffer.lines() {
        result = result + line.unwrap().parse::<i32>().unwrap();
    }

    println!("Result frequency: {}", result);

    Ok(())
}
