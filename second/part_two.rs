use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let buffer = BufReader::new(file);

    let mut ids: Vec<Vec<char>> = Vec::new();
    let mut first = Vec::new();
    let mut second = Vec::new();

    for line in buffer.lines() {
        let letters = line.as_ref().unwrap().chars().collect::<Vec<char>>();

        ids.push(letters);
    }

    'outer: for letters in ids.clone() {
        let clone = ids.clone();

        for id in clone {
            let mut diff = 0;
            let length = id.len();

            for n in 0..length {
                if letters[n] == id[n] {
                    continue;
                } else {
                    diff += 1;
                }
            }

            if diff == 1 {
                first = id;
                second = letters;
                break 'outer;
            }
        }
    }


    println!("Resulting strings: {} and {}", first.iter().cloned().collect::<String>(), second.iter().cloned().collect::<String>());

    Ok(())
}
