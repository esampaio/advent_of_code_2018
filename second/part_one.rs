use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let buffer = BufReader::new(file);

    let mut doubles: i32 = 0;
    let mut triples: i32 = 0;

    for line in buffer.lines() {
        let mut hash = HashMap::new();
        let mut has_double = false;
        let mut has_triple = false;
        let letters = line.as_ref().unwrap().split("").collect::<Vec<_>>();

        for l in letters {
            if l == "" {
                continue;
            }
            hash.entry(l).or_insert(vec![]).push(l)
        }

        for (_k, v) in &hash {
            if v.len() == 2 {
                has_double = true;
            }
            if v.len() == 3 {
                has_triple = true;
            }
            if has_double && has_triple {
                break;
            }
        }

        if has_double {
            doubles += 1;
        }

        if has_triple {
            triples += 1;
        }
    }

    println!("Result frequency: {}", doubles * triples);

    Ok(())
}
