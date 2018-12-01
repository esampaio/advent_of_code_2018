use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    let mut frequency: i32 = 0;
    let mut vec = Vec::new();
    let mut iterator;
    let mut done = false;

    while !done {
        let file = File::open("input.txt")?;
        let buffer = BufReader::new(file);

        for line in buffer.lines() {
            frequency = frequency + line.unwrap().parse::<i32>().unwrap();
            iterator = vec.clone().into_iter();
            if iterator.any(|x| x == frequency) {
                println!("Found first frequency reached twice {}", frequency);
                done = true;
                break;
            } else {
                vec.push(frequency);
            }
        }
    }

    Ok(())
}
